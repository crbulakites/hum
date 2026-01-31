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

use super::editor_state::utils::{
    COMMENT_CHAR, HELP_TEXT, MEASURE_CHAR, NOTE_START_CHAR, is_checkpoint_line, is_comment_line,
};
use super::editor_state::{EditorState, Mode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

const COMMENT_COLOR: Color = Color::Green;
const CHECKPOINT_COLOR: Color = Color::DarkGray;
const TEMPO_TIME_COLOR: Color = Color::Cyan;
const NOTE_COLOR: Color = Color::LightBlue;
const MEASURE_COLOR: Color = Color::Magenta;
const VOICE_COLOR: Color = Color::Yellow;
const STATUS_BAR_BG_COLOR: Color = Color::Blue;
const STATUS_BAR_FG_COLOR: Color = Color::White;

pub const STATUS_BAR_HEIGHT: u16 = 3;
pub const UI_MARGIN: u16 = 1;
pub const UI_BORDER_WIDTH: u16 = 1;

/// Highlights a single line of text based on Hum syntax.
fn highlight_line(raw_line: &str) -> Line<'static> {
    let line_content = raw_line.trim_end_matches(['\n', '\r']);

    if is_comment_line(line_content) {
        return Line::from(Span::styled(
            line_content.to_string(),
            Style::default().fg(COMMENT_COLOR),
        ));
    }

    if is_checkpoint_line(line_content) {
        return Line::from(Span::styled(
            line_content.to_string(),
            Style::default().fg(CHECKPOINT_COLOR),
        ));
    }

    let mut spans = Vec::new();
    let mut current_text = String::new();
    let mut current_style = Style::default();

    let chars: Vec<char> = line_content.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        match c {
            '[' => {
                if !current_text.is_empty() {
                    spans.push(Span::styled(current_text.clone(), current_style));
                    current_text.clear();
                }
                current_style = Style::default().fg(TEMPO_TIME_COLOR);
                current_text.push(c);
            }

            ']' => {
                current_text.push(c);
                spans.push(Span::styled(current_text.clone(), current_style));
                current_text.clear();
                current_style = Style::default();
            }

            c if c == NOTE_START_CHAR => {
                if !current_text.is_empty() {
                    spans.push(Span::styled(current_text.clone(), current_style));
                    current_text.clear();
                }
                current_style = Style::default().fg(NOTE_COLOR);
                current_text.push(c);
            }

            ')' => {
                current_text.push(c);
                spans.push(Span::styled(current_text.clone(), current_style));
                current_text.clear();
                current_style = Style::default();
            }

            c if c == MEASURE_CHAR => {
                if !current_text.is_empty() {
                    spans.push(Span::styled(current_text.clone(), current_style));
                    current_text.clear();
                }
                spans.push(Span::styled(
                    MEASURE_CHAR.to_string(),
                    Style::default().fg(MEASURE_COLOR),
                ));
            }

            '%' => {
                if !current_text.is_empty() {
                    spans.push(Span::styled(current_text.clone(), current_style));
                    current_text.clear();
                }
                let rest: String = chars[i..].iter().collect();
                spans.push(Span::styled(rest, Style::default().fg(VOICE_COLOR)));
                return Line::from(spans);
            }

            c if c == COMMENT_CHAR => {
                if !current_text.is_empty() {
                    spans.push(Span::styled(current_text.clone(), current_style));
                    current_text.clear();
                }
                let rest: String = chars[i..].iter().collect();
                spans.push(Span::styled(rest, Style::default().fg(COMMENT_COLOR)));
                return Line::from(spans);
            }

            _ => {
                current_text.push(c);
            }
        }

        i += 1;
    }

    if !current_text.is_empty() {
        spans.push(Span::styled(current_text, current_style));
    }

    Line::from(spans)
}

/// Renders the editor UI.
pub fn ui(f: &mut Frame, state: &EditorState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(UI_MARGIN)
        .constraints([Constraint::Min(1), Constraint::Length(STATUS_BAR_HEIGHT)].as_ref())
        .split(f.area());

    render_editor_content(f, state, chunks[0]);
    update_cursor_position(f, state, chunks[0]);
    render_status_bar(f, state, chunks[1]);

    if state.show_help {
        render_help_popup(f, state);
    }
}

/// Renders the main editor content area with syntax highlighting.
fn render_editor_content(f: &mut Frame, state: &EditorState, area: ratatui::layout::Rect) {
    let block = Block::default().title("Hum Editor").borders(Borders::ALL);

    let height = area.height as usize;
    let content_height = height.saturating_sub((UI_BORDER_WIDTH * 2) as usize);

    let lines: Vec<Line> = state
        .text
        .lines_at(state.scroll_offset)
        .take(content_height)
        .map(|line| highlight_line(&line.to_string()))
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((0, state.col_scroll_offset as u16));

    f.render_widget(paragraph, area);
}

/// Calculates and sets the terminal cursor position based on editor state.
fn update_cursor_position(f: &mut Frame, state: &EditorState, area: ratatui::layout::Rect) {
    let line_idx = state.text.char_to_line(state.cursor_pos);
    let col_idx = state.cursor_pos - state.text.line_to_char(line_idx);

    if line_idx >= state.scroll_offset && col_idx >= state.col_scroll_offset {
        let visual_line = line_idx - state.scroll_offset;
        let visual_col = col_idx - state.col_scroll_offset;

        if visual_line < area.height as usize && visual_col < area.width as usize {
            f.set_cursor_position((
                area.x + UI_BORDER_WIDTH + visual_col as u16,
                area.y + UI_BORDER_WIDTH + visual_line as u16,
            ));
        }
    }
}

/// Renders the status bar at the bottom of the screen.
fn render_status_bar(f: &mut Frame, state: &EditorState, area: ratatui::layout::Rect) {
    let mode_text = match state.mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
    };

    let status_bar_text = format!(
        "Mode: {} | {} | Oct: {} | Dur: {} | {} | ?: Help",
        mode_text,
        state.filename.as_deref().unwrap_or("[No Name]"),
        state.current_octave,
        state.current_duration,
        state.message
    );

    let status_bar = Paragraph::new(status_bar_text)
        .style(
            Style::default()
                .bg(STATUS_BAR_BG_COLOR)
                .fg(STATUS_BAR_FG_COLOR),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status_bar, area);
}

fn render_help_popup(f: &mut Frame, state: &EditorState) {
    let area = centered_rect(60, 80, f.area());

    let help_text: Vec<Line> = HELP_TEXT.iter().map(|&s| Line::from(s)).collect();

    let block = Block::default()
        .title("Help (Arrows/j/k to scroll, Esc/q to close)")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .scroll((state.help_scroll_offset, 0));

    f.render_widget(Clear, area); // Clear the background
    f.render_widget(paragraph, area);
}

/// Helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
