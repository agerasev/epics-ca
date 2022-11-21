# epics-ca

Rust bindings for [EPICS](https://epics-controls.org/) Channel Access protocol.

## Requrements

This crate requires built [`epics-base`](https://github.com/epics-base/epics-base) or at least `ca` library.

During build you need to provide a path to a static library (`libca.a` or `ca.lib`). This could be done either by:

+ setting `EPICS_BASE` env variable that contains path to `epics-base` root, or
+ adding to `RUSTFLAGS` a path where static library file is stored (e.g. `-L /opt/epics-base/lib/linux-x86_64`).

At run time the crate also needs a dynamic library (`libca.so` or `ca.dll`).
You need to provide path to its location (e.g. via `LD_LIBRARY_PATH`) or put it where it could be found automatically (e.g. along with executable).

## Testing

To run tests you need to have dummy IOC running (located in `ioc` dir):

+ Set appropriate `EPICS_BASE` path in `configure/RELEASE`.
+ Build with `make`.
+ Go to `iocBoot/iocTest/` and run script `st.cmd` and don't stop it.

In separate shell run `cargo test`.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
