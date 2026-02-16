# Relic: A hyper-minimal slstatus-like program

[![License](https://img.shields.io/github/license/zveihander/relic)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![Latest
release](https://img.shields.io/github/v/release/zveihander/relic)](https://github.com/zveihander/relic/releases)
[![Latest commit](https://img.shields.io/github/last-commit/zveihander/relic)](https://github.com/zveihander/relic/commits/master/)

**Relic** is a lightweight status monitoring program for window managers
inspired by [slstatus](http://tools.suckless.org/slstatus/) but rewritten in the Rust programming
language. It adheres to the suckless philosophy and maintains as small a
footprint as possible using as few dependencies as possible, primarily
interacting directly with `libc`.

## Table of contents

* [Installation](#installation)
* [Quick start](#quick-start)
* [Configuration](#configuration)
* [Contributing](#contributing)
* [License](#license)
* [Acknowledgments](#acknowledgments)


## Installation

### Binary installation

Binary releases can be found on the [GitHub
page](https://github.com/zveihander/relic/releases).

### Source installation

* Ensure you have the rust toolchain (cargo, rustc) installed.

``` sh
git clone git.evanalvarez.dev/relic.git
cd relic
cargo build --release
```

The compiled binary will be located in `./target/release/relic`.

## Quick start

To run Relic with the default configuration:

``` sh
./target/release/relic
```

All this does is print some pre-configured things to `stdout`. Most likely, you
will want to use this program in a window manager like `dwm` or `dwl`.

To use it with `dwm`, add the following to your `.xinitrc`:

``` sh
while true; do
    relic
done &
exec dwm
```

To use it with `dwl`, you will want to start `dwl` with the following command:

``` sh
exec relic | dwl
```

## Configuration

In keeping with suckless tradition, Relic is configured at compile-time. Below
are the steps you can take to configure Relic.

1. Open `Cargo.toml`
   - The only thing you need to touch in this file is the default features under
     [features].
   - Below the default features you will see a list of all the different
     components you can enable.
   - To enable a component, simply add it to the default features list.
2. Open `src/config.rs`.
   - You will see a large commented table giving a basic description of each
     component as well as their arguments (if any)
   - The next step in enabling a component (following adding it to the default
     features list) is adding it to the COMPONENTS const (if it isn't already
     there).
   - You will need to configure some component's arguments to tailor it to your
     system.
3. Recompile the project with `cargo install --path .`

## Known issues and limitations

* The process of enabling components both in `Cargo.toml` and `src/config.rs`
  can be somewhat laborious. A better solution is being searched for.
* The `updates` component currently only supports XBPS and it is _very_ limited,
  only supporting offline caching.

## Contributing

Contributions that improve efficiency, add modules that maintain the standard of
minimalism, or improve documentation are 100% welcome.

To contribute, simply:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/my-amazing-feature`).
3. Commit your changes (`git commit -m "this feature is SO cool"`).
3. Submit a pull request.

## License

Relic is distributed under the **GPL-3.0 License**. See the [`LICENSE`](./LICENSE)
file for more details.

## Acknowledgments

* The [suckless.org](https://suckless.org) devs and community for the
  inspiration provided by their philosophy and software.
* [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) for the
  excellent solution to cross-building.
* The Rust `libc` crate maintainers.
* The [tokio](https://tokio.rs/) crate for making async Rust usable.
* The [time](https://docs.rs/time/latest/time/) crate for providing a minimal
  datetime library.
* The [mpris](https://docs.rs/time/latest/mpris/) crate, for being a better
  alternative to playerctl.
