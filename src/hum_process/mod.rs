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

mod hum_math;
mod hum_voice;
use std::collections::HashMap;

static SAMPLE_RATE: u32 = 44_100;

pub fn parse_score(score_contents: String) -> Vec<f32> {
    let mut tempo: f32 = 60.0;
    let mut measure: u32 = 0;
    let mut beats: u32 = 4;
    let mut cursor: f32 = 0.0;
    let mut offset: f32 = 0.0;
    let mut voice: &str = "sine";
    let volume: f32 = 0.05; // Please be careful with your ears and speakers :)

    let mut note_frequencies: HashMap<String, f32> = hum_math::piano_key_frequencies("sharps");

    let mut track: Vec<f32> = Vec::new();

    let sentences: Vec<&str> = score_contents.split(".").collect();

    for i in 0..sentences.len() - 1 {
        let clauses: Vec<&str> = sentences[i].split(":").collect();
        let command = clauses[0];
        let value = clauses[1];

        if command == "Tempo" {
            tempo = value.parse::<f32>().unwrap();
        } else if command == "Key" {
            // Perhaps inefficient if already set to sharps; will refactor later
            note_frequencies = hum_math::piano_key_frequencies(value);
        } else if command == "Measure" {
            measure += 1;
            offset = 0.0;
            beats = value.parse::<u32>().unwrap();
            let beats_per_second = tempo / 60.0;
            let measure_duration = (beats as f32) / beats_per_second;
            if measure > 1 {
                cursor += measure_duration;
            }
        } else if command == "Voice" {
            // On a new voice track, begin writing at the beginning of the measure
            offset = 0.0;
            voice = value;
        } else if command == "#" {
            // Comment command: do nothing
        } else {
            // Any other command: presume that it's a note
            match note_frequencies.get(command) {
                Some(note_frequency) => {
                    // Calculate note duration:
                    let length_values: Vec<&str> = value.split("/").collect();
                    let length_numerator: f32 = length_values[0].parse::<f32>().unwrap();
                    let length_denominator: f32 = length_values[1].parse::<f32>().unwrap();
                    let note_length_of_measure = length_numerator / length_denominator;

                    let beats_per_second = tempo / 60.0;
                    let measure_duration = (beats as f32) / beats_per_second;

                    let note_duration = measure_duration * note_length_of_measure;

                    // Get the current note position:
                    let note_position = cursor + offset;

                    println!(
                        "Added note {} ({} Hz) at {} seconds.",
                        command, note_frequency, note_position
                    );

                    add_note_to_track(
                        note_position,
                        note_duration,
                        note_frequency,
                        volume,
                        voice,
                        &mut track,
                    );

                    // Update the offset:
                    offset += note_duration;
                }
                None => {
                    println!("ERROR: cannot make sense of note {}.", value);
                }
            };
        }
    }

    track
}

fn add_note_to_track(
    position: f32,
    duration: f32,
    frequency: &f32,
    volume: f32,
    voice: &str,
    track: &mut Vec<f32>,
) {
    let note = match voice {
        "square" => hum_math::make_wave(&hum_voice::square, frequency, duration),
        _ => hum_math::make_wave(&hum_voice::sine, frequency, duration),
    };

    let sample_position = (position * (SAMPLE_RATE as f32)) as i64;
    let sample_duration = (duration * (SAMPLE_RATE as f32)) as i64;
    let extended_position = sample_position + sample_duration;

    let difference = extended_position - track.len() as i64;

    if difference > 0 {
        // Stupid, but I'll figure out a better way later:
        track.extend((0..extended_position).map(|i| i as f32 * 0.0))
    }

    for i in 0..sample_duration as usize {
        track[sample_position as usize + i] += note[i] * volume;
    }
}
