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

pub fn play(filename: &str) -> Result<(), pa::Error> {
    let score_contents = hum_io::read(filename);
    let score_commands = hum_parse::hum_grammar::score(&score_contents[..]);
    match score_commands {
        Ok(value) => {
            let waveform = hum_process::run_commands(value);
            hum_io::play(waveform)
        }
        Err(error) => {
            eprintln!("Error parsing grammar: {}", error);
            let waveform = vec![0_f32];
            hum_io::play(waveform)
        }
    }
}

pub fn convert_to_wav(filename: &str, outfname: &str) {
    let score_contents = hum_io::read(filename);
    let score_commands = hum_parse::hum_grammar::score(&score_contents[..]);
    match score_commands {
        Ok(value) => {
            let waveform = hum_process::run_commands(value);
            hum_io::save(waveform, outfname);
        }
        Err(error) => {
            eprintln!("Error parsing grammar: {}", error);
            let waveform = vec![0_f32];
            hum_io::save(waveform, outfname);
        }
    }
}
