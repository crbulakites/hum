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

                    add_note_to_track(
                        note_position,
                        note_duration,
                        note_frequency,
                        volume,
                        voice,
                        &mut track,
                    );

                    println!(
                        "Added note {} ({} Hz) at {} seconds.",
                        command, note_frequency, note_position
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
    position: f32,        // The start position of the note in the track in seconds
    duration: f32,        // The duration of the note to add in seconds
    frequency: &f32,      // The frequency of the note
    volume: f32,          // The volume/amplitude of the note
    voice: &str,          // The "instrument" or "sound" of the note
    track: &mut Vec<f32>, // The master audio track to be mutated
) {
    // Generate the appropriate waveform for the note:
    let note = match voice {
        "square" => hum_math::make_wave(&hum_voice::square, frequency, duration),
        _ => hum_math::make_wave(&hum_voice::sine, frequency, duration),
    };

    // Find the start sample for the note and the duration in number of samples:
    let sample_position = (position * (SAMPLE_RATE as f32)) as usize;
    let sample_duration = (duration * (SAMPLE_RATE as f32)) as usize;

    // Find the end sample for the note:
    let extended_position = sample_position + sample_duration;

    // Extend the master track if it isn't long enough to contain the new note:
    match track.len().checked_sub(extended_position) {
        Some(_) => (),
        None => {
            let num_samples_to_add = extended_position - track.len();
            track.extend(vec![0.0; num_samples_to_add]);
        }
    }

    // Add the waveform to the waveforms already present in the master track:
    for i in 0..sample_duration {
        track[sample_position + i] += note[i] * volume;
    }
}
