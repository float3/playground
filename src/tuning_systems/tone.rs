use crate::{equal_temperament, Fraction, CN1, OCTAVE_SIZE, TWELVE_TONE_NAMES};

#[derive(Clone, Debug, PartialEq)]
pub struct Tone {
    name: String,
    fraction: Fraction,
    octave: u32,
    octave_size: u32,
    tone_index: u32,
}

impl Tone {
    pub fn new(fraction: Fraction, tone_index: u32) -> Tone {
        Tone::new_with_octave_size(fraction, OCTAVE_SIZE, tone_index)
    }

    pub fn new_with_octave_size(mut fraction: Fraction, octave_size: u32, tone_index: u32) -> Tone {
        let name = TWELVE_TONE_NAMES[(tone_index % OCTAVE_SIZE) as usize]; // check what happens for negative tone_index
        let octave: u32 = tone_index / octave_size;
        let adjusted_octave: i32 = octave as i32 - 1;
        let name: String = if adjusted_octave < 0 {
            format!("{}N{}", name, -adjusted_octave)
        } else {
            format!("{}{}", name, adjusted_octave)
        };
        if fraction.base == 0 {
            fraction.numerator += (2u32.pow(octave) - 1) * fraction.denominator;
        }
        Tone {
            name: name.to_string(),
            fraction,
            octave,
            octave_size,
            tone_index,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn octave(&self) -> u32 {
        self.octave
    }

    pub fn octave_size(&self) -> u32 {
        self.octave_size
    }

    pub fn tone_index(&self) -> u32 {
        self.tone_index
    }

    pub fn cents(&self) -> f64 {
        let reference_freq: f64 = equal_temperament(self.tone_index, self.octave_size).into();
        let comparison_freq: f64 = self.frequency();
        1200f64 * (comparison_freq / reference_freq).log2()
    }

    pub fn frequency(&self) -> f64 {
        let ratio: f64 = self.fraction.into();
        ratio * CN1
    }
}
