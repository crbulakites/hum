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

mod hum_error;
mod hum_io;
mod hum_parse;
mod hum_process;


// Some information about the library.
pub const VERSION: &str = "0.5.0";
pub const AUTHOR: &str = "Connor Bulakites <connor@bulakites.net>";
pub const ABOUT: &str = "Hum is a music notation language and synthesizer.";


fn parse_score_contents(filename: &str) -> Result<Vec<f32>, hum_error::HumError> {
    // Read the specified score file and parse it using the included grammar.
    let score_contents = hum_io::read(filename);
    let score_commands = hum_parse::hum_grammar::score(&score_contents[..])?;

    // Use the commands in the score file to generate the waveform.
    Ok(hum_process::run_commands(score_commands)?)
}


pub fn play(filename: &str) -> Result<(), hum_error::HumError> {
    // Generate the waveform and stream it to the speakers.
    let waveform = parse_score_contents(filename)?;
    Ok(hum_io::play(waveform)?)
}


pub fn convert_to_wav(filename: &str, outfname: &str) -> Result<(), hum_error::HumError> {
    // Generate the waveform and save it to a WAV file.
    let waveform = parse_score_contents(filename)?;
    Ok(hum_io::save(waveform, outfname))
}

