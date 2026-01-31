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

use hum::hum_editor::editor_state::formatting;
use ropey::Rope;
use std::fs;
use std::path::Path;

fn make_dirty(input: &str) -> String {
    let mut output = String::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('~') {
            output.push_str(line);
        } else if trimmed.starts_with('*') {
            output.push_str("***");
        } else if trimmed.is_empty() {
            output.push_str(line);
        } else {
            let no_hyphens = line.replace('-', " ");
            let parts: Vec<&str> = no_hyphens.split_whitespace().collect();
            let dirty_line = parts.join(" ");
            output.push_str(&dirty_line);
        }
        output.push('\n');
    }

    output
}

fn test_format_file(filename: &str) {
    let path = Path::new(filename);
    let golden_content = fs::read_to_string(path).expect("Failed to read golden file");
    let dirty_content = make_dirty(&golden_content);
    let mut rope = Rope::from_str(&dirty_content);

    formatting::format_buffer(&mut rope);

    let formatted_content = rope.to_string();
    let formatted_norm = formatted_content.trim();
    let golden_norm = golden_content.trim();

    if formatted_norm != golden_norm {
        println!("Mismatch in file: {}", filename);

        let exp_lines: Vec<&str> = golden_norm.lines().collect();
        let act_lines: Vec<&str> = formatted_norm.lines().collect();

        let max_len = std::cmp::max(exp_lines.len(), act_lines.len());

        for i in 0..max_len {
            let exp = exp_lines.get(i).unwrap_or(&"<END>");
            let act = act_lines.get(i).unwrap_or(&"<END>");

            if exp != act {
                println!("Line {}: Mismatch", i + 1);
                println!("  Exp: '{}'", exp);
                println!("  Act: '{}'", act);
            }
        }

        panic!(
            "Formatted content does not match golden file for {}",
            filename
        );
    }
}

#[test]
fn test_simple_formatting() {
    test_format_file("simple_sample.hum");
}

#[test]
fn test_daisy_formatting() {
    test_format_file("daisy.hum");
}

#[test]
fn test_complex_formatting() {
    test_format_file("complex.hum");
}
