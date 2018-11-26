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

use std::fs::File;
use std::io::prelude::*;

mod hum_parse {
    include!(concat!(env!("OUT_DIR"), "/hum_parse.rs"));
}

fn main() {
    let matches = clap::clap_app!(hum_app =>
        (version: "0.3.0")
        (author: "Connor Bulakites <connor@bulakites.net>")
        (about: "A music notation language and synthesizer written in Rust.")
        (@arg INPUT: +required "Sets the path of the hum notation file.")
        (@arg OUTPUT: -o --output +takes_value "Optionally sets the path of an output WAV file.")
    )
    .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap_or("");

    if output == "" {
        /*
        match hum::play(input) {
            Ok(_) => {},
            e => {
                eprintln!("Audio stream failed with the following: {:?}", e);
            }
        };
        */
        let mut file = File::open(input).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");
        let parsed = hum_parse::score(&contents[..]);
        match parsed {
            Ok(v) => {
                for command in v {
                    println!("{}: {}", command.0, command.1)
                }
            }
            Err(e) => eprintln!("error parsing grammar: {}", e),
        }
    } else {
        hum::convert_to_wav(input, output);
    }
}
