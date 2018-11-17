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

use std::f32::consts::PI;

pub fn silence(_: f32, _: &f32) -> f32 {
    0.0 // coerce every value to 0
}

pub fn sine(time: f32, frequency: &f32) -> f32 {
    (time * frequency * 2.0 * PI).sin()
}

pub fn square(time: f32, frequency: &f32) -> f32 {
    let sine_value: f32 = sine(time, frequency);

    if sine_value >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

pub fn sawtooth(time: f32, frequency: &f32) -> f32 {
    2.0 * (time * frequency - (0.5 + time * frequency).floor())
}
