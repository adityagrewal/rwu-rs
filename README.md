# rwu-rs

## Rust on Microcontrollers: A Survey

Project for the lecture **Embedded Control** (Prof. Dr. Lothar Berger, winter semester 2026/27),
Hochschule Ravensburg-Weingarten.

This repository evaluates how good Rust's support for microcontroller development actually is in
practice. The same six peripheral demos (**LED, switch, UART, ADC, DAC, PWM**) are implemented on
three boards with very different levels of Rust ecosystem maturity, and the experience of writing
each one is used as evidence for a qualitative comparison.

The full write-up, including the reasoning behind every board- and pin-level decision, is in
[`report/`](report).

## Boards

| Board (as provided)   | MCU      | Architecture used | Framework                                                                          |
| --------------------- | -------- | ----------------- | ---------------------------------------------------------------------------------- |
| ESP32-C6-DevKit-NX    | ESP32-C6 | RISC-V (RV32IMAC) | [`esp-hal`](https://github.com/esp-rs/esp-hal) 1.1 (`no_std`)                       |
| Raspberry Pi Pico 2 W | RP2350   | Arm Cortex-M33    | [`embassy-rp`](https://github.com/embassy-rs/embassy) 0.10 (`no_std`, async)        |
| LPC845-BRK Rev A      | LPC845   | Arm Cortex-M0+    | [`lpc8xx-hal`](https://github.com/lpc-rs/lpc8xx-hal) 0.10 + `lpc845-pac` 0.4        |

The RP2350 can also boot its Hazard3 RISC-V cores, but `embassy-rp`'s peripheral drivers target the
Cortex-M33 mode, so that is what is used here. See the report for the full reasoning.

## Repository structure

Each board directory is an independent Cargo project, not a member of a shared workspace, because
each needs a different build target and a different linker configuration.

```
.
├── report/                                 # full write-up: methodology, findings, code listings
│
├── esp32-c6-dev-kit-nx/                    # esp-hal, scaffolded with esp-generate
│   ├── Cargo.toml
│   ├── rust-toolchain.toml
│   ├── .cargo/config.toml
│   └── src/bin/{led,switch,uart,adc,pwm}.rs
│
├── Pico-2W/                                # embassy-rp
│   ├── Cargo.toml
│   ├── build.rs
│   ├── memory.x
│   ├── .cargo/config.toml
│   ├── cyw43-firmware/                     # not in git, see below
│   └── src/bin/{led,switch,uart,adc,pwm}.rs
│
└── lpc845-brk-rev-a/                       # lpc8xx-hal + lpc845-pac
    ├── Cargo.toml
    ├── .cargo/config.toml
    └── src/bin/{led,switch,uart,adc,dac,pwm}.rs
```

Two things worth noting about the layout:

- Only the LPC845 has a `dac.rs`, because it is the only chip here with an on-chip DAC. On the other
  two boards the DAC demo is the `pwm` binary with an RC low-pass filter on the output pin.
- The LPC845 project needs no `memory.x`. `lpc8xx-hal`'s build script generates one for the selected
  target and puts it on the linker search path.

## Demo status

|                   | LED | Switch | UART | ADC | DAC                | PWM              |
| ----------------- | --- | ------ | ---- | --- | ------------------ | ---------------- |
| ESP32-C6          | HAL | HAL    | HAL  | HAL | no DAC on this chip | HAL             |
| RP2350 (Pico 2 W) | HAL | HAL    | HAL  | HAL | no DAC on this chip | HAL             |
| LPC845-BRK Rev A  | HAL | HAL    | HAL  | HAL | raw PAC            | raw PAC          |

"HAL" means the demo goes through the platform's hardware abstraction layer. "raw PAC" means
`lpc8xx-hal` has no driver for that peripheral, so the demo is written directly against the
auto-generated register API in `lpc845-pac`.

Neither the ESP32-C6 nor the RP2350 has an on-chip DAC. This is a hardware limitation of both
families rather than a gap in their Rust ecosystems. The LPC845, the platform with the weakest crate
support of the three, is the only one with a real DAC peripheral.

## Prerequisites

Install once, for all three boards:

```sh
rustup target add riscv32imac-unknown-none-elf   # ESP32-C6
rustup target add thumbv8m.main-none-eabihf      # RP2350
rustup target add thumbv6m-none-eabi             # LPC845
rustup component add llvm-tools-preview

cargo install probe-rs-tools --locked            # ESP32-C6 and LPC845
cargo install cargo-binutils --locked            # optional: cargo size, cargo objdump
```

`picotool` is needed for the RP2350 only. A prebuilt binary is available from the
[pico-sdk-tools releases](https://github.com/raspberrypi/pico-sdk-tools/releases); building it from
source additionally needs CMake and the pico-sdk.

On Linux, `probe-rs` also needs `libudev-dev` and `pkg-config`, plus the `69-probe-rs.rules` udev
rules shipped with `probe-rs-tools` so that a probe can be opened without `sudo`.

A USB-to-serial adapter is required for the ESP32-C6 and RP2350 UART demos. The LPC845-BRK does not
need one, because its on-board debug probe already exposes a VCOM port.

## Building and flashing

Clone the repository and work from inside one board directory. There is nothing to scaffold or
generate; the projects are complete as committed.

```sh
git clone https://github.com/adityagrewal/rwu-rs
cd rwu-rs/<board-directory>
```

Each demo is a separate binary. To check that a demo builds without any hardware attached:

```sh
cargo build --release --bin led
cargo size  --release --bin led -- -A     # flash and RAM usage per section
cargo clippy --release --all-targets
```

To flash and run it, use the per-board instructions below. In every case the flashing tool is wired
up as the Cargo runner in that project's `.cargo/config.toml`, so `cargo run` is all that is needed.

### ESP32-C6-DevKit-NX

```sh
cd esp32-c6-dev-kit-nx
cargo run --release --bin led
```

The DevKit's USB-Serial/JTAG bridge is part of the ESP32-C6 itself, so a single USB cable handles
power, flashing and `defmt` log output. No external probe is needed. The runner is
`probe-rs run --chip=esp32c6`, and RTT output is streamed by the same command.

The LED, switch and PWM demos use an external LED, because the DevKit's on-board LED is an
addressable WS2812 on a strapping pin rather than a plain GPIO. See the pin table below.

### Raspberry Pi Pico 2 W

The Pico 2 W has no debug probe on the board and the RP2350 has no USB debug bridge of its own, so
the board must be in BOOTSEL mode before every flash. Hold the BOOTSEL button while plugging in the
USB cable (or while pressing RESET), then:

```sh
cd Pico-2W
cargo run --release --bin led
```

The runner is `picotool load -u -v -x -t elf`. Because there is no debug probe, there is also no RTT
channel: the `defmt` output in these demos cannot be observed unless an external SWD probe is
attached, for example a second Pico flashed with `debugprobe`. The UART demo is verifiable over its
own serial link, and the LED and switch demos are verifiable visually.

The LED demo additionally needs the CYW43439 firmware blobs, which are not redistributed here. Copy
them from the [embassy repository](https://github.com/embassy-rs/embassy/tree/main/cyw43-firmware)
into `Pico-2W/cyw43-firmware/`:

```
43439A0.bin
43439A0_clm.bin
nvram_rp2040.bin
```

They are needed because the Pico 2 W's user LED is wired to the Wi-Fi chip's own GPIO0 rather than to
an RP2350 pin, so the SPI link to that chip has to be brought up before the LED can be toggled.

### LPC845-BRK Rev A

```sh
cd lpc845-brk-rev-a
cargo run --release --bin led
```

The board carries a CMSIS-DAP compatible debug probe, so flashing runs over the same single USB cable
that powers it. The runner is `probe-rs run --chip LPC845M301JBD48`.

`lpc8xx-hal` has no `defmt` integration, so logging goes over the board's VCOM port instead. USART0
is routed to `PIO0_24`/`PIO0_25`, which the on-board probe bridges to the host as a virtual serial
port. Open it at 115200 8N1 with `picocom`, `minicom` or PuTTY.

Note that `lpc8xx-hal` does not guarantee API stability and has not seen a release since October
2022. If a demo stops compiling after a dependency update, check the crate's current API before
assuming the demo is at fault.

## Pin assignments

`ext.` marks a signal that needs external wiring. Everything else is on-board.

| Demo   | ESP32-C6-DevKit-NX                | Pico 2 W (RP2350)                    | LPC845-BRK Rev A                        |
| ------ | --------------------------------- | ------------------------------------ | --------------------------------------- |
| LED    | GPIO7 (ext. LED + resistor)       | CYW43439 GPIO0, or PIN_15 (ext.)     | PIO1_2, red segment, active low          |
| Switch | GPIO9, on-board BOOT button       | PIN_16 (ext. button, internal pull-up) | PIO0_4, User button K3                 |
| UART   | GPIO0 TX, GPIO1 RX (ext. adapter) | PIN_0 TX, PIN_1 RX (ext. adapter)    | PIO0_25 TXD, PIO0_24 RXD (on-board VCOM) |
| ADC    | GPIO2, ADC1 ch. 2 (ext. pot)      | PIN_26/27/28, ADC0-2 (ext.)          | PIO0_7, ADC ch. 0 (on-board pot RV1)     |
| DAC    | GPIO3 PWM + RC filter (1k, 10uF)  | not implemented, no DAC peripheral   | PIO0_17, DAC0, true 10-bit output        |
| PWM    | GPIO3, LEDC channel 0             | GPIO15 slice 7B, GPIO4 slice 2A      | PIO1_1, SCT0_OUT0, blue segment          |

## Key findings

- The ESP32-C6 and the RP2350 are close to parity at the crate level, and both are clearly ahead of
  the LPC845. Both cover five of the six demos through their HAL, and the sixth is missing hardware
  rather than software.
- Crate maturity and workflow quality do not go together. The Pico 2 W has the most ergonomic HAL of
  the three attached to the most awkward flashing loop, because the board has no debug probe. The
  LPC845-BRK has the weakest crates and the second-best hardware workflow.
- The LPC845's Rust story is more nuanced than "no HAL". A real community HAL exists and covers four
  of the six peripherals, but it disclaims API stability, still targets the `embedded-hal` 0.2 trait
  generation, and has no PWM or DAC driver, so those two demos drop to raw register access.
- On both ESP32-C6 and RP2350, the easiest-sounding demo (LED) turned out to be the most involved
  one, for board-specific hardware reasons rather than anything to do with Rust.

Full reasoning, per-platform assessments and the comparison table are in the report.

## References

Upstream projects this survey relies on:

- [esp-hal](https://github.com/esp-rs/esp-hal), [esp-rtos](https://crates.io/crates/esp-rtos),
  [Rust on ESP Book](https://docs.esp-rs.org/book/)
- [embassy](https://github.com/embassy-rs/embassy), [Embassy Book](https://embassy.dev/book/)
- [lpc8xx-hal](https://github.com/lpc-rs/lpc8xx-hal),
  [lpc845-pac](https://crates.io/crates/lpc845-pac),
  [lpc-pac](https://github.com/lpc-rs/lpc-pac)
- [probe-rs](https://probe.rs), [picotool](https://github.com/raspberrypi/picotool)

Full citations, including the NXP user manuals, are in the report's bibliography.

## Author

Project for the Embedded Control course, RWU. See `report/` for author and matriculation details.
