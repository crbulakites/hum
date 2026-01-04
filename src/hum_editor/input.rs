use super::editor_state::utils::{
    CHECKPOINT_CHAR, DEFAULT_CHECKPOINT_LENGTH, HELP_TEXT, MEASURE_CHAR, RESET_CHAR,
};
use super::editor_state::{EditorState, Mode};
use crossterm::event::{KeyCode, KeyEvent};

const DURATION_WHOLE: &str = "1/1";
const DURATION_HALF: &str = "1/2";
const DURATION_QUARTER: &str = "1/4";
const DURATION_EIGHTH: &str = "1/8";
const DURATION_SIXTEENTH: &str = "1/16";
const DURATION_THIRTYSECOND: &str = "1/32";

/// Handles keyboard input in Normal mode.
///
/// Returns `true` if the editor should exit.
pub fn handle_normal_input(state: &mut EditorState, key: KeyEvent) -> bool {
    if state.show_help {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                state.help_scroll_offset = state.help_scroll_offset.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max_scroll = HELP_TEXT.len().saturating_sub(1) as u16;
                if state.help_scroll_offset < max_scroll {
                    state.help_scroll_offset += 1;
                }
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
                state.show_help = false;
                state.help_scroll_offset = 0;
            }
            _ => {}
        }
        return false;
    }

    match key.code {
        // Help
        KeyCode::Char('?') => state.show_help = true,

        // Stop playback
        KeyCode::Esc => state.stop_playback(),

        // Move cursor
        KeyCode::Char('h') => state.move_cursor_left(),
        KeyCode::Char('l') => state.move_cursor_right(),
        KeyCode::Char('k') => state.move_cursor_up(),
        KeyCode::Char('j') => state.move_cursor_down(),
        KeyCode::Left => state.move_cursor_left(),
        KeyCode::Right => state.move_cursor_right(),
        KeyCode::Up => state.move_cursor_up(),
        KeyCode::Down => state.move_cursor_down(),
        KeyCode::Char('m') => state.move_to_next_measure(),
        KeyCode::Char('M') => state.move_to_prev_measure(),
        KeyCode::Char('<') => state.move_to_prev_note(),
        KeyCode::Char('>') => state.move_to_next_note(),

        // Switch modes
        KeyCode::Char('q') => return true,
        KeyCode::Char('i') => state.mode = Mode::Insert,

        // Save file
        KeyCode::Char('w') => {
            if let Err(e) = state.save_file() {
                state.message = format!("Error saving: {}", e);
            }
        }
        // Insert note
        KeyCode::Char(c) if matches!(c, 'a'..='g') => state.insert_note(c),
        KeyCode::Char('r') => state.insert_rest(),

        // Delete note
        KeyCode::Char('x') => state.delete_note(),

        // Transpose note
        KeyCode::Char(']') => state.transpose_note(1),
        KeyCode::Char('[') => state.transpose_note(-1),

        // Change octave
        KeyCode::Char('{') => state.decrement_octave(),
        KeyCode::Char('}') => state.increment_octave(),

        // Duration control
        KeyCode::Char('1') => state.set_duration(DURATION_WHOLE),
        KeyCode::Char('2') => state.set_duration(DURATION_HALF),
        KeyCode::Char('3') => state.set_duration(DURATION_THIRTYSECOND),
        KeyCode::Char('4') => state.set_duration(DURATION_QUARTER),
        KeyCode::Char('6') => state.set_duration(DURATION_SIXTEENTH),
        KeyCode::Char('8') => state.set_duration(DURATION_EIGHTH),
        KeyCode::Char('+') => state.add_dot_to_duration(),
        KeyCode::Char('-') => state.remove_dot_from_duration(),

        // Playback
        KeyCode::Char('p') => state.play_file(),
        KeyCode::Char('P') => state.play_from_cursor(),

        // Formatting
        KeyCode::Char('F') => state.format_file(),

        // Structure editing
        KeyCode::Char('n') => state.insert_new_line_below(),
        KeyCode::Char(c) if c == MEASURE_CHAR => {
            state.insert_snippet(&format!("{} ", MEASURE_CHAR))
        }
        KeyCode::Char(c) if c == CHECKPOINT_CHAR => {
            state.insert_checkpoint_line(DEFAULT_CHECKPOINT_LENGTH)
        }
        KeyCode::Char(c) if c == RESET_CHAR => state.append_reset_and_newline(),
        KeyCode::Char('%') => state.insert_voice_command(),
        KeyCode::Char('D') => state.delete_line(),

        // Undo/Redo
        KeyCode::Char('u') => state.undo(),
        KeyCode::Char('R') => state.redo(),

        _ => {}
    }
    false
}

/// Handles keyboard input in Insert mode.
pub fn handle_insert_input(state: &mut EditorState, key: KeyEvent) {
    match key.code {
        KeyCode::Left => state.move_cursor_left(),
        KeyCode::Right => state.move_cursor_right(),
        KeyCode::Up => state.move_cursor_up(),
        KeyCode::Down => state.move_cursor_down(),
        KeyCode::Esc => state.mode = Mode::Normal,
        KeyCode::Tab => state.insert_tab(),
        KeyCode::Char(c) => state.insert_char(c),
        KeyCode::Backspace => state.delete_char(),
        KeyCode::Enter => state.insert_char('\n'),
        _ => {}
    }
}
