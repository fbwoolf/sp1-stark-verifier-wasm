# SP1 STARK Verifier Wasm

This project compiles the SP1 STARK verifier from [Succinct Labs](https://github.com/succinctlabs/sp1) to WebAssembly (Wasm) for verifying SP1 proofs in browsers or Node.js environments, using `sp1-sdk v4.2.1`.

Currently, the project is in WIP status and requires more work to successfully verify an SP1 STARK proof in-browser.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/)
- SP1 toolchain (`sp1up`, `cargo-prove`)
- SP1 proof and verifying key data (see "Generating Proof Data")

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/fbwoolf/sp1-stark-verifier-wasm.git
   cd sp1-stark-verifier-wasm
   ```

2. Install Node.js dependencies for the example:
   ```bash
   cd examples/js
   npm install
   cd ../..
   ```

## Generating Proof Data

To test the verifier, you need serialized `ShardProof` and `StarkVerifyingKey` data compatible with `sp1-sdk v4.2.1`. These steps were followed to produce the content in `eth-assets` for demo purposes. To create them yourself, follow these steps to generate data using the SP1 repository’s Fibonacci example project:

1. Install SP1 prerequisites: [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)

2. Create SP1 project: [Fibonacci](https://docs.succinct.xyz/docs/sp1/getting-started/quickstart)

## Building the Wasm Module

1. Build the WASM module:
   ```bash
   wasm-pack build --target web --out-dir examples/js/pkg
   ```

## Running the Example

1. Ensure `fibonacci-proof.bin` and `fibonacci-vk.bin` are in `examples/js/`.
2. Start a local web server:
   ```bash
   cd examples/js
   npm run serve
   ```
3. Open `http://localhost:8080` in your browser and check the console for the verification result.

## Project Structure

- `Cargo.toml`: Rust project configuration.
- `src/lib.rs`: Wasm library entry point.
- `vender`: Patched SP1 core crates.
- `examples/js/`: JavaScript example to test the WASM module.
  - `index.html`: Demo webpage.
  - `index.js`: JavaScript code to call the WASM module.
  - `package.json`: Node.js configuration.

## Notes

- `sp1-core-machine` and `sp1-core-executor` were patched to get past Wasm build errors. Any contributions to help solve these issues are welcome.
   - `sp1-core-machine/build.rs` → Commented-out cc::Build so it didn't attempt to compile cpp/extern.cpp
   - `sp1-core-executor/src/syscall/context.rs` → Cast a 32-bit usize += u64 mismatch
