use super::EditorState;

const PARSE_LOOKBACK_LIMIT: usize = 100;
const DEFAULT_VOICE: &str = "sine";

pub const CHECKPOINT_CHAR: char = '*';
pub const COMMENT_CHAR: char = '~';
pub const MEASURE_CHAR: char = '|';
pub const RESET_CHAR: char = ';';
pub const NOTE_START_CHAR: char = '(';
pub const NOTE_END_CHAR: char = ')';
pub const NOTE_DOT_CHAR: char = '+';
pub const NOTE_OCTAVE_SEPARATOR: char = '_';
pub const TEMPO_SUFFIX: &str = "_bpm";
pub const TIME_SIG_START: char = '[';
pub const TIME_SIG_SEPARATOR: char = '/';
pub const TIME_SIG_END: char = ']';
pub const VOICE_PREFIX: &str = "% ";
pub const DEFAULT_CHECKPOINT_LENGTH: usize = 71;

pub const HELP_TEXT: &[&str] = &[
    "Keyboard Shortcuts",
    "",
    "Navigation:",
    "  h, j, k, l / Arrows : Move cursor",
    "  m / M               : Next / Prev measure",
    "  > / <               : Next / Prev note",
    "",
    "Editing:",
    "  i                   : Insert mode (Esc to exit)",
    "  x                   : Delete note",
    "  D                   : Delete line",
    "  u / R               : Undo / Redo",
    "  n                   : New line below",
    "  ;                   : End line & new line",
    "",
    "Note Entry:",
    "  a-g                 : Insert note",
    "  r                   : Insert rest",
    "  ] / [               : Transpose +1 / -1 semitone",
    "  } / {               : Octave +1 / -1",
    "",
    "Duration:",
    "  1, 2, 4, 8, 6, 3    : Set duration (1/1 to 1/32)",
    "  + / -               : Add / Remove dot",
    "",
    "Commands:",
    "  |                   : Insert measure",
    "  *                   : Insert checkpoint",
    "  %                   : Insert voice",
    "  F                   : Format file",
    "",
    "Playback:",
    "  p                   : Play file",
    "  P                   : Play section",
    "  Esc                 : Stop playback",
    "",
    "General:",
    "  w                   : Save file",
    "  q                   : Quit",
    "  ?                   : Toggle Help",
];

/// Checks if a line is a comment line (starts with `~`).
pub fn is_comment_line(line: &str) -> bool {
    line.trim_start().starts_with(COMMENT_CHAR)
}

/// Checks if a line is a checkpoint line (starts with `*`).
pub fn is_checkpoint_line(line: &str) -> bool {
    line.trim_start().starts_with(CHECKPOINT_CHAR)
}

/// Checks if a line is a tempo command (contains `_bpm`).
pub fn is_tempo_line(line: &str) -> bool {
    line.contains(TEMPO_SUFFIX)
}

/// Checks if a line is a time signature command (contains `[`, `/`, `]`).
pub fn is_time_signature_line(line: &str) -> bool {
    line.contains(TIME_SIG_START)
        && line.contains(TIME_SIG_SEPARATOR)
        && line.contains(TIME_SIG_END)
}

/// Checks if a line is a voice command (starts with `%`).
pub fn is_voice_line(line: &str) -> bool {
    line.trim().starts_with(VOICE_PREFIX.trim())
}

/// Constructs a formatted note string.
///
/// Format: `(NoteName_Octave Duration)Dots`
/// Example: `(C_4 1/4)+`
pub fn construct_note_text(note_name: &str, octave: i32, duration: &str, dots: &str) -> String {
    format!("({}_{} {}){}", note_name, octave, duration, dots)
}

/// Gets the range (start, end) of the note at the current cursor position.
///
/// A note is defined as a sequence enclosed in parentheses `(...)`, potentially
/// followed by dots `+`.
///
/// Returns `None` if the cursor is not currently inside or on a note.
pub fn get_note_range_at_cursor(state: &EditorState) -> Option<(usize, usize)> {
    let len = state.text.len_chars();
    if state.cursor_pos >= len {
        return None;
    }

    let mut start = state.cursor_pos;
    let mut i = 0;
    let mut found_open = false;

    // Scan backwards for '('
    while i < PARSE_LOOKBACK_LIMIT {
        let c = state.text.char(start);
        if c == NOTE_START_CHAR {
            found_open = true;
            break;
        }
        if c == NOTE_END_CHAR && start != state.cursor_pos {
            // Found a closing paren that is not the one we might be standing on
            break;
        }
        if start == 0 {
            break;
        }
        start -= 1;
        i += 1;
    }

    if !found_open {
        return None;
    }

    // Scan forwards for ')'
    let mut end = start;
    let mut found_close = false;
    i = 0;
    while end < len && i < PARSE_LOOKBACK_LIMIT {
        let c = state.text.char(end);
        if c == NOTE_END_CHAR {
            found_close = true;
            end += 1; // Include ')'
            break;
        }
        end += 1;
        i += 1;
    }

    if found_open && found_close {
        // Check for trailing dots
        while end < len {
            let c = state.text.char(end);
            if c == NOTE_DOT_CHAR {
                end += 1;
            } else {
                break;
            }
        }

        if state.cursor_pos >= start && state.cursor_pos < end {
            return Some((start, end));
        }
    }

    None
}

/// Gets the range (start, end) of the word at the current cursor position.
///
/// A word consists of alphanumeric characters and underscores.
pub fn get_word_range_at_cursor(state: &EditorState) -> Option<(usize, usize)> {
    let len = state.text.len_chars();
    if state.cursor_pos >= len {
        return None;
    }

    let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

    // Check if current char is part of a word
    let current_char = state.text.char(state.cursor_pos);
    if !is_word_char(current_char) {
        return None;
    }

    let mut start = state.cursor_pos;
    while start > 0 {
        let c = state.text.char(start - 1);
        if !is_word_char(c) {
            break;
        }
        start -= 1;
    }

    let mut end = state.cursor_pos + 1;
    while end < len {
        let c = state.text.char(end);
        if !is_word_char(c) {
            break;
        }
        end += 1;
    }

    Some((start, end))
}

/// Gets the range (start, end) of the pitch part of a note at the cursor.
///
/// The pitch part is the text inside the parentheses before the first space.
/// Example: In `(C_4 1/4)`, the pitch range covers `C_4`.
pub fn get_pitch_range_at_cursor(state: &EditorState) -> Option<(usize, usize)> {
    if let Some((start, end)) = get_note_range_at_cursor(state) {
        let note_text = state.text.slice(start..end).to_string();
        let open_paren_idx = note_text.find(NOTE_START_CHAR)?;
        let content = &note_text[open_paren_idx + 1..];
        let space_idx = content.find(' ')?;

        let pitch_start = start + open_paren_idx + 1;
        let pitch_end = pitch_start + space_idx;
        Some((pitch_start, pitch_end))
    } else {
        None
    }
}

/// Splits a note string (e.g., "C_4") into its name ("C") and octave (4).
pub fn split_note_name_and_octave(note: &str) -> Option<(&str, i32)> {
    let parts: Vec<&str> = note.split(NOTE_OCTAVE_SEPARATOR).collect();

    if parts.len() != 2 {
        return None;
    }

    let note_part = parts[0];
    let octave_part = parts[1];

    let octave: i32 = match octave_part.parse() {
        Ok(o) => o,
        Err(_) => return None,
    };

    Some((note_part, octave))
}

/// Gets the name of the active voice at the current cursor position.
///
/// Scans backwards for the nearest `% voice_name` command. Defaults to "sine".
pub fn get_voice_at_cursor(state: &EditorState) -> String {
    let line_idx = state.text.char_to_line(state.cursor_pos);

    // Scan backwards from current line
    for i in (0..=line_idx).rev() {
        let line = state.text.line(i).to_string();
        let trimmed = line.trim();
        if trimmed.starts_with(VOICE_PREFIX.trim()) {
            // Extract voice name: "% voice_name"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                return parts[1].to_string();
            }
        }
    }

    DEFAULT_VOICE.to_string()
}
