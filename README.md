# chip-8-rust

WebAssembly [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) emulator written in Rust.

Much of this is informed by [Laurence Muller](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/) and [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM). The included ROMs are from [here](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html) and in the public domain.


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

