# libsonic

Python bindings for [libsonic](https://github.com/ZirekHQ/libsonic-rs), a safe Rust port of Sonic.

```sh
pip install libsonic
```

```python
from libsonic import SonicStream

stream = SonicStream(22050, 1)
stream.set_speed(1.5)
stream.write(samples)  # floats in [-1, 1], interleaved by channel
stream.flush()
out = stream.read(stream.available_frames())
```

`write` drops a trailing partial frame. `SonicStream(rate, channels)` raises `ValueError` for out-of-range dimensions.

Wheels are abi3 and cover CPython 3.11 and later.
