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


extern crate portaudio as pa;

mod hum_io;
mod hum_parse;
mod hum_process;


pub const VERSION: &str = "0.5.0";
pub const AUTHOR: &str = "Connor Bulakites <connor@bulakites.net>";
pub const ABOUT: &str = "Hum is a music notation language and synthesizer.";


pub fn play(filename: &str) -> Result<(), pa::Error> {
    // Read the specified score file and parse it using the included grammar.
    let score_contents = hum_io::read(filename);
    let parse_result = hum_parse::hum_grammar::score(&score_contents[..]);

    match parse_result {
        Ok(score_commands) => {
            // Convert the list of score commands to a waveform and play it.
            let waveform = hum_process::run_commands(score_commands);
            hum_io::play(waveform)
        }
        Err(error) => {
            // In case of a parsing error, print it and then return Ok.
            eprintln!("Error parsing grammar: {}", error);
            Ok(())
        }
    }
}


pub fn convert_to_wav(filename: &str, outfname: &str) {
    // Read the specified score file and parse it using the included grammar.
    let score_contents = hum_io::read(filename);
    let parse_result = hum_parse::hum_grammar::score(&score_contents[..]);

    match parse_result {
        Ok(score_commands) => {
            // Convert the list of score commands to a waveform and save it.
            let waveform = hum_process::run_commands(score_commands);
            hum_io::save(waveform, outfname);
        }
        Err(error) => {
            // In case of a parsing error, print it and then return nothing.
            eprintln!("Error parsing grammar: {}", error);
        }
    }
}

