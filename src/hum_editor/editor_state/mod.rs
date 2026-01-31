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

use crate::hum_process::hum_math::{HIGHEST_OCTAVE, LOWEST_OCTAVE};
use ropey::Rope;
use std::io;

pub mod editing;
pub mod formatting;
pub mod movement;
pub mod playback;
pub mod utils;

const PLAYER_ENV_VAR_NAME: &str = "HUM_PLAYER";
const MACOS_DEFAULT_PLAYER: &str = "afplay";
const LINUX_DEFAULT_PLAYER: &str = "aplay";
const DEFAULT_OCTAVE: u8 = 4;
const DEFAULT_DURATION: &str = "1/4";
const TAB_STRING: &str = "    ";

/// The current editing mode.
pub enum Mode {
    /// Normal mode for navigation and command execution.
    Normal,
    /// Insert mode for typing text.
    Insert,
}

/// The global state of the editor.
pub struct EditorState {
    /// The text buffer being edited.
    pub text: Rope,
    /// The current editing mode.
    pub mode: Mode,
    /// The current cursor position (character index).
    pub cursor_pos: usize,
    /// The vertical scroll offset (line index).
    pub scroll_offset: usize,
    /// The horizontal scroll offset (column index).
    pub col_scroll_offset: usize,
    /// The name of the file being edited, if any.
    pub filename: Option<String>,
    /// A status message to display to the user.
    pub message: String,
    /// The current octave for note insertion.
    pub current_octave: u8,
    /// The current duration for note insertion.
    pub current_duration: String,
    /// The command used for audio playback.
    pub playback_command: String,
    /// The stack of undoable states.
    pub undo_stack: Vec<(Rope, usize)>,
    /// The stack of redoable states.
    pub redo_stack: Vec<(Rope, usize)>,
    /// The currently running playback process, if any.
    pub playback_process: Option<std::process::Child>,
    /// Whether to show the help screen.
    pub show_help: bool,
    /// Scroll offset for the help screen.
    pub help_scroll_offset: u16,
}

impl Default for EditorState {
    fn default() -> EditorState {
        let default_player = if let Ok(player) = std::env::var(PLAYER_ENV_VAR_NAME) {
            player
        } else if cfg!(target_os = "macos") {
            MACOS_DEFAULT_PLAYER.to_string()
        } else {
            LINUX_DEFAULT_PLAYER.to_string()
        };

        EditorState {
            text: Rope::new(),
            mode: Mode::Normal,
            cursor_pos: 0,
            scroll_offset: 0,
            col_scroll_offset: 0,
            filename: None,
            message: String::new(),
            current_octave: DEFAULT_OCTAVE,
            current_duration: DEFAULT_DURATION.to_string(),
            playback_command: default_player,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            playback_process: None,
            show_help: false,
            help_scroll_offset: 0,
        }
    }
}

impl EditorState {
    /// Saves the current buffer to the file.
    pub fn save_file(&mut self) -> io::Result<()> {
        playback::save_file(self)
    }

    /// Plays the entire file.
    pub fn play_file(&mut self) {
        playback::play_file(self)
    }

    /// Plays from the current cursor position.
    pub fn play_from_cursor(&mut self) {
        playback::play_from_cursor(self)
    }

    /// Stops any active playback.
    pub fn stop_playback(&mut self) {
        playback::stop_playback(self)
    }

    /// Moves the cursor left.
    pub fn move_cursor_left(&mut self) {
        movement::move_cursor_left(self)
    }

    /// Moves the cursor right.
    pub fn move_cursor_right(&mut self) {
        movement::move_cursor_right(self)
    }

    /// Moves the cursor up.
    pub fn move_cursor_up(&mut self) {
        movement::move_cursor_up(self)
    }

    /// Moves the cursor down.
    pub fn move_cursor_down(&mut self) {
        movement::move_cursor_down(self)
    }

    /// Moves the cursor to the next measure.
    pub fn move_to_next_measure(&mut self) {
        movement::move_to_next_measure(self)
    }

    /// Moves the cursor to the previous measure.
    pub fn move_to_prev_measure(&mut self) {
        movement::move_to_prev_measure(self)
    }

    /// Moves the cursor to the next note.
    pub fn move_to_next_note(&mut self) {
        movement::move_to_next_note(self)
    }

    /// Adjusts the scroll offset to ensure the cursor is visible.
    pub fn scroll_into_view(&mut self, width: usize, height: usize) {
        let cursor_line = self.text.char_to_line(self.cursor_pos);
        if cursor_line < self.scroll_offset {
            self.scroll_offset = cursor_line;
        } else if cursor_line >= self.scroll_offset + height {
            self.scroll_offset = cursor_line - height + 1;
        }

        let line_start_char = self.text.line_to_char(cursor_line);
        let cursor_col = self.cursor_pos - line_start_char;

        if cursor_col < self.col_scroll_offset {
            self.col_scroll_offset = cursor_col;
        } else if cursor_col >= self.col_scroll_offset + width {
            self.col_scroll_offset = cursor_col - width + 1;
        }
    }

    /// Moves the cursor to the previous note.
    pub fn move_to_prev_note(&mut self) {
        movement::move_to_prev_note(self)
    }

    /// Inserts a character at the current cursor position.
    pub fn insert_char(&mut self, c: char) {
        self.save_snapshot();
        editing::insert_char(self, c)
    }

    /// Inserts a new line below the current line.
    pub fn insert_new_line_below(&mut self) {
        self.save_snapshot();
        editing::insert_new_line_below(self)
    }

    /// Deletes the character before the cursor.
    pub fn delete_char(&mut self) {
        self.save_snapshot();
        editing::delete_char(self)
    }

    /// Inserts a string snippet at the current cursor position.
    pub fn insert_snippet(&mut self, snippet: &str) {
        self.save_snapshot();
        editing::insert_snippet(self, snippet)
    }

    /// Inserts a note with the current octave and duration.
    pub fn insert_note(&mut self, note_char: char) {
        self.save_snapshot();
        editing::insert_note(self, note_char)
    }

    /// Inserts a rest with the current duration.
    pub fn insert_rest(&mut self) {
        self.save_snapshot();
        editing::insert_rest(self)
    }

    /// Transposes the note at the cursor by the given delta.
    pub fn transpose_note(&mut self, delta: i32) {
        self.save_snapshot();
        editing::transpose_note(self, delta)
    }

    /// Deletes the note at the cursor.
    pub fn delete_note(&mut self) {
        self.save_snapshot();
        editing::delete_note(self)
    }

    /// Adds a dot to the current duration.
    pub fn add_dot_to_duration(&mut self) {
        editing::add_dot_to_duration(self)
    }

    /// Removes a dot from the current duration.
    pub fn remove_dot_from_duration(&mut self) {
        editing::remove_dot_from_duration(self)
    }

    /// Formats the entire file.
    pub fn format_file(&mut self) {
        self.save_snapshot();
        formatting::format_buffer(&mut self.text);

        // Clamp cursor to valid range
        let len_chars = self.text.len_chars();
        if self.cursor_pos > len_chars {
            self.cursor_pos = len_chars;
        }

        self.message = "Formatted file".to_string();
    }

    /// Inserts a tab character (spaces).
    pub fn insert_tab(&mut self) {
        self.save_snapshot();
        self.insert_snippet(TAB_STRING);
    }

    /// Saves the current state to the undo stack.
    pub fn save_snapshot(&mut self) {
        self.undo_stack.push((self.text.clone(), self.cursor_pos));
        self.redo_stack.clear();
    }

    /// Undoes the last action.
    pub fn undo(&mut self) {
        if let Some((text, cursor_pos)) = self.undo_stack.pop() {
            self.redo_stack.push((self.text.clone(), self.cursor_pos));
            self.text = text;
            self.cursor_pos = cursor_pos;
            self.message = "Undo".to_string();
        } else {
            self.message = "Already at oldest change".to_string();
        }
    }

    /// Redoes the last undone action.
    pub fn redo(&mut self) {
        if let Some((text, cursor_pos)) = self.redo_stack.pop() {
            self.undo_stack.push((self.text.clone(), self.cursor_pos));
            self.text = text;
            self.cursor_pos = cursor_pos;
            self.message = "Redo".to_string();
        } else {
            self.message = "Already at newest change".to_string();
        }
    }

    /// Deletes the current line.
    pub fn delete_line(&mut self) {
        self.save_snapshot();
        editing::delete_line(self);
    }

    /// Inserts a checkpoint line.
    pub fn insert_checkpoint_line(&mut self, length: usize) {
        self.save_snapshot();
        editing::insert_checkpoint_line(self, length);
    }

    /// Inserts a voice command.
    pub fn insert_voice_command(&mut self) {
        self.save_snapshot();
        editing::insert_voice_command(self);
    }

    /// Appends a reset character and starts a new line.
    pub fn append_reset_and_newline(&mut self) {
        self.save_snapshot();
        editing::append_reset_and_newline(self);
    }

    /// Increments the current octave, clamping at HIGHEST_OCTAVE.
    pub fn increment_octave(&mut self) {
        if self.current_octave < HIGHEST_OCTAVE {
            self.current_octave += 1;
            self.message = format!("Octave: {}", self.current_octave);
        }
    }

    /// Decrements the current octave, clamping at LOWEST_OCTAVE.
    pub fn decrement_octave(&mut self) {
        if self.current_octave > LOWEST_OCTAVE {
            self.current_octave -= 1;
            self.message = format!("Octave: {}", self.current_octave);
        }
    }

    /// Sets the current duration.
    pub fn set_duration(&mut self, duration: &str) {
        self.current_duration = duration.to_string();
        self.message = format!("Duration: {}", duration);
    }
}

impl Drop for EditorState {
    fn drop(&mut self) {
        if let Some(mut child) = self.playback_process.take() {
            let _ = child.kill();
        }
    }
}
