# VEML6030 Driver & Embassy Application

A robust, platform-agnostic Rust driver for the **Vishay VEML6030** high-accuracy ambient light sensor, featuring a sample asynchronous application for the **nRF52840** using the **Embassy** framework.

## Project Structure

This project is organized as a Cargo Workspace:

- **`veml6030_driver/`**: A pure, `no_std` driver library.
  - Supports both **Synchronous** (`embedded-hal`) and **Asynchronous** (`embedded-hal-async`) interfaces via Cargo features.
  - Optional `defmt` logging support.
  - Comprehensive unit tests using `embedded-hal-mock`.
- **`veml6030_app/`**: A reference implementation for the nRF52840.
  - Built on the **Embassy** asynchronous runtime.
  - Demonstrates interrupt-driven light sensing and power management.

## Features

- [x] Dual-mode driver (Async/Sync).
- [x] High-precision Lux calculation with gain and integration time compensation.
- [x] Threshold-based interrupt handling.
- [x] Fully automated workflow via `invoke` (Python).
- [x] `defmt` integration for efficient embedded logging.

## Getting Started

### Prerequisites

- [Rust Toolchain](https://rustup.rs/) (Stable)
- [probe-rs](https://probe.rs/) (for flashing)
- [Python Invoke](http://www.pyinvoke.org/) (optional, for automation)

### Automation Workflow

We use a `tasks.py` script to simplify development. You can run the following commands from the root directory:

```bash
# Run unit tests for the driver (host-side)
inv test

# Run Clippy static analysis for the whole workspace
inv clippy

# Build and flash the firmware to an nRF52840 DK
inv run

# Run a full suite (test, build, clippy)
inv all
```

## Usage in your project

To use the driver in your own Embassy project, add the following to your `Cargo.toml`:

```toml
[dependencies]
veml6030_driver = { path = "path/to/veml6030_driver", features = ["async", "defmt"] }
```

## License

MIT License

