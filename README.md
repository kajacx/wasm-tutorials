# `wasm_bridge` WASM modules tutorial

Learn how to load and run WASM modules with `wasm_bridge`.

 - The `rust-guest` folder (not yet created) contains the WASM module that will be loaded by your programs.
 - The [`rust-runtime`](/rust-runtime) folder contains you program that loads and runs the WASM module.
   - It is built to be ran both as a binary application on your system, or as a library in the browser.
   - Either way, the code in the [`program.rs`](/rust-runtime/src/program.rs) that runs the WASM module is the same in both version.
 - The [`build-and-run-sys.sh`](/build-and-run-sys.sh) script shows how to run the program "normally".
 - The [`build-and-run-web.sh`](/build-and-run-web.sh) script shows how to run the code in a browser using docker, but feel free to use any other hosting method.