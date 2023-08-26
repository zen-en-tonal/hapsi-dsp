use hapsi::prelude::{ChromaLike, ToneLike};
use itertools::Itertools;
use std::collections::HashMap;

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

    pub fn create<T, Item>(&self, magnitude: &Magnitude, chroma: &T) -> Chromagram<Item>
    where
        T: ChromaLike<Tone = Item>,
        Item: Eq + std::hash::Hash + hapsi::core::ToneLike,
    {
        let mut chroma_map = HashMap::<Item, f32>::new();
        let tones = chroma.tones_with_start(&chroma.tone(0));
        for tone in tones {
            let oct_sum = self.oct_sum(magnitude, &tone);
            chroma_map.insert(tone, oct_sum);
        }
        Chromagram(chroma_map)
    }

    fn oct_sum<Item>(&self, magnitude: &Magnitude, tone: &Item) -> f32
    where
        Item: PartialEq + std::hash::Hash + hapsi::core::ToneLike,
    {
        let mut oct_sum = f32::default();
        for oct in 1..=self.num_oct {
            let harmonic_sum = self.harm_sum(magnitude, tone, oct);
            oct_sum += harmonic_sum;
        }
        oct_sum
    }

    fn harm_sum<Item>(&self, magnitude: &Magnitude, tone: &Item, oct: usize) -> f32
    where
        Item: PartialEq + std::hash::Hash + hapsi::core::ToneLike,
    {
        let mut harmonic_sum = f32::default();
        for harmonic in 1..=self.num_harmonic {
            let max = self.harmonic_bin(magnitude, tone, oct, harmonic);
            harmonic_sum += max / harmonic as f32;
        }
        harmonic_sum
    }

    fn harmonic_bin<Item>(
        &self,
        magnitude: &Magnitude,
        tone: &Item,
        oct: usize,
        harmonic: usize,
    ) -> f32
    where
        Item: PartialEq + std::hash::Hash + hapsi::core::ToneLike,
    {
        let (min_bin, max_bin) = self.bin_range(
            magnitude.sample_rate(),
            magnitude.inner().len(),
            tone,
            oct,
            harmonic,
        );
        let mut max = f32::default();
        for i in min_bin..max_bin {
            if magnitude.inner()[i] > max {
                max = magnitude.inner()[i];
            }
        }
        max
    }

    fn bin_range<Item: ToneLike>(
        &self,
        sample_rate: f32,
        len: usize,
        tone: &Item,
        oct: usize,
        harmonic: usize,
    ) -> (usize, usize) {
        let freq_par_sample = sample_rate / len as f32;
        let freq = freq(tone, self.ref_freq) * oct as f32 * harmonic as f32;
        let center_bin = (freq / freq_par_sample).round() as usize;
        let min_bin = center_bin - (self.r * harmonic);
        let max_bin = center_bin + (self.r * harmonic);
        (min_bin, max_bin)
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
}
