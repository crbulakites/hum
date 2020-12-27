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

#[macro_use]

extern crate clap;
extern crate hum;


fn main() {
    // Parse command line arguments.
    let matches = clap::clap_app!(hum_app =>
        (version: hum::VERSION)
        (author: hum::AUTHOR)
        (about: hum::ABOUT)
        (@arg INPUT: +required "Sets the path of the hum notation file.")
        (@arg OUTPUT: -o --output +takes_value "Optionally sets the path of an output WAV file.")
    ).get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap_or("");

    // Read the contents of the input file.
    let score_contents = hum::hum_io::read(input);

    // Run the program.
    if output == "" {
        match hum::play(score_contents) {
            Ok(_) => {},
            Err(message) => eprintln!("{}", message),
        }
    } else {
        match hum::convert_to_wav(score_contents, output) {
            Ok(_) => {},
            Err(message) => eprintln!("{}", message),
        }
    }
}

