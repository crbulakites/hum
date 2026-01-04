use super::{EditorState, utils};
use std::{
    fs::File,
    io::{self, BufWriter},
};

const FULL_PLAYBACK_FILENAME: &str = "hum_full_playback.wav";
const SECTION_PREVIEW_FILENAME: &str = "hum_section_preview.wav";
const NOTE_PREVIEW_FILENAME: &str = "hum_note_preview.wav";
const DEFAULT_PREVIEW_HEADER: &str = "[ 120_bpm ][ 4/4 ]";

const MSG_PLAYBACK_STOPPED: &str = "Playback stopped.";
const MSG_NO_FILENAME: &str = "No filename specified";
const MSG_PLAYING_FILE: &str = "Playing file...";
const MSG_PLAYING_SECTION: &str = "Playing section...";
const MSG_PLAYING_NOTE: &str = "Playing note...";

/// Stops any active playback process.
pub fn stop_playback(state: &mut EditorState) {
    if let Some(mut child) = state.playback_process.take() {
        let _ = child.kill();
        state.message = MSG_PLAYBACK_STOPPED.to_string();
    }
}

/// Saves the current buffer to the file.
pub fn save_file(state: &mut EditorState) -> io::Result<()> {
    if let Some(filename) = &state.filename {
        let file = File::create(filename)?;
        state.text.write_to(BufWriter::new(file))?;
        state.message = format!("Saved to {}", filename);
    } else {
        state.message = MSG_NO_FILENAME.to_string();
    }
    Ok(())
}

/// Plays the entire file content without saving to disk.
///
/// Generates a temporary WAV file from the current editor buffer.
pub fn play_file(state: &mut EditorState) {
    let score_contents = state.text.to_string();
    play_score_content(
        state,
        &score_contents,
        FULL_PLAYBACK_FILENAME,
        MSG_PLAYING_FILE,
    );
}

/// Plays the section starting from the last checkpoint before the cursor.
pub fn play_from_cursor(state: &mut EditorState) {
    let checkpoint_line = find_last_checkpoint_line(state);
    let score_contents = if let Some(line_idx) = checkpoint_line {
        let header = extract_context_header(state, line_idx);
        let section = extract_section_content(state, line_idx);
        format!("{}{}", header, section)
    } else {
        state.text.to_string()
    };

    play_score_content(
        state,
        &score_contents,
        SECTION_PREVIEW_FILENAME,
        MSG_PLAYING_SECTION,
    );
}

/// Plays a single note for preview.
pub fn play_note(state: &mut EditorState, note: &str) {
    // Find voice
    let voice_name = super::utils::get_voice_at_cursor(state);

    // Construct a minimal score
    let score_contents = format!(
        "{} {}{} {} {} {}",
        DEFAULT_PREVIEW_HEADER,
        utils::VOICE_PREFIX,
        voice_name,
        utils::MEASURE_CHAR,
        note,
        utils::RESET_CHAR
    );

    play_score_content(
        state,
        &score_contents,
        NOTE_PREVIEW_FILENAME,
        MSG_PLAYING_NOTE,
    );
}

/// Common helper to convert score content to a temp WAV and play it.
fn play_score_content(
    state: &mut EditorState,
    score_contents: &str,
    temp_filename: &str,
    message: &str,
) {
    let temp_dir = std::env::temp_dir();
    let wav_path = temp_dir.join(temp_filename);
    let wav_filename = wav_path.to_string_lossy().to_string();

    match crate::convert_to_wav(score_contents, &wav_filename) {
        Ok(_) => {
            stop_playback(state);
            state.message = message.to_string();
            match std::process::Command::new(&state.playback_command)
                .arg(&wav_filename)
                .spawn()
            {
                Ok(child) => {
                    state.playback_process = Some(child);
                }
                Err(e) => state.message = format!("Error playing: {}", e),
            }
        }
        Err(e) => state.message = format!("Error converting: {}", e),
    }
}

/// Finds the index of the last checkpoint line before the cursor.
fn find_last_checkpoint_line(state: &EditorState) -> Option<usize> {
    let line_idx = state.text.char_to_line(state.cursor_pos);
    for i in (0..=line_idx).rev() {
        let line = state.text.line(i).to_string();
        if line.trim().starts_with(utils::CHECKPOINT_CHAR) {
            return Some(i);
        }
    }
    None
}

/// Extracts context (BPM, Time, Voice) from the start of the file up to `end_line`.
///
/// Searches backwards from `end_line` to find the most recent definitions.
fn extract_context_header(state: &EditorState, end_line: usize) -> String {
    let mut context_lines = Vec::new();
    let mut found_bpm = false;
    let mut found_time = false;
    let mut found_voice = false;

    for i in (0..end_line).rev() {
        if found_bpm && found_time && found_voice {
            break;
        }

        let line = state.text.line(i).to_string();
        let trimmed = line.trim();
        let mut useful = false;

        if !found_bpm && utils::is_tempo_line(trimmed) {
            found_bpm = true;
            useful = true;
        }
        if !found_time && utils::is_time_signature_line(trimmed) {
            found_time = true;
            useful = true;
        }
        if !found_voice && utils::is_voice_line(trimmed) {
            found_voice = true;
            useful = true;
        }

        if useful {
            context_lines.push(line);
        }
    }

    context_lines.reverse();
    context_lines.join("")
}

/// Extracts the content of the section starting at `start_line`.
fn extract_section_content(state: &EditorState, start_line: usize) -> String {
    let start_char = state.text.line_to_char(start_line);
    state.text.slice(start_char..).to_string()
}
