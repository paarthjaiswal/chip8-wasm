<!-- PROJECT_OVERVIEW_START -->
A CHIP-8 emulator implemented in Rust and compiled to WebAssembly, with a browser interface written in plain JavaScript. It executes CHIP-8 ROMs and renders their 64×32 display in an HTML canvas, while translating keyboard input into the CHIP-8 keypad. The Rust core is exposed to JavaScript with `wasm-bindgen`, keeping emulation separate from browser rendering and input handling.
<!-- PROJECT_OVERVIEW_END -->

<!-- HIGHLIGHTS_START -->
- **Highlight 1:** Implements CHIP-8 CPU state, instruction execution, timers, keypad input, and sprite drawing in Rust.
- **Highlight 2:** Shares the emulator display buffer with JavaScript through WebAssembly memory, then scales its 64×32 pixels onto a 640×320 canvas.
- **Highlight 3:** Uses a lightweight browser UI with selectable bundled ROMs and no JavaScript package or bundler configuration.
<!-- HIGHLIGHTS_END -->

# CHIP-8 WASM Emulator

A browser-based CHIP-8 emulator powered by a Rust core compiled to WebAssembly.

## Tech Stack

- **Rust** for CHIP-8 emulation (`emulator/`)
- **WebAssembly** and [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen) for the Rust-to-JavaScript interface
- **JavaScript**, HTML, and CSS for browser controls, keyboard input, and canvas rendering

## Architecture / Key Features

- `emulator/src/cpu.rs` contains the CHIP-8 machine state and instruction execution, including display, timers, RAM, stack, and keypad.
- `emulator/src/lib.rs` exposes emulator operations and the display buffer to JavaScript through `wasm-bindgen`.
- `main.js` loads ROMs, maps QWERTY keys to the 16-key CHIP-8 keypad, advances the emulator, and draws the display to a canvas.
- `index.html` provides the ROM selector, reset button, canvas, and key-mapping hint.
- The `roms/` directory contains five ROM files. The browser selector offers the IBM logo, Corax+, flags, and keypad ROMs.

## Local Setup & Run

Requirements: Rust with the `wasm32-unknown-unknown` target, [`wasm-pack`](https://rustwasm.github.io/wasm-pack/), and Python 3 (or another static HTTP server).

From the repository root, build the WebAssembly package and serve the project:

```sh
rustup target add wasm32-unknown-unknown
wasm-pack build emulator --target web
python3 -m http.server 8000
```

If needed, install `wasm-pack` with `cargo install wasm-pack` before building. Open [http://localhost:8000](http://localhost:8000) in a browser. The build creates `emulator/pkg`, which is imported by `main.js`; serving over HTTP allows the browser to load the WebAssembly module and ROM files.
