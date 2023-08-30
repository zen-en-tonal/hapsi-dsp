use hapsi::prelude::Octave;
use itertools::Itertools;
use std::{
    collections::{hash_map::Iter, HashMap},
    hash::Hash,
};

use crate::{frequency::freq, spectrum::Magnitude};

#[derive(Debug, Clone, Copy)]
pub struct ChromagramFactory {
    num_oct: usize,
    num_harmonic: usize,
    /// width of bin
    r: usize,
    ref_freq: f32,
}

impl Default for ChromagramFactory {
    fn default() -> Self {
        Self {
            num_oct: 2,
            num_harmonic: 2,
            r: 2,
            /// The freqency (Hz) of C1.
            ref_freq: 130.812_79,
        }
    }
}

impl ChromagramFactory {
    pub fn new(num_oct: usize, num_harmonic: usize, r: usize, ref_freq: f32) -> Self {
        Self {
            num_oct,
            num_harmonic,
            r,
            ref_freq,
        }
    }

    pub fn create<T: Octave>(&self, magnitude: &Magnitude, chroma: &T) -> Chromagram<T::PitchClass>
    where
        T::PitchClass: Eq + Hash + Clone + Copy,
    {
        let mut chroma_map = HashMap::<T::PitchClass, f32>::new();
        let tones = chroma.iter();
        for note in tones {
            let mut oct_sum = f32::default();

            for oct in 1..=self.num_oct {
                let mut harmonic_sum = f32::default();

                for harmonic in 1..=self.num_harmonic {
                    let divider = magnitude.sample_rate() / magnitude.inner().len() as f32;
                    let freq = freq(chroma, note, self.ref_freq);
                    let center_bin =
                        (freq * oct as f32 * harmonic as f32 / divider).round() as usize;
                    let min_bin = center_bin - (self.r * harmonic);
                    let max_bin = center_bin + (self.r * harmonic);
                    let mut max = f32::default();
                    for i in min_bin..max_bin {
                        if magnitude.inner()[i] > max {
                            max = magnitude.inner()[i];
                        }
                    }
                    harmonic_sum += max / harmonic as f32;
                }
                oct_sum += harmonic_sum;
            }
            chroma_map.insert(note.clone(), oct_sum);
        }
        Chromagram::new(chroma_map)
    }
}

#[derive(Debug)]
pub struct Chromagram<T>(HashMap<T, f32>);

impl<T: Copy> Chromagram<T> {
    pub fn new(hash: HashMap<T, f32>) -> Self {
        Chromagram(hash)
    }

    pub fn tones(&self) -> Vec<&T> {
        self.0.keys().collect()
    }

    pub fn top_k(&self, k: usize) -> Vec<&T> {
        self.0
            .iter()
            .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
            .map(|x| x.0)
            .take(k)
            .collect()
    }

    pub fn iter(&self) -> Iter<'_, T, f32> {
        self.0.iter()
    }
}
