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

extern crate hum;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        //let outfname = &args[2];

        //hum::convert_to_wav(filename, outfname);

        match hum::play(filename) {
            Ok(_) => {},
            e => {
                eprintln!("Audio stream failed with the following: {:?}", e);
            }
        };
    } else {
        println!(
            "Two arguments are required in order: 1) the path of the hum score file and \
             2) the path of the output WAV file."
        );
    }
}
