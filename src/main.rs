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

mod hum_io;
mod hum_process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let filename = &args[1];
        let outfname = &args[2];

        println!("Transcribing score...");

        let score_contents = hum_io::read_hum(filename);
        let waveform = hum_process::parse_score(score_contents);

        hum_io::write_wav(waveform, outfname);

        println!("Finished!");

        println!("DISCLAIMER: THIS PROGRAM IS NOT YET STABLE. PLEASE TURN DOWN YOUR VOLUME BEFORE \
        PLAYING ANY OUTPUTTED WAV FILES TO PROTECT YOUR SPEAKERS AND HEARING, ESPECIALLY AFTER \
        MODIFYING THE CODE YOURSELF. USE AT YOUR OWN RISK.");
    } else {
        println!("Two arguments are required in order: 1) the path of the hum score file and \
        2) the path of the output WAV file.");
    }
}
