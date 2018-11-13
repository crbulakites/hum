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

static SAMPLE_RATE: u32 = 44_100;

pub fn piano_key_frequencies(key: &str) -> HashMap<String, f32> {
    let note_names_sharps = [
        "An", "As", "Bn", "Cn", "Cs", "Dn", "Ds", "En", "Fn", "Fs", "Gn", "Gs",
    ];

    let note_names_flats = [
        "An", "Bf", "Bn", "Cn", "Df", "Dn", "Ef", "En", "Fn", "Gf", "Gn", "Af",
    ];

    // Source: https://en.wikipedia.org/wiki/Piano_key_frequencies
    let frequencies: Vec<f32> = (1..89)
        .map(|k| (2 as f32).powf(((k as f32) - 49.0) / 12.0) * 440.0)
        .collect();

    let mut notes: HashMap<String, f32> = HashMap::new();

    for i in 0..88 {
        let octave = (i / 12).to_string();

        let note_name = if key == "sharps" {
            note_names_sharps[i % 12]
        } else {
            note_names_flats[i % 12]
        };

        let note = format!("{}_{}", note_name, octave);

        notes.insert(note, frequencies[i]);
    }

    notes
}

pub fn make_wave(signal: &Fn(f32, &f32) -> f32, frequency: &f32, duration: f32) -> Vec<f32> {
    let sample_rate = SAMPLE_RATE as f32;
    let num_samples = (sample_rate * duration) as i64;
    (0..num_samples)
        .map(|x| x as f32 / sample_rate)
        .map(|t| signal(t, frequency))
        .map(|s| {
            if s > 1.0 {
                1.0
            } else if s < -1.0 {
                -1.0
            } else {
                s
            }
        })
        .collect()
}
