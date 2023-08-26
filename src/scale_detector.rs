use hapsi::{prelude::detect_scale, scale::Diatonic, twelve_tet::Tone};

use crate::chromagram::Chromagram;

pub trait ScaleDatactor {
    type Scale;
    fn detect_scale(&self) -> Self::Scale;
}

impl ScaleDatactor for Chromagram<Tone> {
    type Scale = Diatonic;

    fn detect_scale(&self) -> Self::Scale {
        let topk: Vec<Tone> = self.top_k(7).into_iter().map(|i| i.clone()).collect();
        detect_scale(topk.as_slice())
    }
}
