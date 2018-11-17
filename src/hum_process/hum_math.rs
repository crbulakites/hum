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
use std::f32::NAN;

pub fn generate_wave(signal: &Fn(f32, &f32) -> f32, frequency: &f32, duration: f32) -> Vec<f32> {
    let sample_rate = 44_100_f32; // The number of samples per second
    let num_samples = (sample_rate * duration) as usize;
    // Find all of the time values in the wave and calculate the function of time (signal):
    (0..num_samples)
        .map(|sample_index| sample_index as f32 / sample_rate)
        .map(|time_in_seconds| signal(time_in_seconds, frequency))
        .map(|signal_value| {
            // Clip all signal values outside of the allowable range:
            if signal_value > 1.0 {
                1.0
            } else if signal_value < -1.0 {
                -1.0
            } else {
                signal_value
            }
        })
        .collect()
}

// Returns eight octaves of the standard 12 note scale tuned to A 440Hz:
pub fn get_standard_note_frequencies(key: &str) -> HashMap<String, f32> {
    let note_names = if key == "sharps" {
        vec![
            "Cn", "Cs", "Dn", "Ds", "En", "Fn", "Fs", "Gn", "Gs", "An", "As", "Bn",
        ]
    } else {
        vec![
            "Cn", "Df", "Dn", "Ef", "En", "Fn", "Gf", "Gn", "Af", "An", "Bf", "Bn",
        ]
    };

    let lowest_octave = 0_u8;
    let highest_octave = 7_u8;
    let concert_pitch_name = "An";
    let concert_pitch_octave = 4_u8;
    let concert_pitch_frequency = 440_f32;

    calculate_note_frequencies(
        note_names,
        lowest_octave,
        highest_octave,
        concert_pitch_name,
        concert_pitch_octave,
        concert_pitch_frequency,
    )
}

// Calculates the frequencies of notes with an arbitrary scale and tuning (concert) pitch:
fn calculate_note_frequencies(
    note_names: Vec<&str>,
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
        note_names.contains(&concert_pitch_name),
        "Hum ERR: the concert pitch is not represented in the options for note names."
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
        .unwrap();

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
    note_frequencies.insert("Rest".to_string(), NAN);

    note_frequencies
}
