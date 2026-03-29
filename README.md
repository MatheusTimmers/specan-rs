# specan-rs

<div align="center">

**Rust library for automating spectrum analyzer measurements via SCPI**

Built for RF certification testing under [Anatel norm 14448](https://www.gov.br/anatel/pt-br)

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

</div>

---

`specan-rs` provides a layered, fully decoupled API for controlling spectrum analyzers. It handles instrument communication, SCPI command formatting, and implements the measurement assays required for Wi-Fi and Bluetooth certification — without coupling any of these concerns together.

Each layer depends only on the trait above it, so you can swap `TcpTransport` for a `MockTransport` in tests, or add a new instrument without touching any assay code.

- ✅ Fully trait-based — instrument and transport are swappable
- ✅ Structured logging via [`tracing`](https://docs.rs/tracing) at every layer
- ✅ Optional `serde` support for JSON serialization of results
- ✅ No real instrument needed for testing — `MockTransport` included

---

## Architecture

```
┌─────────────────────────────────────────┐
│              Your Application           │
│         (CLI, HTTP server, etc.)        │
└────────────────────┬────────────────────┘
                     │
┌────────────────────▼────────────────────┐
│                  Runner                 │  runs assays sequentially, emits traces
├─────────────────────────────────────────┤
│                  Session                │  manages the instrument connection
├─────────────────────────────────────────┤
│              SpectrumAnalyzer (trait)   │  frequency, amplitude, measurements
│                  N9010a                 │  Keysight N9010A implementation
│                   Esr                   │  Rohde & Schwarz ESR26 implementation
├─────────────────────────────────────────┤
│                   Scpi                  │  SCPI protocol wrapper
├─────────────────────────────────────────┤
│               Transport (trait)         │  send / query abstraction
│               TcpTransport              │  TCP implementation (port 5025)
└─────────────────────────────────────────┘
```

---

## Usage

This repository contains two crates:

- **`specan`**     — the library, usable directly in any Rust project
- **`specan-cli`** — a terminal client built on top of the library

### As a library

Add to your `Cargo.toml`:

```toml
[dependencies]
specan = { path = "specan" }

# with JSON serialization support
specan = { path = "specan", features = ["serde"] }
```

```rust
use specan::{
    transport::TcpTransport,
    instrument::N9010a,
    session::Session,
    runner::Runner,
    assay::{AssayConfig, AssayKind},
    assay::occupied_bandwidth::OccupiedBandwidth,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = TcpTransport::connect("192.168.0.1", 5025, 5000)?;
    let instrument = N9010a::new(transport);
    let session = Session::new(instrument);
    let mut runner = Runner::new(session);

    let config = AssayConfig {
        center_frequency_mhz: 2437.0,
        bandwidth_mhz: 20.0,
        attenuation_db: 10.0,
        reference_level_dbm: 0.0,
        capture_screen: false,
    };

    let mut assays = vec![
        AssayKind::OccupiedBandwidth(OccupiedBandwidth { xdb_down: 26 }),
    ];

    for result in runner.run_all(&mut assays, &config) {
        match result {
            Ok(r) => println!("{}: {:.3} {}", r.name, r.measurements[0].value, r.measurements[0].unit),
            Err(e) => eprintln!("error: {e}"),
        }
    }

    Ok(())
}
```

### As a CLI

```sh
cargo run --bin specan-cli -- --ip 192.168.0.1 --port 5025 --tech wifi
```

The CLI guides you through assay selection and configuration interactively, then saves results to a timestamped folder:

```
results/
└── 20260329_143022/
    ├── results.json
    └── power_spectral_density.png
```

---

## Available Assays

### Shared — Wi-Fi and Bluetooth

| Assay | Description | Output |
|---|---|---|
| `OccupiedBandwidth` | OBW at 99% occupancy, configurable xdB-down threshold | kHz |
| `MaximumPeakPower` | Channel power with MAXH/POS detector | dBm |
| `SpuriousEmissions` | Peak power across configurable frequency ranges | dBm |

### Wi-Fi

| Assay | Description | Output |
|---|---|---|
| `AverageMaximumOutputPower` | Channel power with AVER/RMS detector | dBm |
| `PowerSpectralDensity` | Spectral density visualization | screenshot |
| `AveragePowerSpectralDensity` | Peak marker, RBW = 3 kHz, AVER/RMS detector | dBm/Hz |

### Bluetooth

| Assay | Description | Output |
|---|---|---|
| `OutputPower` | Two-step: OBW at −26 dB → channel power over real bandwidth | dBm |
| `PeakPowerSpectralDensity` | Peak marker, RBW = 3 kHz, MAXH/POS detector | dBm/Hz |
| `ChannelSeparation` | Frequency separation between hop channels | MHz |
| `HopFrequencyCount` | Number of channels above power threshold | channels |
| `OccupancyTime` | Burst duration in zero-span mode | ms |

---

## Supported Instruments

| Instrument | Transport | Port |
|---|---|---|
| Keysight N9010A EXA | TCP | 5025 |
| Rohde & Schwarz ESR26 | TCP | 5025 |

Adding support for a new instrument requires only implementing the `SpectrumAnalyzer` trait.

---

## License

MIT
