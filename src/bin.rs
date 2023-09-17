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

extern crate clap;
extern crate hum;


fn main() {
    // Parse command line arguments.
    let matches = clap::Command::new("hum")
        .version(hum::VERSION)
        .author(hum::AUTHOR)
        .about(hum::ABOUT)
        .arg(
            clap::Arg::new("INPUT")
                .help("Sets the path of the hum notation file.")
                .required(true)
                .index(1)
        )
        .arg(
            clap::Arg::new("OUTPUT")
                .help("Sets the path of the output WAV file.")
                .required(true)
                .index(2)
        )
        .get_matches();

    let input = matches.get_one::<String>("INPUT").unwrap();
    let output = matches.get_one::<String>("OUTPUT").unwrap();

    // Read the contents of the input file.
    let score_contents = hum::hum_io::read(input)
        .expect("There was a problem reading the score file.");

    // Run the program.
    match hum::convert_to_wav(score_contents, output) {
        Ok(_) => {},
        Err(message) => eprintln!("{}", message),
    }
}
