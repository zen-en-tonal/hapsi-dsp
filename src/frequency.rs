use hapsi::prelude::ToneLike;

pub fn freq(tone: &impl ToneLike, ref_freq: f32) -> f32 {
    ref_freq * 2.0_f32.powf(tone.step() as f32 / tone.chroma_size() as f32)
}
