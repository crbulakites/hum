/*
Hum: A Music Markup Language Synthesizer
Copyright (C) 2026 Connor R. Bulakites

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

use hum::SAMPLE_RATE;
use hum::hum_parse::hum_grammar;
use hum::hum_process;

#[test]
fn test_dotted_note_duration() {
    // 1/4 note at 60 BPM (1 beat per second) -> 1 second duration
    // 44100 samples
    let score_base = "[ 60_bpm ] (An_4 1/4)";
    let commands_base = hum_grammar::score(score_base).unwrap();
    let audio_base = hum_process::run_commands(commands_base).unwrap();
    let base_len = audio_base.len();

    // 1/4+ note -> 1.5 seconds
    let score_dot1 = "[ 60_bpm ] (An_4 1/4)+";
    let commands_dot1 = hum_grammar::score(score_dot1).unwrap();
    let audio_dot1 = hum_process::run_commands(commands_dot1).unwrap();
    let dot1_len = audio_dot1.len();

    // 1/4++ note -> 1.75 seconds
    let score_dot2 = "[ 60_bpm ] (An_4 1/4)++";
    let commands_dot2 = hum_grammar::score(score_dot2).unwrap();
    let audio_dot2 = hum_process::run_commands(commands_dot2).unwrap();
    let dot2_len = audio_dot2.len();

    assert_eq!(
        base_len, SAMPLE_RATE as usize,
        "Base note duration incorrect"
    );
    assert_eq!(
        dot1_len,
        (SAMPLE_RATE as f32 * 1.5) as usize,
        "Single dot duration incorrect"
    );

    // This assertion is expected to fail with the current implementation
    assert_eq!(
        dot2_len,
        (SAMPLE_RATE as f32 * 1.75) as usize,
        "Double dot duration incorrect"
    );
}
