# cutmidel in Rust

An experiment to rewrite [cutmidel] from C to Rust.

The code is only a little shorter and simpler to write, but the size grew substantially.

| Language |  Size [B] |  Speed [ms] |
|----------|----------:|------------:|
| C        |    49,928 |      0.0055 |
| C++      |    53,104 |      0.0055 |
| Rust     |   300,368 |      0.0060 |
| V        |   490,832 |      0.0075 |
| Zig      |   877,054 |      0.0060 |
| Go       | 1,983,776 |      0.0070 |

See the others in [C++], [V], [Zig] and [Go].

## Build

    cargo build --release
    ./target/release/cutmidel longtest 1 2
    cargo clippy
    cargo fmt

[cutmidel]: https://github.com/prantlf/cutmidel
[C++]: https://github.com/prantlf/cpp-cutmidel
[V]: https://github.com/prantlf/v-cutmidel
[Go]: https://github.com/prantlf/go-cutmidel
[Zig]: https://github.com/prantlf/zig-cutmidel
