# SORACOM Orbit SDK for Rust

This is the SDK for Rust programming language for SORACOM Orbit.

Please use it to generate WASM modules for data transformation processing in SORACOM Orbit from Rust language source code.

## Prerequisites

To compile the SDK itself, as well as programs using the SDK, [Rust](https://www.rust-lang.org/) toolchain is required. Please install it in advance.

After installing Rust, please install the required WASM target:

```console
rustup target add wasm32-unknown-unknown
```

## Usage

The `src` directory in this repository contains the SDK's source code. For more specific usage, please refer to each sample in the `examples` directory.

### Notice

Please use the latest **tagged** release that supports the features currently available on the Soracom platform. The `main` branch may contain unreleased features and could break your code.

## Examples

The `examples` directory in this repository includes the following samples:

- [`lte-m-button`](./examples/lte-m-button/)

  A sample that adds auxiliary information to the data sent from the SORACOM LTE-M Button series ([SORACOM LTE-M Button Plus](https://soracom.jp/store/5207/) and [SORACOM LTE-M Button for Enterprise](https://soracom.jp/store/5206/)) and transmits it.

- [`sensit`](./examples/sensit/)

  A sample for working with data sent from [https://support.sigfox.com/docs/sens'it-v2-user-guide](https://soracom.jp/store/5235/).

## License

This SDK is released under the MIT License. For details, please see the [LICENSE](./LICENSE) file.
