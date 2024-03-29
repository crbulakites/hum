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

extern crate hound;

use std::fs::File;
use std::i16;
use std::io;
use std::io::Read;


static NUM_CHANNELS: u16 = 1;
static BIT_DEPTH: u16 = 16;
static SAMPLE_RATE: u32 = 44_100;


pub fn read(filename: &str) -> Result<String, io::Error> {
    let mut score_file = File::open(filename)?;
    let mut score_contents = String::new();
    score_file.read_to_string(&mut score_contents)?;
    Ok(score_contents)
}


pub fn save(waveform: Vec<f32>, filename: &str) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: NUM_CHANNELS,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: BIT_DEPTH,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec)?;

    for sample in waveform {
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16)?;
    }

    Ok(writer.finalize()?)
}
