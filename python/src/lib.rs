use std::sync::{Mutex, MutexGuard};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Changes the speed, pitch and volume of speech while keeping it intelligible.
#[pyclass(name = "SonicStream", module = "libsonic")]
struct PySonicStream {
    inner: Mutex<sonic::SonicStream>,
}

impl PySonicStream {
    fn stream(&self) -> MutexGuard<'_, sonic::SonicStream> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }
}

#[pymethods]
impl PySonicStream {
    /// Raises `ValueError` when the sample rate or channel count is out of range.
    #[new]
    fn new(sample_rate: u32, channels: u32) -> PyResult<Self> {
        let inner = sonic::SonicStream::new(sample_rate, channels)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self {
            inner: Mutex::new(inner),
        })
    }

    fn set_speed(&self, speed: f32) {
        self.stream().set_speed(speed);
    }

    fn set_pitch(&self, pitch: f32) {
        self.stream().set_pitch(pitch);
    }

    fn set_volume(&self, volume: f32) {
        self.stream().set_volume(volume);
    }

    /// Buffers interleaved samples in `[-1, 1]`; a trailing partial frame is dropped.
    fn write(&self, py: Python<'_>, samples: Vec<f32>) {
        py.detach(|| self.stream().write_f32(&samples));
    }

    /// Forces output from whatever is buffered.
    fn flush(&self) {
        self.stream().flush();
    }

    fn available_frames(&self) -> usize {
        self.stream().available_frames()
    }

    /// Returns up to `max_frames` frames of interleaved samples.
    fn read(&self, py: Python<'_>, max_frames: usize) -> Vec<f32> {
        py.detach(|| self.stream().read_f32(max_frames))
    }
}

#[pymodule]
fn libsonic(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySonicStream>()?;
    Ok(())
}
