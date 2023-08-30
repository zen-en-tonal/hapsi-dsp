use hapsi::prelude::Octave;

pub fn freq<O: Octave>(octave: &O, tone: &O::PitchClass, ref_freq: f32) -> f32 {
    let den = octave.len();
    let num = octave.get_number(tone).unwrap();
    ref_freq * 2.0_f32.powf(num as f32 / den as f32)
}
