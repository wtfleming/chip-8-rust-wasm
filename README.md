# chip-8-rust

WebAssembly [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) emulator implemented in Rust.

The included programs are from https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html and are in the public domain.


Commands to run in a web browser locally:

```
cd chip_8_wasm
npm install
npm start
```

Run tests:

```
cargo test
```


You can also run as a Rust native app, but currently no displays are implemented, so there won't be any graphics to see.

```
cargo run
```

