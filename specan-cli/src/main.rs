use std::fs;
use std::path::PathBuf;
use clap::Parser;
use chrono::Local;
use dialoguer::{Input, MultiSelect};
use specan::assay::AssayResult;
use specan::assay::{
    AssayKind,
    wifi_assay_names,
    bluetooth_assay_names,
    occupied_bandwidth::OccupiedBandwidth,
    maximum_peak_power::MaximumPeakPower,
    spurious_emissions::SpuriousEmissions,
    wifi::average_maximum_output_power::AverageMaximumOutputPower,
    wifi::power_spectral_density::PowerSpectralDensity,
    wifi::average_power_spectral_density::AveragePowerSpectralDensity,
    bluetooth::output_power::OutputPower,
    bluetooth::peak_power_spectral_density::PeakPowerSpectralDensity,
    bluetooth::channel_separation::ChannelSeparation,
    bluetooth::hop_frequency_count::HopFrequencyCount,
    bluetooth::occupancy_time::OccupancyTime,
};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    ip: String,

    #[arg(long)]
    port: u16,

    #[arg(long)]
    tech: Tech,

    #[arg(long)]
    instruments: Instruments,
}

#[derive(clap::ValueEnum, Clone)]
enum Tech {
    Wifi,
    Bluetooth,
}

#[derive(clap::ValueEnum, Clone)]
enum Instruments {
    N9010a,
    Esr,
}

fn build_assay(name: &str) -> Result<AssayKind, Box<dyn std::error::Error>> {
    match name {
        "Occupied Bandwidth" => {
            let xdb_down: u16 = Input::new()
                .with_prompt("  xdB down threshold")
                .interact()?;
            Ok(AssayKind::OccupiedBandwidth(OccupiedBandwidth { xdb_down }))
        }
        "Maximum Peak Power" => Ok(AssayKind::MaximumPeakPower(MaximumPeakPower)),
        "Average Maximum Output Power" => Ok(AssayKind::AverageMaximumOutputPower(AverageMaximumOutputPower)),
        "Power Spectral Density" => Ok(AssayKind::PowerSpectralDensity(PowerSpectralDensity)),
        "Average Power Spectral Density" => Ok(AssayKind::AveragePowerSpectralDensity(AveragePowerSpectralDensity)),
        "Output Power" => Ok(AssayKind::OutputPower(OutputPower)),
        "Peak Power Spectral Density" => Ok(AssayKind::PeakPowerSpectralDensity(PeakPowerSpectralDensity)),
        "Spurious Emissions" => {
            let input: String = Input::new()
                .with_prompt("  ranges (start:stop MHz, comma separated — ex: 30:1000,1000:3000)")
                .interact()?;
            let ranges = parse_ranges(&input)?;
            Ok(AssayKind::SpuriousEmissions(SpuriousEmissions { ranges }))
        }
        "Channel Separation" => {
            let channel_count: u32 = Input::new()
                .with_prompt("  number of channels to search")
                .interact()?;
            Ok(AssayKind::ChannelSeparation(ChannelSeparation { channel_count }))
        }
        "Hop Frequency Count" => {
            let threshold_dbm: f64 = Input::new()
                .with_prompt("  power threshold (dBm)")
                .interact()?;
            let max_markers: u32 = Input::new()
                .with_prompt("  max markers")
                .interact()?;
            Ok(AssayKind::HopFrequencyCount(HopFrequencyCount { threshold_dbm, max_markers }))
        }
        "Occupancy Time" => {
            let sweep_time_ms: u64 = Input::new()
                .with_prompt("  sweep window (ms)")
                .interact()?;
            Ok(AssayKind::OccupancyTime(OccupancyTime { sweep_time_ms }))
        }
        other => Err(format!("unknown assay: {other}").into()),
    }
}

fn save_results(results: &[AssayResult]) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let dir = PathBuf::from("results").join(&timestamp);
    fs::create_dir_all(&dir)?;

    // JSON
    let json = serde_json::to_string_pretty(results)?;
    fs::write(dir.join("results.json"), json)?;

    // images
    for result in results {
        if let Some(screenshot) = &result.screenshot {
            let filename = result.name.to_lowercase().replace(' ', "_") + ".png";
            fs::write(dir.join(filename), screenshot)?;
        }
    }

    Ok(dir)
}

fn parse_ranges(input: &str) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    input.split(',')
        .map(|r| {
            let parts: Vec<&str> = r.trim().split(':').collect();
            if parts.len() != 2 {
                return Err(format!("invalid range: {r}").into());
            }
            let start: f64 = parts[0].trim().parse()?;
            let stop: f64 = parts[1].trim().parse()?;
            Ok((start, stop))
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let names = match args.tech {
        Tech::Wifi => wifi_assay_names(),
        Tech::Bluetooth => bluetooth_assay_names(),
    };

    let selected = MultiSelect::new()
        .with_prompt("Selecione os ensaios")
        .items(names)
        .interact()?;

    let mut assays: Vec<AssayKind> = selected.iter()
        .map(|&i| build_assay(names[i]))
        .collect::<Result<Vec<_>, _>>()?;

    println!("\nConfiguração do instrumento:");

    let center_frequency_mhz: f64 = Input::new()
        .with_prompt("  frequência central (MHz)")
        .interact()?;

    let bandwidth_mhz: f64 = Input::new()
        .with_prompt("  largura de banda (MHz)")
        .interact()?;

    let attenuation_db: f64 = Input::new()
        .with_prompt("  atenuação (dB)")
        .interact()?;

    let reference_level_dbm: f64 = Input::new()
        .with_prompt("  nível de referência (dBm)")
        .interact()?;

    let capture_screen: bool = dialoguer::Confirm::new()
        .with_prompt("  capturar screenshot?")
        .interact()?;

    let config = specan::assay::AssayConfig {
        center_frequency_mhz,
        bandwidth_mhz,
        attenuation_db,
        reference_level_dbm,
        capture_screen,
    };

    println!("\nConectando em {}:{}...", args.ip, args.port);

    let transport = specan::transport::TcpTransport::connect(&args.ip, args.port, 5000)?;

    let completed = match args.instruments {
        Instruments::N9010a => execute(specan::instrument::N9010a::new(transport), &mut assays, &config),
        Instruments::Esr    => execute(specan::instrument::Esr::new(transport), &mut assays, &config),
    };

    if !completed.is_empty() {
        let dir = save_results(&completed)?;
        println!("\nResultados salvos em: {}", dir.display());
    }

    Ok(())
}

fn execute<A: specan::instrument::SpectrumAnalyzer>(
    instrument: A,
    assays: &mut [AssayKind],
    config: &specan::assay::AssayConfig,
) -> Vec<AssayResult> {
    let session = specan::session::Session::new(instrument);
    let mut runner = specan::runner::Runner::new(session);

    println!("Executando {} ensaio(s)...\n", assays.len());

    let mut completed = Vec::new();
    for result in runner.run_all(assays, config) {
        match result {
            Ok(r) => {
                println!("✓ {}", r.name);
                for m in &r.measurements {
                    println!("  {:.3} {}", m.value, m.unit);
                }
                completed.push(r);
            }
            Err(e) => println!("✗ erro: {e}"),
        }
    }
    completed
}
