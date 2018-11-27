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
extern crate portaudio as pa;

use std::fs::File;
use std::i16;
use std::io::prelude::*;

static NUM_CHANNELS: u16 = 1;
static BIT_DEPTH: u16 = 16;
static SAMPLE_RATE: u32 = 44_100;
static FRAMES_PER_BUFFER: u32 = 0; // Let PortAudio decide what the best buffer size is

pub fn read(filename: &str) -> String {
    let mut score_file = File::open(filename).expect("Score file not found.");
    let mut score_contents = String::new();

    score_file
        .read_to_string(&mut score_contents)
        .expect("Something went wrong reading the score file.");

    score_contents
}

pub fn play(waveform: Vec<f32>) -> Result<(), pa::Error> {
    let pa_instance = try!(pa::PortAudio::new());

    let settings = try!(pa_instance.default_output_stream_settings(
        NUM_CHANNELS as i32,
        SAMPLE_RATE as f64,
        FRAMES_PER_BUFFER,
    ));

    let table_size = waveform.len();
    let num_seconds = table_size as f32 / SAMPLE_RATE as f32;

    // This routine will be called by the PortAudio engine when audio is needed. It may called at
    // interrupt level on some machines, so don't do anything that could mess up the system like
    // dynamic resource allocation or IO.
    let mut cursor = 0;
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        for i in 0..frames {
            buffer[i] = waveform[cursor % table_size];
            cursor += 1;
        }
        pa::Continue
    };

    let mut stream = try!(pa_instance.open_non_blocking_stream(settings, callback));

    try!(stream.start());

    pa_instance.sleep((num_seconds * 1_000_f32) as i32);

    try!(stream.stop());
    try!(stream.close());

    Ok(())
}

pub fn save(waveform: Vec<f32>, filename: &str) {
    let spec = hound::WavSpec {
        channels: NUM_CHANNELS,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: BIT_DEPTH,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec).unwrap();

    for sample in waveform {
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }

    writer.finalize().unwrap();
}
