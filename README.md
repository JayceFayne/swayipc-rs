# swayipc-rs

A Rust library for controlling swaywm through its [IPC interface](https://github.com/swaywm/sway/blob/master/sway/sway-ipc.7.scd).
This library can be used  in either a synchronous or asynchronous manner.
The async feature can be enabled be adding the following to your Cargo.toml:

```toml
[dependencies.swayipc]
features = ["async"]
```

## Usage

Examples of how to use the library can be found [here](examples).

## Versioning

This library targets the latest stable release of [sway](https://github.com/swaywm/sway).

## Contributing

 If you find any errors in swayipc-rs or just want to add a new feature feel free to [submit a PR](https://github.com/jaycefayne/swayipc-rs/pulls).

## Credits

- [Trevor Merrifield](https://github.com/stapelberg) for his awesome work implementing the i3ipc protocol in [go](https://github.com/i3/go-i3).
- [Michael Stapelberg](https://github.com/tmerr) for his awesome work implementing the i3ipc protocol in [rust](https://github.com/tmerr/i3ipc-rs).
- And ofc [Drew DeVault](https://github.com/tmerr) and all the sway contributors for creating sway.