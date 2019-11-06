### Benchmark Summary

| Crate                                                          | `no_std` compatible | Reverse dependencies | Licence                          | Notes |
| -------------------------------------------------------------- | ------------------- | -------------------- | -------------------------------- | ----- |
| [hex](https://crates.io/crates/hex)                            | Yes                 | 438                  | [MIT][MIT], [Apache-2.0][Apache] | No clippy warnings
| [rustc_hex](https://crates.io/crates/rustc-hex)                | Yes                 | 39                   | [MIT][MIT], [Apache-2.0][Apache] | Build warnings
| [faster_hex](https://crates.io/crates/faster-hex)              | No                  | 8                    | [MIT][MIT]                       | 
| [base16](https://crates.io/crates/base16)                      | Yes                 | 3                    | [CC0-1.0][CC]                    | No clippy warnings
| [binascii](https://crates.io/crates/binascii)                  | Yes                 | 2                    | [MIT][MIT]                       | No build warnings, lots of clippy ones 
| [casperlabs](https://crates.io/crates/casperlabs-contract-ffi) | Yes                 | 0                    | [Apache-2.0][Apache]             |

```
encode/hex            [32 bytes]   202.39  ns
encode/rustc_hex      [32 bytes]   237.24  ns
encode/faster_hex     [32 bytes]    36.994 ns
encode/base16         [32 bytes]    29.421 ns
encode/binascii       [32 bytes]    46.762 ns
encode/casperlabs     [32 bytes]  1801.3   ns

encode/hex          [1024 bytes]   5769.5  ns
encode/rustc_hex    [1024 bytes]   6883.2  ns
encode/faster_hex   [1024 bytes]    106.13 ns
encode/base16       [1024 bytes]    613.71 ns
encode/binascii     [1024 bytes]    808.32 ns
encode/casperlabs   [1024 bytes]  55479.0  ns

encode/hex         [65536 bytes]   359.99   us
encode/rustc_hex   [65536 bytes]   444.46   us
encode/faster_hex  [65536 bytes]     4.0708 us
encode/base16      [65536 bytes]    40.775  us
encode/binascii    [65536 bytes]    47.240  us
encode/casperlabs  [65536 bytes]  3465.7    us


decode/hex            [32 bytes]  300.35  ns
decode/rustc_hex      [32 bytes]  228.02  ns
decode/faster_hex     [32 bytes]   33.031 ns
decode/base16         [32 bytes]   34.766 ns
decode/binascii       [32 bytes]   70.986 ns
decode/casperlabs     [32 bytes]  493.80  ns

decode/hex          [1024 bytes]   7991.3  ns
decode/rustc_hex    [1024 bytes]   4736.0  ns
decode/faster_hex   [1024 bytes]    283.22 ns
decode/base16       [1024 bytes]    725.41 ns
decode/binascii     [1024 bytes]   2055.9  ns
decode/casperlabs   [1024 bytes]  16425.0  ns

decode/hex         [65536 bytes]   727.21  us
decode/rustc_hex   [65536 bytes]   548.08  us
decode/faster_hex  [65536 bytes]    15.170 us
decode/base16      [65536 bytes]    45.802 us
decode/binascii    [65536 bytes]   424.21  us
decode/casperlabs  [65536 bytes]  1173.8   us
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[MIT]: https://spdx.org/licenses/MIT.html
[Apache]: https://spdx.org/licenses/Apache-2.0.html
[CC]: https://spdx.org/licenses/CC0-1.0.html
