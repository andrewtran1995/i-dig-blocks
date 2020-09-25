# i-dig-blocks
A simple Discord bot to interact with [CubedHost APIs](https://github.com/CubedHost/prisma-api-docs).

Built using [Rust](https://www.rust-lang.org/) with [Serenity](https://github.com/serenity-rs/serenity).

## Compiling for the Raspberry Pi
This project can be targeted for a Raspberry Pi (all models should be compatible with the [Raspberry Pi OS](https://www.raspberrypi.org/downloads/)).

The release binary can be compiled with the following commands.
### Compiling with downloaded toolchains
The project can be compiled using `cargo` directly if the appropriate toolchains are downloaded (such as these [toolchains](https://gnutoolchains.com/raspberry/) for Windows).
```shell script
cargo build --release --target arm-unknown-linux-gnueabihf
```
### Compiling with Cross
Alternatively, the project can be compiled using [Cross](https://github.com/rust-embedded/cross).
```shell script
cross.exe build --release --target arm-unknown-linux-gnueabihf
```