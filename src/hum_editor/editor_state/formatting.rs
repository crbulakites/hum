use crate::hum_editor::editor_state::utils::{CHECKPOINT_CHAR, COMMENT_CHAR, MEASURE_CHAR};
use crate::hum_parse::hum_grammar;
use ropey::Rope;

const FLOAT_COMPARISON_TOLERANCE: f64 = 1e-6;
const MIN_CHECKPOINT_LINE_LENGTH: usize = 79;
const MIN_NOTE_PADDING: usize = 2;
const QUARTERS_PER_WHOLE_NOTE: f64 = 4.0;
const SPACES_AROUND_DASHES: usize = 2;

/// Main entry point for formatting a text buffer.
///
/// This function identifies blocks of music code (delimited by checkpoint lines),
/// calculates the optimal layout for each block to ensure vertical alignment,
/// and reconstructs the text with consistent spacing.
pub fn format_buffer(text: &mut Rope) {
    let mut formatted_lines = Vec::new();
    let len_lines = text.len_lines();
    let blocks = identify_blocks(text);

    let mut current_line_idx = 0;

    for (start, end) in blocks {
        // Fast forward to the start of the block before formatting
        while current_line_idx < start {
            let line = text.line(current_line_idx).to_string();
            formatted_lines.push(line);
            current_line_idx += 1;
        }

        // Calculate layout for the entire block to ensure alignment
        let layouts = calculate_block_layout(text, start, end);

        // Format each line in the block
        for i in start..end {
            let line = text.line(i).to_string();
            formatted_lines.push(format_block_line(&line, &layouts));
            current_line_idx += 1;
        }
    }

    // Append remaining lines after the last block
    while current_line_idx < len_lines {
        let line = text.line(current_line_idx).to_string();
        formatted_lines.push(line);
        current_line_idx += 1;
    }

    let new_text = apply_checkpoint_expansion(formatted_lines);
    *text = Rope::from_str(&new_text);
}

/// Identifies the start and end line indices of music blocks.
///
/// Blocks are sections of code delimited by checkpoint lines (lines starting with '*').
fn identify_blocks(text: &Rope) -> Vec<(usize, usize)> {
    let mut blocks = Vec::new();
    let mut start_line = 0;
    let len_lines = text.len_lines();

    for i in 0..len_lines {
        let line = text.line(i).to_string();

        if line.trim().starts_with(CHECKPOINT_CHAR) {
            if i > start_line {
                blocks.push((start_line, i));
            }
            start_line = i + 1;
        }
    }

    if start_line < len_lines {
        blocks.push((start_line, len_lines));
    }

    blocks
}

/// Calculates the max line length and expands checkpoint lines to match.
fn apply_checkpoint_expansion(lines: Vec<String>) -> String {
    // Calculate max line length (ignoring checkpoints)
    let max_len = lines
        .iter()
        .filter(|line| !line.trim().starts_with(CHECKPOINT_CHAR))
        .map(|line| line.trim_end().len())
        .max()
        .unwrap_or(0);

    let checkpoint_len = std::cmp::max(MIN_CHECKPOINT_LINE_LENGTH, max_len);
    let checkpoint_line = CHECKPOINT_CHAR.to_string().repeat(checkpoint_len);

    let mut result = String::with_capacity(lines.iter().map(|l| l.len()).sum());

    for line in lines {
        if line.trim().starts_with(CHECKPOINT_CHAR) {
            result.push_str(&checkpoint_line);
            result.push('\n');
        } else {
            result.push_str(&line);
        }
    }

    result
}

// --- Layout Calculation ---

#[derive(Debug, Clone)]
struct Segment {
    start: f64,
    end: f64,
    width: f64,
}

#[derive(Debug, Clone)]
struct MeasureLayout {
    segments: Vec<Segment>,
}

/// Calculates the layout for a block of music code.
///
/// This involves:
/// 1. Collecting all note events across all lines in the block.
/// 2. Grouping events by measure.
/// 3. Calculating the optimal width for each time segment within a measure.
fn calculate_block_layout(text: &Rope, start: usize, end: usize) -> Vec<MeasureLayout> {
    let measure_events = collect_block_events(text, start, end);

    measure_events
        .into_iter()
        .map(calculate_measure_layout)
        .collect()
}

/// Collects all note events in a block, grouped by measure index.
///
/// Returns a vector where each element represents a measure, containing a list
/// of events (start_time, duration, min_width).
fn collect_block_events(text: &Rope, start: usize, end: usize) -> Vec<Vec<(f64, f64, f64)>> {
    let mut measure_events: Vec<Vec<(f64, f64, f64)>> = Vec::new();

    for i in start..end {
        let line = text.line(i).to_string();
        if let Ok(commands) = hum_grammar::score(&line) {
            let mut measure_idx: i32 = -1;
            let mut current_time = 0.0;

            for (verb, noun) in commands {
                if verb == "measure" {
                    measure_idx += 1;
                    current_time = 0.0;
                } else if !is_reserved_command(&verb) {
                    // It's a note
                    let quarters = parse_duration(&noun);

                    if quarters > 0.0 {
                        let min_width = calculate_note_min_width(&verb, &noun);

                        let idx = if measure_idx < 0 {
                            0
                        } else {
                            measure_idx as usize
                        };
                        while idx >= measure_events.len() {
                            measure_events.push(Vec::new());
                        }

                        measure_events[idx].push((current_time, quarters, min_width));
                        current_time += quarters;
                    }
                }
            }
        }
    }
    measure_events
}

/// Calculates the layout segments for a single measure.
///
/// Uses a "Skyline" algorithm to distribute width requirements across time segments.
fn calculate_measure_layout(events: Vec<(f64, f64, f64)>) -> MeasureLayout {
    let cut_points = identify_cut_points(&events);
    let mut segments = create_initial_segments(&cut_points);

    distribute_widths_to_segments(&mut segments, events);

    MeasureLayout { segments }
}

/// Identifies all unique time points where note events start or end.
fn identify_cut_points(events: &[(f64, f64, f64)]) -> Vec<f64> {
    let mut cut_points = vec![0.0];

    for (start, dur, _) in events {
        cut_points.push(*start);
        cut_points.push(*start + *dur);
    }

    cut_points.sort_by(|a, b| a.partial_cmp(b).unwrap());
    cut_points.dedup_by(|a, b| (*a - *b).abs() < FLOAT_COMPARISON_TOLERANCE);
    cut_points
}

/// Creates initial zero-width segments from cut points.
fn create_initial_segments(cut_points: &[f64]) -> Vec<Segment> {
    let mut segments = Vec::new();

    for i in 0..cut_points.len().saturating_sub(1) {
        segments.push(Segment {
            start: cut_points[i],
            end: cut_points[i + 1],
            width: 0.0,
        });
    }

    segments
}

/// Distributes the minimum width requirements of events onto the segments.
///
/// Events are processed shortest-first to ensure local density is handled before
/// global density.
fn distribute_widths_to_segments(segments: &mut [Segment], mut events: Vec<(f64, f64, f64)>) {
    // Sort events by duration (shortest first)
    events.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (start, dur, min_width) in events {
        let end = start + dur;

        let mut covered_indices = Vec::new();
        let mut current_width = 0.0;
        let mut total_duration = 0.0;

        for (i, seg) in segments.iter().enumerate() {
            let seg_mid = (seg.start + seg.end) / 2.0;
            if seg_mid >= start && seg_mid < end {
                covered_indices.push(i);
                current_width += seg.width;
                total_duration += seg.end - seg.start;
            }
        }

        if current_width < min_width {
            let deficit = min_width - current_width;
            if total_duration > 0.0 {
                for idx in covered_indices {
                    let seg = &mut segments[idx];
                    let weight = (seg.end - seg.start) / total_duration;
                    seg.width += deficit * weight;
                }
            }
        }
    }
}

// --- Line Reconstruction ---

/// Formats a single line within a music block.
///
/// This function handles:
/// 1. Preserving manual comments (lines starting with '~').
/// 2. Parsing the line into commands.
/// 3. Reconstructing the line using the calculated layout.
fn format_block_line(line: &str, layouts: &[MeasureLayout]) -> String {
    if line.trim_start().starts_with(COMMENT_CHAR) {
        return line.to_string();
    }

    match hum_grammar::score(line) {
        Ok(commands) => {
            let mut formatted = reconstruct_line(commands, layouts);
            if line.ends_with('\n') {
                formatted.push('\n');
            }
            formatted
        }
        Err(_) => line.to_string(), // Fallback if parsing fails
    }
}

/// Reconstructs a line of code from parsed commands and the calculated layout.
fn reconstruct_line(commands: Vec<(String, String)>, layouts: &[MeasureLayout]) -> String {
    let mut result = String::new();
    let mut measure_idx: i32 = -1;
    let mut current_time = 0.0;
    let cmd_len = commands.len();

    for (i, (verb, noun)) in commands.iter().enumerate() {
        if is_reserved_command(verb) {
            result.push_str(&format_reserved_command(verb, noun));
            if verb == "measure" {
                measure_idx += 1;
                current_time = 0.0;
            }
        } else {
            result.push_str(&format_note_command(
                verb,
                noun,
                measure_idx,
                &mut current_time,
                layouts,
                i < cmd_len - 1,
            ));
        }
    }

    result.trim_end().to_string()
}

/// Formats reserved commands (non-notes).
fn format_reserved_command(verb: &str, noun: &str) -> String {
    match verb {
        "measure" => format!("{} ", MEASURE_CHAR),
        "reset" => {
            if noun.is_empty() {
                ";".to_string()
            } else {
                format!("; {}", noun)
            }
        }
        "voice" => format!("% {} ", noun),
        "tempo" => format!("[ {}_bpm ] ", noun),
        "time" => format!("[ {} ] ", noun),
        "comment" => format!("{} {}", COMMENT_CHAR, noun),
        "checkpoint" => CHECKPOINT_CHAR.to_string(),
        _ => String::new(),
    }
}

/// Formats a note command, including calculating padding based on the layout.
fn format_note_command(
    verb: &str,
    noun: &str,
    measure_idx: i32,
    current_time: &mut f64,
    layouts: &[MeasureLayout],
    has_next_command: bool,
) -> String {
    let quarters = parse_duration(noun);
    let note_str = format_note_token(verb, noun);

    if quarters <= 0.0 {
        return format!("{} ", note_str);
    }

    let target_len =
        calculate_target_length(measure_idx, *current_time, quarters, layouts, &note_str);
    *current_time += quarters;

    apply_padding(&note_str, target_len, has_next_command)
}

/// Calculates the target display length for a note based on the measure layout.
fn calculate_target_length(
    measure_idx: i32,
    start_time: f64,
    duration: f64,
    layouts: &[MeasureLayout],
    note_str: &str,
) -> usize {
    let idx = if measure_idx < 0 {
        0
    } else {
        measure_idx as usize
    };

    if idx < layouts.len() {
        let layout = &layouts[idx];
        let end_time = start_time + duration;
        let mut target_len = 0.0;

        for seg in &layout.segments {
            let seg_mid = (seg.start + seg.end) / 2.0;
            if seg_mid >= start_time && seg_mid < end_time {
                target_len += seg.width;
            }
        }
        target_len.ceil() as usize
    } else {
        note_str.len() + MIN_NOTE_PADDING // Fallback if no layout found
    }
}

/// Applies padding (spaces and dashes) to a note string to reach the target length.
fn apply_padding(note_str: &str, target_len: usize, has_next_command: bool) -> String {
    let mut result = String::with_capacity(target_len);
    result.push_str(note_str);

    if target_len > note_str.len() {
        let padding_space = target_len - note_str.len();

        result.push(' ');

        if padding_space >= SPACES_AROUND_DASHES {
            let dash_count = padding_space - SPACES_AROUND_DASHES;

            if dash_count > 0 && has_next_command {
                result.push_str(&"-".repeat(dash_count));
            }

            result.push(' ');
        }
    } else {
        result.push(' ');
    }

    result
}

// --- Helpers ---

/// Checks if a command verb is a reserved keyword (not a note).
fn is_reserved_command(verb: &str) -> bool {
    matches!(
        verb,
        "comment" | "tempo" | "time" | "checkpoint" | "voice" | "measure" | "reset"
    )
}

/// Parses a duration string (e.g., "1/4+") into quarter notes.
fn parse_duration(duration_str: &str) -> f64 {
    let pluses = duration_str.matches('+').count();
    let clean_duration = duration_str.replace("+", "");

    let parts: Vec<&str> = clean_duration.split('/').collect();
    if parts.len() != 2 {
        return 0.0;
    }

    let num: f64 = parts[0].parse().unwrap_or(0.0);
    let den: f64 = parts[1].parse().unwrap_or(1.0);

    if den == 0.0 {
        return 0.0;
    }

    let base_value = num / den;

    // Apply pluses: each plus adds half the value of the previous dot
    let multiplier = 2.0 - (0.5f64).powi(pluses as i32);
    let total_value = base_value * multiplier;

    // Convert to quarters (1/4 = 1.0)
    total_value * QUARTERS_PER_WHOLE_NOTE
}

/// Calculates the minimum display width for a note token.
///
/// Includes the note string `(Note Duration)` plus padding for dashes.
fn calculate_note_min_width(verb: &str, noun: &str) -> f64 {
    let note_str = format_note_token(verb, noun);
    (note_str.len() + MIN_NOTE_PADDING) as f64
}

/// Formats a note token string: `(Verb Noun)`.
///
/// Handles the placement of dots/pluses outside the parentheses.
fn format_note_token(verb: &str, noun: &str) -> String {
    let pluses = noun.matches('+').count();
    let clean_noun = noun.replace("+", "");
    let plus_str = "+".repeat(pluses);
    format!("({} {}){}", verb, clean_noun, plus_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        // 1/4 = 1.0 quarter notes
        assert!((parse_duration("1/4") - 1.0).abs() < FLOAT_COMPARISON_TOLERANCE);

        // 1/4+ = 1.5 quarter notes
        assert!((parse_duration("1/4+") - 1.5).abs() < FLOAT_COMPARISON_TOLERANCE);

        // 1/4++ = 1.75 quarter notes
        assert!((parse_duration("1/4++") - 1.75).abs() < FLOAT_COMPARISON_TOLERANCE);

        // 1/4+++ = 1.875 quarter notes
        assert!((parse_duration("1/4+++") - 1.875).abs() < FLOAT_COMPARISON_TOLERANCE);
    }
}
