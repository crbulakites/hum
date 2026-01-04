use super::{EditorState, utils};

/// Moves the cursor one character to the left.
pub fn move_cursor_left(state: &mut EditorState) {
    if state.cursor_pos > 0 {
        state.cursor_pos -= 1;
    }
}

/// Moves the cursor one character to the right.
pub fn move_cursor_right(state: &mut EditorState) {
    if state.cursor_pos < state.text.len_chars() {
        state.cursor_pos += 1;
    }
}

/// Moves the cursor up one line, maintaining the column index if possible.
pub fn move_cursor_up(state: &mut EditorState) {
    let line_idx = state.text.char_to_line(state.cursor_pos);
    if line_idx > 0 {
        let current_col = state.cursor_pos - state.text.line_to_char(line_idx);
        let prev_line_idx = line_idx - 1;
        let prev_line_len = state.text.line(prev_line_idx).len_chars();
        // Adjust for newline char if present
        let new_col = current_col.min(prev_line_len.saturating_sub(1));
        state.cursor_pos = state.text.line_to_char(prev_line_idx) + new_col;
    }
}

/// Moves the cursor down one line, maintaining the column index if possible.
pub fn move_cursor_down(state: &mut EditorState) {
    let line_idx = state.text.char_to_line(state.cursor_pos);
    if line_idx < state.text.len_lines() - 1 {
        let current_col = state.cursor_pos - state.text.line_to_char(line_idx);
        let next_line_idx = line_idx + 1;
        let next_line_len = state.text.line(next_line_idx).len_chars();
        let new_col = current_col.min(next_line_len.saturating_sub(1));
        state.cursor_pos = state.text.line_to_char(next_line_idx) + new_col;
    }
}

/// Moves the cursor to the next measure bar `|`.
///
/// If no measure bar is found on the current line, moves to the end of the line.
pub fn move_to_next_measure(state: &mut EditorState) {
    let len = state.text.len_chars();
    let mut pos = state.cursor_pos + 1;
    while pos < len {
        if state.text.char(pos) == utils::MEASURE_CHAR {
            state.cursor_pos = pos;
            return;
        }
        pos += 1;
    }

    // Fallback: Go to end of current line
    let line_idx = state.text.char_to_line(state.cursor_pos);
    let line_start = state.text.line_to_char(line_idx);
    let line_len = state.text.line(line_idx).len_chars();

    let mut target = line_start + line_len;
    if line_len > 0 && state.text.char(target - 1) == '\n' {
        target -= 1;
    }

    state.cursor_pos = target;
    state.message = "Moved to end of line".to_string();
}

/// Moves the cursor to the previous measure bar `|`.
///
/// If no measure bar is found, moves to the start of the line.
pub fn move_to_prev_measure(state: &mut EditorState) {
    if state.cursor_pos == 0 {
        return;
    }
    let mut pos = state.cursor_pos - 1;
    loop {
        if state.text.char(pos) == utils::MEASURE_CHAR {
            state.cursor_pos = pos;
            return;
        }
        if pos == 0 {
            break;
        }
        pos -= 1;
    }

    // Fallback: Go to start of line
    let line_idx = state.text.char_to_line(state.cursor_pos);
    state.cursor_pos = state.text.line_to_char(line_idx);
    state.message = "Moved to start of line".to_string();
}

/// Moves the cursor to the start of the next note `(`.
pub fn move_to_next_note(state: &mut EditorState) {
    let len = state.text.len_chars();
    let mut pos = state.cursor_pos + 1;
    while pos < len {
        if state.text.char(pos) == utils::NOTE_START_CHAR {
            state.cursor_pos = pos;
            return;
        }
        pos += 1;
    }
    state.message = "No next note found".to_string();
}

/// Moves the cursor to the start of the previous note `(`.
pub fn move_to_prev_note(state: &mut EditorState) {
    if state.cursor_pos == 0 {
        return;
    }
    let mut pos = state.cursor_pos - 1;
    loop {
        if state.text.char(pos) == utils::NOTE_START_CHAR {
            state.cursor_pos = pos;
            return;
        }
        if pos == 0 {
            break;
        }
        pos -= 1;
    }
    state.message = "No previous note found".to_string();
}
