# swayipc &emsp; [![Action Badge]][actions] [![Version Badge]][crates.io] [![License Badge]][license]

[Version Badge]: https://img.shields.io/crates/v/swayipc.svg
[crates.io]: https://crates.io/crates/swayipc
[Action Badge]: https://github.com/JayceFayne/swayipc-rs/workflows/Rust/badge.svg
[actions]: https://github.com/JayceFayne/swayipc-rs/actions
[License Badge]: https://img.shields.io/crates/l/swayipc.svg
[license]: https://github.com/JayceFayne/swayipc-rs/blob/master/LICENSE.md

A Rust library for controlling swaywm through its [IPC interface](https://github.com/swaywm/sway/blob/master/sway/sway-ipc.7.scd).
This library can be used  in either a synchronous or asynchronous manner.
The async feature can be enabled by adding the following to your Cargo.toml:

```toml
[dependencies.swayipc]
features = ["async"]
```

## Usage

Examples of how to use the library can be found [here](examples).
They can be run using cargo, e.g.`cargo run --example hovered_window`.

## Versioning

This library targets the latest stable release of [sway](https://github.com/swaywm/sway).

## Contributing

 If you find any errors in swayipc-rs or just want to add a new feature feel free to [submit a PR](https://github.com/jaycefayne/swayipc-rs/pulls).

## Credits

- [Michael Stapelberg](https://github.com/stapelberg) for his awesome work implementing the i3ipc protocol in [go](https://github.com/i3/go-i3).
- [Trevor Merrifield](https://github.com/tmerr) for his awesome work implementing the i3ipc protocol in [rust](https://github.com/tmerr/i3ipc-rs).
- And ofc [Drew DeVault](https://github.com/ddevault) and all the sway contributors for creating sway.
