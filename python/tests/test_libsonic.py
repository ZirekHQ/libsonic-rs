import math

import pytest
from libsonic import SonicStream


def tone(frames, rate=22050):
    return [0.5 * math.sin(2 * math.pi * 200 * n / rate) for n in range(frames)]


def run(speed, frames=22050):
    stream = SonicStream(22050, 1)
    stream.set_speed(speed)
    stream.write(tone(frames))
    stream.flush()
    return stream.read(stream.available_frames())


def test_faster_speed_shortens_output():
    assert len(run(2.0)) < len(run(1.0)) < len(run(0.5))


def test_output_is_within_full_scale():
    assert all(abs(s) <= 1.0001 for s in run(1.5))


def test_out_of_range_dimensions_raise_value_error():
    with pytest.raises(ValueError):
        SonicStream(22050, 0)
    with pytest.raises(ValueError):
        SonicStream(1, 1)


def test_read_is_capped_at_max_frames():
    stream = SonicStream(22050, 1)
    stream.write(tone(22050))
    stream.flush()
    assert len(stream.read(100)) == 100
