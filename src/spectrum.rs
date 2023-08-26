use num_complex::{Complex32, ComplexFloat};

#[derive(Debug)]
pub struct Spectrum {
    inner: Vec<Complex32>,
    sample_rate: f32,
}

impl Spectrum {
    pub fn new(inner: Vec<Complex32>, sample_rate: f32) -> Self {
        Self { inner, sample_rate }
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn inner(&self) -> &[Complex32] {
        self.inner.as_slice()
    }

    pub fn into_magnitude(self) -> Magnitude {
        let sample_rate = self.sample_rate();
        Magnitude {
            inner: self
                .inner
                .into_iter()
                .map(|c| c.abs())
                .collect::<Vec<f32>>(),
            sample_rate,
        }
    }
}

#[derive(Debug)]
pub struct Magnitude {
    inner: Vec<f32>,
    sample_rate: f32,
}

impl Magnitude {
    pub fn new(inner: Vec<f32>, sample_rate: f32) -> Self {
        Self { inner, sample_rate }
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn inner(&self) -> &[f32] {
        self.inner.as_slice()
    }
}

impl From<Spectrum> for Magnitude {
    fn from(value: Spectrum) -> Self {
        value.into_magnitude()
    }
}
