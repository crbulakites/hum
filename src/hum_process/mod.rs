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

// The number of samples of the waveform per seconds of audio:
static SAMPLE_RATE: u32 = 44_100;

pub fn run_commands(score_commands: Vec<(String, String)>) -> Vec<f32> {
    // The following variables may or will change as we iterate through the *.hum file:
    // -------------------------------------------------------------------------------------------
    // The current number of beats per second (defaults to 1.0 for a tempo of 60.0):
    let mut beats_per_second: f32 = 1.0;
    // The current measure being operated on, indexed from 0:
    // This starts at -1 because it should be immediately indexed at the start of the song:
    let mut measure_index: i32 = -1;
    // The greatest measure index seen so far, indexed from 0:
    let mut measure_greatest: i32 = -1;
    // The anticipated value of measure index immediately after a checkpoint:
    let mut checkpoint_index: i32 = -1;
    // The floating point value of the current time signature (defaults to 1.0):
    let mut time_signature: f32 = 1.0;
    // The number of beats in the current measure (defaults to 4):
    let mut beats_per_measure: f32 = 4.0;
    // The duration of the current measure in seconds:
    let mut measure_duration = beats_per_measure / beats_per_second;
    // The number of seconds into the track at the start of the current measure:
    let mut timestamp_at_measure_start: f32 = 0.0;
    // The number of seconds into the current measure from the start of the current measure:
    let mut timestamp_offset_in_measure: f32 = 0.0;
    // The current "instrument" or "sound" for notes being inserted into the track (default sine):
    let mut voice: String = "sine".to_string();
    // The audio track itself; waveforms will be added to this as the *hum file is parsed:
    let mut track: Vec<f32> = Vec::new();
    // -------------------------------------------------------------------------------------------

    // Get all of the frequencies for the 12-note scale with redundant sharps and flats:
    let mut note_frequencies = hum_math::get_standard_note_frequencies("sharps");
    let note_frequencies_flats = hum_math::get_standard_note_frequencies("flats");
    note_frequencies.extend(note_frequencies_flats);

    // Carry out the commands of all the sentences in the order that they appear:
    for command in score_commands {
        // Separate the command into its verb clause and its noun clause:
        let verb = command.0;
        let noun = command.1;

        // Each possible match should correspond to a command as defined in hum_parse.rustpeg
        match verb.as_ref() {
            "comment" => {
                // Don't do anything on a comment
            }
            "tempo" => {
                // Beats per second corresponds to tempo (beats per minute) divided by seconds:
                beats_per_second = noun.parse::<f32>().unwrap() / 60.0;
            }
            "time" => {
                // Parse the numerator and denominator of the time signature:
                let time_signature_parts: Vec<&str> = noun.split("/").collect();
                let numerator: f32 = time_signature_parts[0].parse::<f32>().unwrap();
                let denominator: f32 = time_signature_parts[1].parse::<f32>().unwrap();

                // Calculate the floating point value of the time signature:
                time_signature = numerator / denominator;

                // The denominator of the time signature corresponds to the beats per measure:
                beats_per_measure = denominator;
                measure_duration = beats_per_measure / beats_per_second;
            }
            "checkpoint" => {
                // Advance checkpoint by index of the next measure (disregarding first measure):
                // Using greatest measure in case tracks within one checkpoint have different
                // numbers of measures (which is erroneous anyway):
                checkpoint_index = measure_greatest + 1;
                measure_index = measure_greatest;
            }
            "voice" => {
                voice = noun;
            }
            "measure" => {
                // Indicate that a measure was added:
                measure_index += 1;

                // Advance timestamp to the beginning of the new measure:
                timestamp_at_measure_start = measure_duration * (measure_index as f32);

                // Always set measure offset to 0.0 at the beginning of a new measure:
                timestamp_offset_in_measure = 0.0;

                // The measure index can backtrack, but the greatest measure should not:
                if measure_index > measure_greatest {
                    measure_greatest = measure_index;
                }
            }
            "reset" => {
                // Reset the measure index back to the last checkpoint:
                measure_index = checkpoint_index - 1;
            }
            // Assume that anything else is a note:
            _ => {
                match note_frequencies.get(&verb[..]) {
                    // Calculate note duration:
                    // If the note is recognized:
                    Some(note_frequency) => {
                        let length_parts: Vec<&str> = noun.split("/").collect();
                        let length_numerator: f32 = length_parts[0].parse::<f32>().unwrap();

                        // Count the number of pluses "dots" in note before parsing denominator:
                        let mut length_denominator = length_parts[1].to_string();
                        let mut pluses = 0;

                        for (i, ch) in length_parts[1].chars().enumerate() {
                            if ch == '+' {
                                pluses += 1;
                                // Remove after counting so we don't break f32 parsing:
                                length_denominator.remove(i);
                            }
                        }

                        // Shadow previous text value with evaluated floating point:
                        let length_denominator: f32 = length_denominator.parse::<f32>().unwrap();

                        // The fraction of the measure that a note takes up is its indicated
                        // length divided by the time signature:
                        let note_length_of_measure =
                            (length_numerator / length_denominator) / time_signature;

                        // Calcaulate the note duration:
                        let mut note_duration = measure_duration * note_length_of_measure;

                        // A "plus" or "dot" increases the note duration by 50 percent:
                        note_duration = note_duration + (pluses as f32) * (0.5 * note_duration);

                        // Get the current note position in the track in seconds:
                        let note_position =
                            timestamp_at_measure_start + timestamp_offset_in_measure;

                        add_note_to_track(
                            note_position,  // Start position of the note in the track in seconds
                            note_duration,  // Duration of the note to add in seconds
                            note_frequency, // Frequency of the note
                            &voice[..],     // "instrument" or "sound" of the note
                            &mut track,     // Master audio track to be mutated
                        );

                        // Update the measure offset:
                        timestamp_offset_in_measure += note_duration;
                    }
                    // If the note isn't recognized:
                    None => {
                        println!("ERROR: there is no note named {}.", verb);
                    }
                }
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
    let note = if frequency.is_nan() {
        // A frequency of NAN corresponds to a rest:
        hum_math::generate_wave(&hum_voice::silence, frequency, duration)
    } else {
        match voice {
            "square" => hum_math::generate_wave(&hum_voice::square, frequency, duration),
            "sawtooth" => hum_math::generate_wave(&hum_voice::sawtooth, frequency, duration),
            _ => hum_math::generate_wave(&hum_voice::sine, frequency, duration),
        }
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
