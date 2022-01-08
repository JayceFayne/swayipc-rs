# swayipc-async &emsp; [![Action Badge]][actions] [![Version Badge]][crates.io] [![License Badge]][license] [![Docs Badge]][docs]

[Version Badge]: https://img.shields.io/crates/v/swayipc-async.svg
[crates.io]: https://crates.io/crates/swayipc-async
[Action Badge]: https://github.com/JayceFayne/swayipc-rs/workflows/Rust/badge.svg
[actions]: https://github.com/JayceFayne/swayipc-rs/actions
[License Badge]: https://img.shields.io/crates/l/swayipc-async.svg
[license]: https://github.com/JayceFayne/swayipc-rs/blob/master/LICENSE.md
[Docs Badge]: https://docs.rs/swayipc-async/badge.svg
[docs]: https://docs.rs/swayipc-async

A Rust library for controlling swaywm through its [IPC interface](https://github.com/swaywm/sway/blob/master/sway/sway-ipc.7.scd). This library is executor agnostic and can be used with any async executor!

## Usage

Examples of how to use the library can be found [here](../examples).

## i3 compatibility

[i3](https://github.com/i3/i3) compatibility is kept if possible even though this library primarily targets sway.

## Versioning

This library targets the latest stable release of [sway](https://github.com/swaywm/sway).

## Contributing

 If you find any errors in swayipc or just want to add a new feature feel free to [submit a PR](https://github.com/jaycefayne/swayipc-rs/pulls).
