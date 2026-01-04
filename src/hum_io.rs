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

use std::fs;
use std::io;

use crate::SAMPLE_RATE;

static NUM_CHANNELS: u16 = 1;
static BIT_DEPTH: u16 = 16;

/// Reads the contents of a file into a String.
///
/// # Arguments
///
/// * `filename` - The path to the file to read.
///
/// # Returns
///
/// A `Result` containing the file contents as a `String` or an `io::Error`.
pub fn read(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

/// Saves a waveform to a WAV file.
///
/// # Arguments
///
/// * `waveform` - A vector of floating-point samples representing the audio.
/// * `filename` - The path where the WAV file should be saved.
///
/// # Returns
///
/// A `Result` indicating success or containing a `hound::Error`.
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

    writer.finalize()
}
