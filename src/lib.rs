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

pub mod hum_parse;
pub mod hum_process;

pub mod hum_editor;
pub mod hum_error;
pub mod hum_io;

// Global constants
pub const SAMPLE_RATE: u32 = 44_100;

// Some information about the library.
/// The current version of the Hum library.
pub const VERSION: &str = "0.7.0-dev";
/// The author of the Hum library.
pub const AUTHOR: &str = "Connor Bulakites <connor@bulakites.net>";
/// A brief description of the Hum library.
pub const ABOUT: &str = "Hum is a music notation language and synthesizer.";

fn parse_score_contents(score_contents: &str) -> Result<Vec<f32>, hum_error::HumError> {
    // Parse the score file and use the derived commands to generate the waveform.
    let score_commands = hum_parse::hum_grammar::score(score_contents)?;
    Ok(hum_process::run_commands(score_commands)?)
}

/// Converts a Hum notation string into a WAV file.
///
/// # Arguments
///
/// * `score_contents` - A string slice containing the Hum notation.
/// * `outfname` - The path where the output WAV file should be saved.
///
/// # Returns
///
/// A `Result` indicating success or containing a `HumError`.
pub fn convert_to_wav(score_contents: &str, outfname: &str) -> Result<(), hum_error::HumError> {
    // Generate the waveform and save it to a WAV file.
    let waveform = parse_score_contents(score_contents)?;
    Ok(hum_io::save(waveform, outfname)?)
}
