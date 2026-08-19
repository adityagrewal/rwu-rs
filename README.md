# rwu-rs

# Rust on Microcontrollers — A Survey

Project for the lecture **Embedded Control** (Prof. Dr. Lothar Berger, winter semester 2025/26),
Hochschule Ravensburg-Weingarten.

This repository evaluates how good Rust's support for microcontroller development actually is
in practice, by implementing the same six peripheral demos — **LED, switch, UART, ADC, DAC,
PWM** — on three boards with very different levels of Rust ecosystem maturity, then comparing
the experience across them.

The full write-up, including the reasoning behind every board- and pin-level decision, is in
[`report/`](./report).

## Boards

| Board (as provided) | MCU | Architecture | Framework used |
|---|---|---|---|
| ESP32-C6-DevKit-NX | ESP32-C6 | RISC-V (RV32IMAC) | [`esp-hal`](https://github.com/esp-rs/esp-hal) (`no_std`) |
| Raspberry Pi Pico 2 W | RP2350 | Arm Cortex-M33 | [`embassy-rp`](https://github.com/embassy-rs/embassy) (`no_std`, async) |
| LPC845-BRK Rev A | LPC845 | Arm Cortex-M0+ | [`lpc8xx-hal`](https://github.com/lpc-rs/lpc8xx-hal) / [`lpc845-pac`](https://crates.io/crates/lpc845-pac) |

Each board has one architecture and one framework — there was no choice to make there. What
varies is how mature and complete that framework is for the six peripherals under test; see the
report's comparison table for the full breakdown.

## Repository structure

```
.
├── report/
│   └── rust_mcu_survey_report.docx      # full write-up: methodology, findings, code listings
│
├── esp32-c6/                            # esp-hal, scaffolded with esp-generate
│   ├── Cargo.toml
│   ├── .cargo/config.toml
│   └── src/bin/
│       ├── led.rs
│       ├── switch.rs
│       ├── uart.rs
│       ├── adc.rs
│       ├── dac.rs                       # PWM + RC-filter workaround (no on-chip DAC)
│       └── pwm.rs
│
├── rp2350/                              # embassy-rp
│   ├── Cargo.toml
│   ├── .cargo/config.toml
│   ├── memory.x
│   ├── build.rs
│   └── src/bin/
│       ├── led.rs                       # via CYW43439 (Pico 2 W has no plain GPIO LED)
│       ├── switch.rs
│       ├── uart.rs
│       ├── adc.rs
│       ├── dac.rs                       # workaround, see report §4.2
│       └── pwm.rs
│
└── lpc845/                              # lpc8xx-hal + lpc845-pac
    ├── Cargo.toml
    ├── .cargo/config.toml
    ├── memory.x
    ├── Embed.toml
    └── src/bin/
        ├── led.rs
        ├── switch.rs
        ├── uart.rs
        ├── adc.rs
        ├── dac.rs                       # only board in this survey with a real on-chip DAC
        └── pwm.rs                       # no HAL path — written directly against lpc845-pac
```

Each board directory is an independent crate with its own target and toolchain, rather than a
shared Cargo workspace — see the report for why.

## Demo status

| | LED | Switch | UART | ADC | DAC | PWM |
|---|---|---|---|---|---|---|
| ESP32-C6 | ✅ | ✅ | ✅ | ✅ | ⚠️ workaround | ✅ |
| RP2350 (Pico 2 W) | ✅ | ✅ | ✅ | ✅ | ⚠️ workaround | ✅ |
| LPC845-BRK Rev A | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

Neither RISC-V board has an on-chip DAC — both DAC entries are a PWM output through an external
RC low-pass filter rather than a true DAC peripheral. The LPC845 is, ironically, the only board
here with a real one.

## Getting started

### ESP32-C6-DevKit-NX

```bash
cargo install esp-generate --locked
cargo install probe-rs-tools --locked
esp-generate --chip esp32c6 esp32c6-demos   # generates Cargo.toml, .cargo/config.toml, linker setup
cd esp32c6-demos
cargo run --release --bin led
```

`esp-generate` produces a complete, buildable project — no manual `.cargo/config.toml` needed.

### Raspberry Pi Pico 2 W

We can reference embassy-rs/examples/embassy-rp235x,for some support as a first step:

```bash
rustup target add thumbv8m.main-none-eabihf
cargo install probe-rs-tools --locked
cargo install flip-link

# add the dependencies, .cargo/config.toml, memory.x, few sources and build.rs from embassy-rs/examples/rp235x/ in this repo
cargo run --release --bin blinky_wifi.rs
```

### LPC845-BRK Rev A

Same story — no template, so `.cargo/config.toml`, `memory.x`, and `Embed.toml` (shown in
`lpc845/`) need to exist before the first build:

```bash
rustup target add thumbv6m-none-eabi
cargo install probe-rs-tools --locked
rustup component add llvm-tools-preview

cargo new --bin lpc845-demos && cd lpc845-demos
# add the dependencies, .cargo/config.toml, memory.x, and Embed.toml from lpc845/ in this repo
cargo embed --release --example led --features 845-rt
```

## Key findings

- ESP32-C6 and RP2350 both have strong, actively maintained Rust support (5 of 6 demos apiece)
  — but on both boards, the single easiest-sounding demo (LED) turned out to be the most
  interesting one, for board-specific hardware reasons rather than Rust ones.
- Neither RISC-V microcontroller in this survey has an on-chip DAC — a hardware limitation
  shared by both families, not a Rust ecosystem gap.
- The LPC845's Rust story is more nuanced than "no HAL": a real community HAL
  (`lpc8xx-hal`) exists and covers five of six peripherals, but explicitly disclaims API
  stability and has no PWM driver at all.

Full reasoning, per-platform assessments, and the comparison table are in the report.

## References

Key upstream projects this survey relies on and cites:

- [esp-hal](https://github.com/esp-rs/esp-hal) / [Rust on ESP Book](https://docs.esp-rs.org/book/)
- [embassy](https://github.com/embassy-rs/embassy) / [Embassy Book](https://embassy.dev/book/)
- [lpc8xx-hal](https://github.com/lpc-rs/lpc8xx-hal) / [lpc845-pac](https://crates.io/crates/lpc845-pac) / [lpc-pac](https://github.com/lpc-rs/lpc-pac)
- [probe-rs](https://probe.rs)
- [The Embedded Rustacean](https://blog.theembeddedrustacean.com)

Full citations, including NXP user manuals and page-level references, are in the report's
bibliography.

## Author

Project for the Embedded Control course, RWU — see `report/` for author and matriculation
details.