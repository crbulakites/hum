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
extern crate peg;
extern crate portaudio;

use std::error;
use std::fmt;


// Custom type for general Hum processing errors
#[derive(Debug)]
pub struct GenerateError {
    pub message: String,
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for GenerateError {}


// Declaration of Peg ParseError (type specification required)
pub type ParseError = peg::error::ParseError<peg::str::LineCol>;


// Custom type for catch-all Hum errors.
#[derive(Debug)]
pub enum HumError {
    FileSaveError(hound::Error),
    PlaybackError(portaudio::Error),
    GenerateError(GenerateError),
    HumParseError(ParseError),
}
 
impl From<hound::Error> for HumError {
    fn from(err: hound::Error) -> HumError {
        HumError::FileSaveError(err)
    }
}
 
impl From<portaudio::Error> for HumError {
    fn from(err: portaudio::Error) -> HumError {
        HumError::PlaybackError(err)
    }
}
 
impl From<GenerateError> for HumError {
    fn from(err: GenerateError) -> HumError {
        HumError::GenerateError(err)
    }
}

impl From<ParseError> for HumError {
    fn from(err: ParseError) -> HumError {
        HumError::HumParseError(err)
    }
}

impl fmt::Display for HumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HumError::FileSaveError(ref err) => write!(f, "FileSaveError: {}", err),
            HumError::PlaybackError(ref err) => write!(f, "PlaybackError: {}", err),
            HumError::GenerateError(ref err) => write!(f, "GenerateError: {}", err),
            HumError::HumParseError(ref err) => write!(f, "HumParseError: {}", err),
        }
    }
}

impl error::Error for HumError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            HumError::FileSaveError(ref err) => Some(err),
            HumError::PlaybackError(ref err) => Some(err),
            HumError::GenerateError(ref err) => Some(err),
            HumError::HumParseError(ref err) => Some(err),
        }
    }
}

