/*
Hum: A Music Markup Language Synthesizer
Copyright (C) 2018 Connor R. Bulakites

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::collections::HashMap;

use crate::SAMPLE_RATE;

const CONCERT_PITCH_NAME: &str = "An";
const CONCERT_PITCH_OCTAVE: u8 = 4;
const CONCERT_PITCH_FREQ: f32 = 440.0;

pub const NOTES_SHARPS: [&str; 12] = [
    "Cn", "Cs", "Dn", "Ds", "En", "Fn", "Fs", "Gn", "Gs", "An", "As", "Bn",
];
pub const NOTES_FLATS: [&str; 12] = [
    "Cn", "Df", "Dn", "Ef", "En", "Fn", "Gf", "Gn", "Af", "An", "Bf", "Bn",
];

pub const LOWEST_OCTAVE: u8 = 0;
pub const HIGHEST_OCTAVE: u8 = 7;

/// Represents the accidental style for note generation (sharps or flats).
pub enum AccidentalStyle {
    Sharps,
    Flats,
}

/// Generates a waveform for a given signal function, frequency, and duration.
///
/// # Arguments
///
/// * `signal` - A closure that takes time and frequency and returns amplitude.
/// * `frequency` - The frequency of the wave in Hz.
/// * `duration` - The duration of the wave in seconds.
///
/// # Returns
///
/// A `Vec<f32>` containing the generated samples.
pub fn generate_wave(signal: &dyn Fn(f32, f32) -> f32, frequency: f32, duration: f32) -> Vec<f32> {
    let sample_rate = SAMPLE_RATE as f32; // The number of samples per second
    let num_samples = (sample_rate * duration) as usize;
    // Find all of the time values in the wave and calculate the function of time (signal):
    (0..num_samples)
        .map(|sample_index| sample_index as f32 / sample_rate)
        .map(|time_in_seconds| signal(time_in_seconds, frequency))
        .map(|signal_value| signal_value.clamp(-1.0, 1.0))
        .collect()
}

/// Returns eight octaves of the standard 12 note scale tuned to A 440Hz.
///
/// # Arguments
///
/// * `style` - The accidental style to use ("sharps" or "flats").
///
/// # Returns
///
/// A `HashMap` mapping note names (e.g., "Cn_4") to their frequencies.
pub fn get_standard_note_frequencies(style: AccidentalStyle) -> HashMap<String, f32> {
    let note_names = match style {
        AccidentalStyle::Sharps => &NOTES_SHARPS,
        AccidentalStyle::Flats => &NOTES_FLATS,
    };

    calculate_note_frequencies(
        note_names,
        LOWEST_OCTAVE,
        HIGHEST_OCTAVE,
        CONCERT_PITCH_NAME,
        CONCERT_PITCH_OCTAVE,
        CONCERT_PITCH_FREQ,
    )
}

// Calculates the frequencies of notes with an arbitrary scale and tuning (concert) pitch:
fn calculate_note_frequencies(
    note_names: &[&str],
    lowest_octave: u8,
    highest_octave: u8,
    concert_pitch_name: &str,
    concert_pitch_octave: u8,
    concert_pitch_frequency: f32,
) -> HashMap<String, f32> {
    assert!(
        lowest_octave <= highest_octave,
        "Hum ERR: the lowest octave is indexed higher than the highest octave."
    );
    assert!(
        concert_pitch_octave >= lowest_octave && concert_pitch_octave <= highest_octave,
        "Hum ERR: the concert pitch octave is outside the range of possible octaves."
    );

    let notes_per_octave = note_names.len();
    let root = 2_f32.powf(1_f32 / notes_per_octave as f32);

    let concert_pitch_position = note_names
        .iter()
        .position(|&name| name == concert_pitch_name)
        .expect("Hum ERR: the concert pitch is not represented in the options for note names.");

    let mut note_frequencies: HashMap<String, f32> = HashMap::new();

    for octave in lowest_octave..(highest_octave + 1) {
        for (position, note) in note_names.iter().enumerate() {
            let name_offset = position as i32 - concert_pitch_position as i32;
            let octave_offset = octave as i32 - concert_pitch_octave as i32;
            let note_offset = name_offset + octave_offset * notes_per_octave as i32;
            let frequency = concert_pitch_frequency * root.powi(note_offset);
            note_frequencies.insert(format!("{}_{}", note, octave), frequency);
        }
    }

    // Insert the "rest" note with a frequency of "not a number":
    note_frequencies.insert("Rest".to_string(), f32::NAN);

    note_frequencies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concert_pitch() {
        let freqs = get_standard_note_frequencies(AccidentalStyle::Sharps);
        let a4 = freqs.get("An_4").expect("An_4 should exist");
        assert!((a4 - 440.0).abs() < 0.001, "An_4 should be 440.0 Hz");
    }

    #[test]
    fn test_octave_relationship() {
        let freqs = get_standard_note_frequencies(AccidentalStyle::Sharps);
        let a4 = freqs.get("An_4").unwrap();
        let a5 = freqs.get("An_5").unwrap();

        assert!(
            (a5 - (a4 * 2.0)).abs() < 0.001,
            "An_5 should be double An_4"
        );
    }
}
