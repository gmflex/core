# ƒlex core
Core module of [ƒlex](https://github.com/gmflex/fl) written in [Rust](https://github.com/rust-lang/rust)

ƒlex core provides usefull tools:
* mongodb driver
* file system
* geode package manager
* yaml parser
* toml parser
* bson serializer and deserializer
* crypto-functions

## Installation

1. Download the .dll file from the releases tab corresponding to your platform.
2. Place downloaded file in /garrysmod/lua/bin folder in your server.

## Building from source

1. Install rust compliler with `rustup`
2. Clone this repo and run `cargo build -r --target *your target*`(for targets look at the [rust docs](https://doc.rust-lang.org/nightly/rustc/platform-support.html))
3. Rename library in target folder to gmsv_flcore_([see](https://wiki.facepunch.com/gmod/Creating_Binary_Modules#naminglocation))
4. Place module in /garrysmod/lua/bin folder.

## Documentation
Docs avaliable at the [wiki](https://github.com/gmflex/fl-core/wiki)
