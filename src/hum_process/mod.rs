/*
Hum: A Music Markup Language Synthesizer
Copyright (C) 2018-2026 Connor R. Bulakites

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

pub mod hum_math;
mod hum_voice;

use std::collections::HashMap;

use super::hum_error::GenerateError;
use crate::SAMPLE_RATE;

const DEFAULT_VOLUME: f32 = 0.05;

struct PlaybackState {
    beats_per_second: f32,
    measure_index: i32,
    measure_greatest: i32,
    checkpoint_index: i32,
    time_signature: f32,
    beats_per_measure: f32,
    measure_duration: f32,
    timestamp_at_measure_start: f32,
    timestamp_offset_in_measure: f32,
    voice: String,
}

impl PlaybackState {
    fn new() -> Self {
        let beats_per_second = 1.0;
        let beats_per_measure = 4.0;
        PlaybackState {
            beats_per_second,
            measure_index: -1,
            measure_greatest: -1,
            checkpoint_index: -1,
            time_signature: 1.0,
            beats_per_measure,
            measure_duration: beats_per_measure / beats_per_second,
            timestamp_at_measure_start: 0.0,
            timestamp_offset_in_measure: 0.0,
            voice: "sine".to_string(),
        }
    }
}

/// Processes a list of commands to generate an audio waveform.
///
/// # Arguments
///
/// * `score_commands` - A vector of tuples representing the parsed commands from the hum file.
///
/// # Returns
///
/// A `Result` containing the generated waveform as a `Vec<f32>` or a `GenerateError`.
pub fn run_commands(score_commands: Vec<(String, String)>) -> Result<Vec<f32>, GenerateError> {
    let mut state = PlaybackState::new();
    let mut track: Vec<f32> = Vec::new();

    // Get all of the frequencies for the 12-note scale with redundant sharps and flats:
    let mut note_frequencies =
        hum_math::get_standard_note_frequencies(hum_math::AccidentalStyle::Sharps);
    let note_frequencies_flats =
        hum_math::get_standard_note_frequencies(hum_math::AccidentalStyle::Flats);
    note_frequencies.extend(note_frequencies_flats);

    for command in score_commands {
        let verb = command.0;
        let noun = command.1;

        match verb.as_ref() {
            "comment" => {}
            "tempo" => handle_tempo(&mut state, &noun)?,
            "time" => handle_time(&mut state, &noun)?,
            "checkpoint" => handle_checkpoint(&mut state),
            "voice" => state.voice = noun,
            "measure" => handle_measure(&mut state),
            "reset" => handle_reset(&mut state),
            _ => handle_note(&mut state, &mut track, &note_frequencies, &verb, &noun)?,
        }
    }

    Ok(track)
}

fn handle_tempo(state: &mut PlaybackState, noun: &str) -> Result<(), GenerateError> {
    state.beats_per_second = noun.parse::<f32>().map_err(|_| GenerateError {
        message: format!("Invalid tempo value: {}", noun),
    })? / 60.0;
    Ok(())
}

fn handle_time(state: &mut PlaybackState, noun: &str) -> Result<(), GenerateError> {
    let time_signature_parts: Vec<&str> = noun.split("/").collect();
    if time_signature_parts.len() != 2 {
        return Err(GenerateError {
            message: format!("Invalid time signature format: {}", noun),
        });
    }
    let numerator: f32 = time_signature_parts[0]
        .parse::<f32>()
        .map_err(|_| GenerateError {
            message: format!(
                "Invalid time signature numerator: {}",
                time_signature_parts[0]
            ),
        })?;
    let denominator: f32 = time_signature_parts[1]
        .parse::<f32>()
        .map_err(|_| GenerateError {
            message: format!(
                "Invalid time signature denominator: {}",
                time_signature_parts[1]
            ),
        })?;

    state.time_signature = numerator / denominator;
    state.beats_per_measure = numerator;
    state.measure_duration = state.beats_per_measure / state.beats_per_second;
    Ok(())
}

fn handle_checkpoint(state: &mut PlaybackState) {
    state.checkpoint_index = state.measure_greatest + 1;
    state.measure_index = state.measure_greatest;
}

fn handle_measure(state: &mut PlaybackState) {
    state.measure_index += 1;
    state.timestamp_at_measure_start = state.measure_duration * (state.measure_index as f32);
    state.timestamp_offset_in_measure = 0.0;

    if state.measure_index > state.measure_greatest {
        state.measure_greatest = state.measure_index;
    }
}

fn handle_reset(state: &mut PlaybackState) {
    state.measure_index = state.checkpoint_index - 1;
}

fn handle_note(
    state: &mut PlaybackState,
    track: &mut Vec<f32>,
    note_frequencies: &HashMap<String, f32>,
    verb: &str,
    noun: &str,
) -> Result<(), GenerateError> {
    match note_frequencies.get(verb) {
        Some(note_frequency) => {
            let length_parts: Vec<&str> = noun.split("/").collect();
            if length_parts.len() != 2 {
                return Err(GenerateError {
                    message: format!("Invalid note length format: {}", noun),
                });
            }
            let length_numerator: f32 =
                length_parts[0].parse::<f32>().map_err(|_| GenerateError {
                    message: format!("Invalid note length numerator: {}", length_parts[0]),
                })?;

            let mut length_denominator_str = length_parts[1].to_string();
            let pluses = length_denominator_str.matches('+').count();
            length_denominator_str = length_denominator_str.replace("+", "");

            let length_denominator: f32 =
                length_denominator_str
                    .parse::<f32>()
                    .map_err(|_| GenerateError {
                        message: format!(
                            "Invalid note length denominator: {}",
                            length_denominator_str
                        ),
                    })?;

            let note_length_of_measure =
                (length_numerator / length_denominator) / state.time_signature;
            let base_duration = state.measure_duration * note_length_of_measure;

            // Calculate duration with dots: each dot adds half the value of the previous dot
            // Formula: duration * (2 - (1/2)^n)
            let multiplier = 2.0 - (0.5f32).powi(pluses as i32);
            let note_duration = base_duration * multiplier;

            let note_position =
                state.timestamp_at_measure_start + state.timestamp_offset_in_measure;

            add_note_to_track(
                note_position,
                note_duration,
                *note_frequency,
                &state.voice,
                track,
            );

            state.timestamp_offset_in_measure += note_duration;
            Ok(())
        }
        None => Err(GenerateError {
            message: format!("There is no note named {}", verb),
        }),
    }
}

fn add_note_to_track(
    position: f32,        // Start position of the note in the track in seconds
    duration: f32,        // Duration of the note to add in seconds
    frequency: f32,       // Frequency of the note
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
    let volume = DEFAULT_VOLUME;

    // Add the waveform to the waveforms already present in the master track:
    for i in 0..sample_duration {
        track[sample_position + i] += note[i] * volume;
    }
}
