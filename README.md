# libsonic

Safe Rust port of [Sonic](https://github.com/waywardgeek/sonic), the pitch-synchronous speech speed, pitch and volume changer by Bill Cox. No `unsafe`, no C toolchain, no dependencies.

```toml
[dependencies]
libsonic = "0.1"
```

```rust
use libsonic::SonicStream;

let mut stream = SonicStream::new(22_050, 1)?;
stream.set_speed(1.5);
stream.write_f32(&samples);
stream.flush();
let frames = stream.available_frames();
let faster = stream.read_f32(frames);
```

## Scope

- Float input and output, speed, pitch and volume. Sample rate 1000..=500000 Hz, 1..=32 channels.
- Output is sample-exact with `sonic.c` (quality 0, chord pitch off) over the matrix frozen in `tests/sonic_golden.tsv`.
- Not ported: chord pitch, quality mode, the `short`/`uchar` entry points, spectrograms and the microcontroller variants.

## Development

```
cargo test
cargo bench
```

## License

Apache-2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

## Python

`pip install libsonic` installs the same stream API as a prebuilt wheel; see [`python/README.md`](python/README.md).
