use super::EditorState;
use super::utils::{
    CHECKPOINT_CHAR, RESET_CHAR, VOICE_PREFIX, construct_note_text, get_note_range_at_cursor,
    get_pitch_range_at_cursor, get_word_range_at_cursor, split_note_name_and_octave,
};

/// Add a character at the current cursor position and move the cursor forward.
pub fn insert_char(state: &mut EditorState, c: char) {
    state.text.insert_char(state.cursor_pos, c);
    state.cursor_pos += 1;
}

/// Delete the character before the current cursor position and move the cursor
/// back.
pub fn delete_char(state: &mut EditorState) {
    if state.cursor_pos > 0 {
        state.text.remove(state.cursor_pos - 1..state.cursor_pos);
        state.cursor_pos -= 1;
    }
}

/// Get the character index at the end of the current line.
pub fn get_current_eol_idx(state: &EditorState) -> usize {
    let line_idx = state.text.char_to_line(state.cursor_pos);
    let line_len = state.text.line(line_idx).len_chars();
    state.text.line_to_char(line_idx) + line_len
}

/// Insert a new line below the current line and move the cursor to the start of
/// that line.
pub fn insert_new_line_below(state: &mut EditorState) {
    let line_idx = state.text.char_to_line(state.cursor_pos);
    let line = state.text.line(line_idx);
    let has_newline = line.chars().last() == Some('\n');
    let current_eol_idx = get_current_eol_idx(state);

    state.text.insert_char(current_eol_idx, '\n');

    if has_newline {
        state.cursor_pos = current_eol_idx;
    } else {
        state.cursor_pos = current_eol_idx + 1;
    }
}

/// Prepares an empty line for insertion.
///
/// Unlike `insert_new_line_below`, this function checks if the current line is
/// already empty (or contains only whitespace). If so, it clears the line and
/// places the cursor there. Otherwise, it inserts a new line below.
fn prepare_empty_line(state: &mut EditorState) {
    let line_idx = state.text.char_to_line(state.cursor_pos);
    let line = state.text.line(line_idx);
    let line_str = line.to_string();

    if line_str.trim().is_empty() {
        let start = state.text.line_to_char(line_idx);
        let len = line.len_chars();
        let has_newline = line.chars().last() == Some('\n');
        let remove_len = if has_newline { len - 1 } else { len };

        if remove_len > 0 {
            state.text.remove(start..start + remove_len);
        }
        state.cursor_pos = start;
    } else {
        insert_new_line_below(state);
    }
}

/// Delete the entire current line where the cursor is located and move the
/// cursor to the start of the next line.
pub fn delete_line(state: &mut EditorState) {
    let line_idx = state.text.char_to_line(state.cursor_pos);
    let start_char = state.text.line_to_char(line_idx);

    let end_char = if line_idx + 1 < state.text.len_lines() {
        state.text.line_to_char(line_idx + 1)
    } else {
        state.text.len_chars()
    };

    state.text.remove(start_char..end_char);

    state.cursor_pos = state.cursor_pos.min(state.text.len_chars());
    state.message = "Deleted line".to_string();
}

/// Insert a string of text at the current cursor position and move the cursor
/// to the end of the inserted string.
pub fn insert_snippet(state: &mut EditorState, snippet: &str) {
    state.text.insert(state.cursor_pos, snippet);
    state.cursor_pos += snippet.chars().count();
    state.message = format!("Inserted snippet: '{}'", snippet.trim());
}

/// Appends a reset character (semicolon) to the end of the current line and
/// starts a new line below.
pub fn append_reset_and_newline(state: &mut EditorState) {
    let line_idx = state.text.char_to_line(state.cursor_pos);
    let line = state.text.line(line_idx);
    let has_newline = line.chars().last() == Some('\n');
    let line_start = state.text.line_to_char(line_idx);
    let line_len = line.len_chars();

    let insert_pos = if has_newline {
        line_start + line_len - 1
    } else {
        line_start + line_len
    };

    state.text.insert_char(insert_pos, RESET_CHAR);
    state.cursor_pos = insert_pos + 1;

    insert_new_line_below(state);
    state.message = "Inserted reset and newline".to_string();
}

/// Add a dot to the current duration setting.
pub fn add_dot_to_duration(state: &mut EditorState) {
    state.current_duration.push('+');
}

/// Remove a dot from the current duration setting.
pub fn remove_dot_from_duration(state: &mut EditorState) {
    if state.current_duration.ends_with('+') {
        state.current_duration.pop();
    }
}

/// Parse the current duration settin into fraction and dots.
pub fn parse_duration_setting(duration: &str) -> (String, String) {
    if let Some(idx) = duration.find('+') {
        let fraction = duration[..idx].to_string();
        let dots = duration[idx..].to_string();
        (fraction, dots)
    } else {
        (duration.to_string(), String::new())
    }
}

/// Insert a note at the current cursor position with the current duration and
/// octave settings. Play the note after insertion.
pub fn insert_note(state: &mut EditorState, note_char: char) {
    let duration = &state.current_duration;
    let (fraction, dots) = parse_duration_setting(duration);

    let note_name = format!("{}n", note_char.to_uppercase());
    let note = format!(
        "{}  ",
        construct_note_text(&note_name, state.current_octave.into(), &fraction, &dots)
    );

    insert_snippet(state, &note);
    super::playback::play_note(state, note.trim());
}

/// Insert a rest at the current cursor position with the current duration
// setting.
pub fn insert_rest(state: &mut EditorState) {
    let duration = &state.current_duration;
    let (fraction, dots) = parse_duration_setting(duration);
    let note = format!("(Rest {}){}  ", fraction, dots);
    insert_snippet(state, &note);
}

/// Delete the note at the current cursor position and move the cursor to its
/// start.
pub fn delete_note(state: &mut EditorState) {
    let range = get_note_range_at_cursor(state).or_else(|| get_word_range_at_cursor(state));

    if let Some((start, end)) = range {
        let word = state.text.slice(start..end).to_string();
        state.text.remove(start..end);

        // Also try to remove a following space if it exists, to keep spacing clean
        if start < state.text.len_chars() && state.text.char(start) == ' ' {
            state.text.remove(start..start + 1);
        }

        state.cursor_pos = start;
        state.message = format!("Deleted {}", word);
    }
}

/// Transpose the note at the current cursor position by the given delta in
/// semitones.
pub fn transpose_note(state: &mut EditorState, delta: i32) {
    if let Some((start, end)) = get_pitch_range_at_cursor(state) {
        let pitch = state.text.slice(start..end).to_string();

        let (note_name, octave) = match split_note_name_and_octave(&pitch) {
            Some((n, o)) => (n, o),
            None => {
                state.message = "Failed to parse note for transposition".to_string();
                return;
            }
        };

        match calculate_transposition(note_name, octave, delta) {
            Ok((new_note, _)) => {
                replace_note_at_cursor(state, start, end, &new_note);
                play_transposed_note(state);
            }
            Err(msg) => state.message = msg,
        }
    }
}

/// Calculates the new note name and octave after transposition.
fn calculate_transposition(
    note_name: &str,
    octave: i32,
    delta: i32,
) -> Result<(String, i32), String> {
    let sharps = crate::hum_process::hum_math::NOTES_SHARPS;
    let flats = crate::hum_process::hum_math::NOTES_FLATS;

    let (note_idx, use_sharps) = if let Some(idx) = sharps.iter().position(|&n| n == note_name) {
        (idx as i32, true)
    } else if let Some(idx) = flats.iter().position(|&n| n == note_name) {
        (idx as i32, false)
    } else {
        return Err("Invalid note name".to_string());
    };

    let notes_per_octave = sharps.len() as i32;
    if sharps.len() != flats.len() {
        return Err("Inconsistent note name arrays".to_string());
    }

    let current_pitch_idx = octave * notes_per_octave + note_idx;
    let new_pitch_idx = current_pitch_idx + delta;

    if new_pitch_idx < 0 {
        return Err("Transposition too low".to_string());
    }

    let new_octave = new_pitch_idx / notes_per_octave;
    let lowest_octave = crate::hum_process::hum_math::LOWEST_OCTAVE as i32;
    let highest_octave = crate::hum_process::hum_math::HIGHEST_OCTAVE as i32;

    if new_octave < lowest_octave || new_octave > highest_octave {
        return Err("Transposition out of octave range".to_string());
    }

    let new_note_idx = (new_pitch_idx % notes_per_octave) as usize;
    let new_note_name = if use_sharps {
        sharps[new_note_idx]
    } else {
        flats[new_note_idx]
    };

    Ok((format!("{}_{}", new_note_name, new_octave), new_octave))
}

/// Replaces the note text in the editor and updates the message.
fn replace_note_at_cursor(state: &mut EditorState, start: usize, end: usize, new_note: &str) {
    state.text.remove(start..end);
    state.text.insert(start, new_note);
    state.message = format!("Transposed to {}", new_note);
}

/// Plays the note at the cursor for preview.
fn play_transposed_note(state: &mut EditorState) {
    let play_text = if let Some((n_start, n_end)) = get_note_range_at_cursor(state) {
        state.text.slice(n_start..n_end).to_string()
    } else {
        // Fallback should ideally not happen if we just transposed successfully
        return;
    };
    super::playback::play_note(state, &play_text);
}

/// Insert a checkpoint line of the given length at the current cursor position
/// and move the cursor below it.
pub fn insert_checkpoint_line(state: &mut EditorState, length: usize) {
    prepare_empty_line(state);

    // Ensure blank line before: if the line above is not empty, insert an extra newline
    let line_idx = state.text.char_to_line(state.cursor_pos);
    if line_idx > 0 {
        let prev_line = state.text.line(line_idx - 1);
        if !prev_line.to_string().trim().is_empty() {
            insert_new_line_below(state);
        }
    }

    let checkpoint = CHECKPOINT_CHAR.to_string().repeat(length);
    insert_snippet(state, &checkpoint);

    insert_new_line_below(state);
    insert_new_line_below(state);

    state.message = "Inserted checkpoint".to_string();
}

/// Insert a voice command at the current cursor position and switch to insert
/// mode.
pub fn insert_voice_command(state: &mut EditorState) {
    prepare_empty_line(state);
    insert_snippet(state, VOICE_PREFIX);
    state.mode = super::Mode::Insert;
    state.message = "Enter voice name".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_transposition_sharps() {
        // C_4 + 1 semitone -> Cs_4
        let (note, octave) = calculate_transposition("Cn", 4, 1).unwrap();
        assert_eq!(note, "Cs_4");
        assert_eq!(octave, 4);

        // Cs_4 + 1 semitone -> Dn_4
        let (note, octave) = calculate_transposition("Cs", 4, 1).unwrap();
        assert_eq!(note, "Dn_4");
        assert_eq!(octave, 4);
    }

    #[test]
    fn test_calculate_transposition_flats() {
        // Df_4 - 1 semitone -> Cn_4
        let (note, octave) = calculate_transposition("Df", 4, -1).unwrap();
        assert_eq!(note, "Cn_4");
        assert_eq!(octave, 4);
    }

    #[test]
    fn test_calculate_transposition_octave_crossing() {
        // B_4 + 1 semitone -> C_5
        let (note, octave) = calculate_transposition("Bn", 4, 1).unwrap();
        assert_eq!(note, "Cn_5");
        assert_eq!(octave, 5);

        // C_4 - 1 semitone -> B_3
        let (note, octave) = calculate_transposition("Cn", 4, -1).unwrap();
        assert_eq!(note, "Bn_3");
        assert_eq!(octave, 3);
    }

    #[test]
    fn test_calculate_transposition_limits() {
        // Test upper limit (assuming HIGHEST_OCTAVE is 7)
        // Bn_7 + 1 -> Error
        let result = calculate_transposition("Bn", 7, 1);
        assert!(result.is_err());

        // Test lower limit (assuming LOWEST_OCTAVE is 0)
        // Cn_0 - 1 -> Error
        let result = calculate_transposition("Cn", 0, -1);
        assert!(result.is_err());
    }
}
