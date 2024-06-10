# Embedded test issue

This is a minimal project to demonstrate an issue with `embedded-test` (v0.4).
Test are designed to run on a NUCLEO-F303ZE.

## Installation

You should ensure [rustup](https://rustup.rs/) is
installed and up-to-date. This provides the Rust toolchain, needed to compile
Rust code.

Additionally, you will need to add the relevant target to the rust toolchain.
The MCU used in this project is using the ARM Cortex-M4F (with FPU support)
architecture, therefore you will need to run:
`rustup target add thumbv7em-none-eabihf`
to install the relevant cross-compilation target.

Finally, you will need to install [probe-rs](https://probe.rs/) which will
allow you to flash the firmware to your device, and also provides
debugging and logging (using [defmt](https://github.com/knurling-rs/)). You
will then need to
[set up your debug probe](https://probe.rs/docs/getting-started/probe-setup/)
to work with probe-rs.

To install `probe-rs`, run:

```bash
cargo install probe-rs-tools@0.24.0
```

## Usage

To run the tests with the debugger, attached, from the
`minimal-firmware` subproject, run:
`cargo test`

The device will be flashed with the firmware, and the debugger will be attached.
You should see logging output in the terminal.

## Error messages

The error shown when running the above command is:

```bash
ERROR !! A panic occured: "PanicInfo { payload: Any { .. }, message: Some(assertion `left == right` failed\n  left: 255\n right: -1), location: Location { file: \"/home/tom/.cargo/registry/src/index.crates.io-6f17d22bba15001f/semihosting-0.1.10/src/sys/arm_compat/mod.rs\", line: 221, col: 9 }, can_unwind: true, force_no_backtrace: false }"
└─ embedded_test::panic @ /home/tom/.cargo/registry/src/index.crates.io-6f17d22bba15001f/embedded-test-0.4.0/src/fmt.rs:62  
Frame 0: syscall_readonly @ 0x080077ee
       /home/tom/.cargo/registry/src/index.crates.io-6f17d22bba15001f/semihosting-0.1.10/src/sys/arm_compat/syscall/arm.rs:88:9
Frame 1: sys_exit_extended @ 0x080078c0
       /home/tom/.cargo/registry/src/index.crates.io-6f17d22bba15001f/semihosting-0.1.10/src/sys/arm_compat/mod.rs:198:2
Frame 2: <unknown function @ 0x00020026> @ 0x00020026
Error: Unexpected semihosting command Unknown(UnknownCommandDetails { operation: 0, parameter: 536930668 }) cmdline_requested: true
error: test failed, to rerun pass `--test simple_test`

Caused by:
  process didn't exit successfully: `probe-rs run --chip STM32F303ZETx /home/tom/code/GSP/embedded-test-issue/minimal-firmware/target/thumbv7em-none-eabihf/debug/deps/simple_test-44000fac4dbb4997` (exit status: 1)
```
