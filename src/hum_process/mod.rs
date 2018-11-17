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

// The number of samples of the waveform per seconds of audio:
static SAMPLE_RATE: u32 = 44_100;

// Add notes to the audio per instructions in the *.hum file:
pub fn parse_score(score_contents: String) -> Vec<f32> {
    // The following variables may or will change as we iterate through the *.hum file:
    // -------------------------------------------------------------------------------------------
    // The current number of beats per second (defaults to 1.0 for a tempo of 60.0):
    let mut beats_per_second: f32 = 1.0;
    // The current measure being operated on, indexed from 0:
    let mut measure_index: u32 = 0;
    // The number of beats in the current measure (defaults to 4):
    let mut beats_per_measure: f32 = 4.0;
    // The duration of the current measure in seconds:
    let mut measure_duration = beats_per_measure / beats_per_second;
    // The number of seconds into the track at the start of the current measure:
    let mut timestamp_at_measure_start: f32 = 0.0;
    // The number of seconds into the current measure from the start of the current measure:
    let mut timestamp_offset_in_measure: f32 = 0.0;
    // The current "instrument" or "sound" for notes being inserted into the track (default sine):
    let mut voice: &str = "sine";
    // The current key of the track ("sharps" versus "flats", defaults to sharps):
    let mut key: &str = "sharps";
    // The current note/frequency mappings (the HashMap keys vary depending on the musical key):
    let mut note_frequencies: HashMap<String, f32> = hum_math::get_standard_note_frequencies(key);
    // The audio track itself; waveforms will be added to this as the *hum file is parsed:
    let mut track: Vec<f32> = Vec::new();
    // -------------------------------------------------------------------------------------------

    // Load all of the sentences from the *.hum file into a vector:
    let sentences: Vec<&str> = score_contents.split(".").collect();

    // Carry out the commands of all the sentences in the order that they appear:
    for i in 0..sentences.len() - 1 {
        // Separate the sentence into its command clause and its value clause:
        let clauses: Vec<&str> = sentences[i].split(":").collect();
        let command = clauses[0];
        let value = clauses[1];

        match command {
            "Tempo" => {
                // Beats per second corresponds to tempo (beats per minute) divided by seconds:
                beats_per_second = value.parse::<f32>().unwrap() / 60.0;
            }
            "Key" => {
                // Possibly expensive operation, do not repeat if unnecessary:
                if value != key {
                    key = value;
                    note_frequencies = hum_math::get_standard_note_frequencies(key);
                }
            }
            "Measure" => {
                // Update beats per measure and measure duration:
                beats_per_measure = value.parse::<f32>().unwrap();
                measure_duration = beats_per_measure / beats_per_second;

                // Advance timestamp by the length of new measure (disregarding the first measure):
                if measure_index > 0 {
                    timestamp_at_measure_start += measure_duration;
                }

                // Indicate that a measure was added:
                measure_index += 1;
            }
            "Voice" => {
                // Always set offset to 0.0 at the beginning of a new voice track:
                timestamp_offset_in_measure = 0.0;
                // Save voice for later so that we know what type of waveform to generate:
                voice = value;
            }
            "#" => {
                // This indicates a comment sentence: don't do anything
            }
            // For now, let's assume that any other value is a note
            _ => {
                match note_frequencies.get(command) {
                    // If the note is recognized:
                    Some(note_frequency) => {
                        // Calculate note duration:
                        let length_values: Vec<&str> = value.split("/").collect();
                        let length_numerator: f32 = length_values[0].parse::<f32>().unwrap();
                        let length_denominator: f32 = length_values[1].parse::<f32>().unwrap();
                        let note_length_of_measure = length_numerator / length_denominator;
                        let note_duration = measure_duration * note_length_of_measure;

                        // Get the current note position in the track in seconds:
                        let note_position =
                            timestamp_at_measure_start + timestamp_offset_in_measure;

                        add_note_to_track(
                            note_position,  // Start position of the note in the track in seconds
                            note_duration,  // Duration of the note to add in seconds
                            note_frequency, // Frequency of the note
                            voice,          // "instrument" or "sound" of the note
                            &mut track,     // Master audio track to be mutated
                        );

                        // Provide a little bit of output for the user:
                        println!(
                            "Added note {} ({} Hz) at {} seconds.",
                            command, note_frequency, note_position
                        );

                        // Update the offset:
                        timestamp_offset_in_measure += note_duration;
                    }
                    // If the note isn't recognized:
                    None => {
                        println!("ERROR: cannot make sense of note {}.", command);
                    }
                };
            }
        }
    }

    track
}

fn add_note_to_track(
    position: f32,        // Start position of the note in the track in seconds
    duration: f32,        // Duration of the note to add in seconds
    frequency: &f32,      // Frequency of the note
    voice: &str,          // "instrument" or "sound" of the note
    track: &mut Vec<f32>, // Master audio track to be mutated
) {
    // Generate the appropriate waveform for the note:
    let note = match voice {
        "square" => hum_math::generate_wave(&hum_voice::square, frequency, duration),
        "sawtooth" => hum_math::generate_wave(&hum_voice::sawtooth, frequency, duration),
        _ => hum_math::generate_wave(&hum_voice::sine, frequency, duration),
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

    // Please be careful with your ears and speakers! :)
    let volume = 0.05;

    // Add the waveform to the waveforms already present in the master track:
    for i in 0..sample_duration {
        track[sample_position + i] += note[i] * volume;
    }
}
