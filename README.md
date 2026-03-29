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
├─────────────────────────────────────────┤
│                   Scpi                  │  SCPI protocol wrapper
├─────────────────────────────────────────┤
│               Transport (trait)         │  send / query abstraction
│              TcpTransport               │  TCP implementation (port 5025)
└─────────────────────────────────────────┘
```

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
specan = "0.1"
```

With optional serde support:

```toml
[dependencies]
specan = { version = "0.1", features = ["serde"] }
```

## Usage

### Connecting and running assays

```rust
use specan::{
    transport::TcpTransport,
    instrument::N9010a,
    session::Session,
    runner::Runner,
    assay::{AssayConfig, AssayKind},
    assay::occupied_bandwidth::OccupiedBandwidth,
    assay::maximum_peak_power::MaximumPeakPower,
    assay::wifi::average_maximum_output_power::AverageMaximumOutputPower,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = TcpTransport::connect("192.168.0.1", 5025, 5000)?;
    let instrument = N9010a::new(transport);
    let session = Session::new(instrument);
    let mut runner = Runner::new(session);

    let config = AssayConfig {
        center_frequency_mhz: 2437.0, // Wi-Fi channel 6
        bandwidth_mhz: 20.0,
        attenuation_db: 10.0,
        reference_level_dbm: 0.0,
        capture_screen: false,
    };

    let mut assays = vec![
        AssayKind::OccupiedBandwidth(OccupiedBandwidth { xdb_down: 26 }),
        AssayKind::MaximumPeakPower(MaximumPeakPower),
        AssayKind::AverageMaximumOutputPower(AverageMaximumOutputPower),
    ];

    for result in runner.run_all(&mut assays, &config) {
        match result {
            Ok(r) => {
                println!("{}", r.name);
                for m in &r.measurements {
                    println!("  {:.3} {}", m.value, m.unit);
                }
            }
            Err(e) => eprintln!("error: {e}"),
        }
    }

    Ok(())
}
```

### Structured logging

`specan-rs` uses [`tracing`](https://docs.rs/tracing) to emit structured events at every step. Add a subscriber in your application to enable them:

```toml
[dependencies]
tracing-subscriber = "0.3"
```

```rust
fn main() {
    tracing_subscriber::fmt::init();
    // ...
}
```

<details>
<summary>Example output</summary>

```
INFO  specan::runner > assay{name="Occupied Bandwidth"}: starting
DEBUG specan::scpi   > assay{name="Occupied Bandwidth"}: send cmd=":FREQ:CENT 2437 MHz"
DEBUG specan::scpi   > assay{name="Occupied Bandwidth"}: send cmd=":SENS:OBW:PERC 99"
DEBUG specan::scpi   > assay{name="Occupied Bandwidth"}: send cmd=":INIT:IMM"
DEBUG specan::scpi   > assay{name="Occupied Bandwidth"}: query cmd=":FETC:OBW?" response="20000.0"
INFO  specan::runner > assay{name="Occupied Bandwidth"}: completed elapsed_ms=15043
```

</details>

### Serialization

Enable the `serde` feature to serialize results to JSON or any other format:

```rust
let json = serde_json::to_string(&result)?;
// {"name":"Occupied Bandwidth","measurements":[{"value":20.0,"unit":"kHz"}],"screenshot":null}
```

## Available Assays

### Shared — Wi-Fi and Bluetooth

| Assay | Description | Output |
|---|---|---|
| `OccupiedBandwidth` | OBW at 99% occupancy, configurable xdB-down threshold | kHz |
| `MaximumPeakPower` | Channel power with MAXH/POS detector | dBm |

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

## Supported Instruments

| Instrument | Transport | Port |
|---|---|---|
| Keysight N9010A EXA | TCP | 5025 |

Adding support for a new instrument requires only implementing the `SpectrumAnalyzer` trait.

## Error Handling

```rust
pub enum SpecanError {
    Connection(std::io::Error),       // network errors, including timeouts
    Parse(std::num::ParseFloatError), // failed to parse instrument response
    Instrument(String),               // instrument returned unexpected data
}
```

## Testing

Tests use a `MockTransport` that serves pre-defined SCPI responses, so no real instrument is needed:

```sh
cargo test
```

> [!NOTE]
> Some tests include `thread::sleep` calls that match real instrument sweep times. Expect ~15 s total with parallel execution.

## License

MIT