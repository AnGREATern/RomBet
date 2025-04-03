use rand::{Rng, rng};

type Float = f64;

pub struct Deviation(Float);

impl Deviation {
    pub fn value(&self) -> Float {
        self.0
    }

    pub fn generate(min: Float, max: Float) -> Self {
        let value = rng().random_range(min..=max);
        Self(value)
    }
}
