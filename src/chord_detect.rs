use std::collections::HashMap;

use hapsi::{
    chord::Chord,
    prelude::{Octave, Quality},
    twelve_tet::{Tone, Twelve},
};
use once_cell::sync::Lazy;

use crate::chromagram::Chromagram;

static ALL_CHORDS: Lazy<Vec<Chord<Tone>>> = Lazy::new(|| {
    let mut vec = Vec::<Chord<Tone>>::new();
    for tone in Twelve.iter() {
        for quality in Quality::enumerate() {
            vec.push(Chord::new(tone.clone(), quality.clone()));
        }
    }
    vec
});

static CHORD_PROFILES: Lazy<HashMap<&Chord<Tone>, [u8; 12]>> = Lazy::new(|| {
    let mut map: HashMap<&Chord<Tone>, [u8; 12]> = Default::default();
    for chord in ALL_CHORDS.iter() {
        let mut array = [0; 12];
        for class in chord.into_class().into_vec() {
            let index: usize = class.into();
            array[index] = 1;
        }
        map.insert(chord, array);
    }
    map
});

pub fn detect_chord(chroma: &Chromagram<Tone>) -> Chord<Tone> {
    let mut best_score: f32 = f32::MAX;
    let mut chord: Chord<Tone> = Chord::new("C".parse().unwrap(), Quality::Major);
    for profile in CHORD_PROFILES.iter() {
        let mut sum: f32 = 0.0;
        for item in chroma.iter() {
            let index: usize = item.0.clone().into();
            sum += (1.0 - profile.1[index] as f32) * item.1.powf(2.0);
        }
        if sum < best_score {
            chord = *profile.0.clone();
            best_score = sum;
        }
    }
    chord
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use hapsi::{prelude::chord::Chord, twelve_tet::Tone};

    use crate::chromagram::Chromagram;

    #[test]
    fn detect_chord() {
        let mut map = HashMap::<Tone, f32>::new();
        map.insert("C".parse().unwrap(), 0.0);
        map.insert("Cs".parse().unwrap(), 0.0);
        map.insert("D".parse().unwrap(), 0.0);
        map.insert("Ds".parse().unwrap(), 0.0);
        map.insert("E".parse().unwrap(), 1.0);
        map.insert("F".parse().unwrap(), 0.0);
        map.insert("Fs".parse().unwrap(), 0.0);
        map.insert("G".parse().unwrap(), 1.0);
        map.insert("Gs".parse().unwrap(), 0.0);
        map.insert("A".parse().unwrap(), 0.0);
        map.insert("As".parse().unwrap(), 0.0);
        map.insert("B".parse().unwrap(), 1.0);
        let chroma = Chromagram::new(map);
        let chord = super::detect_chord(&chroma);
        assert_eq!(
            chord,
            Chord::new("E".parse().unwrap(), hapsi::prelude::Quality::Minor)
        );
    }
}
