use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use ropey::Rope;
use std::{
    error::Error,
    fs::File,
    io::{self, BufReader},
};

pub mod editor_state;
pub mod input;
pub mod view;

use editor_state::{EditorState, Mode};
use input::{handle_insert_input, handle_normal_input};
use view::ui;

const UI_HEIGHT_DEDUCTION: u16 =
    (view::UI_MARGIN * 2) + view::STATUS_BAR_HEIGHT + (view::UI_BORDER_WIDTH * 2);
const UI_WIDTH_DEDUCTION: u16 = (view::UI_MARGIN * 2) + (view::UI_BORDER_WIDTH * 2);

/// Sets up the terminal in raw mode and returns a Terminal instance.
fn setup_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Creates the initial application state, loading a file if provided.
fn create_editor_state(filename: Option<String>) -> Result<EditorState, Box<dyn Error>> {
    let mut state = EditorState::default();

    if let Some(name) = filename {
        state.filename = Some(name.clone());

        match File::open(&name) {
            Ok(file) => {
                state.text = Rope::from_reader(BufReader::new(file))?;
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                // File doesn't exist, start with empty state (already default)
                state.message = format!("New file: {}", name);
            }
            Err(e) => {
                // Other error (permission, etc.)
                return Err(Box::new(e));
            }
        }
    }

    Ok(state)
}

/// Exits the editor, restoring the terminal state.
fn exit_editor(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}

/// Handles the result of the editor application.
fn handle_editor_result(res: io::Result<()>) {
    if let Err(err) = res {
        println!("{:?}", err)
    }
}

/// Runs the Hum text editor.
pub fn run_editor(filename: Option<String>) -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    let state = create_editor_state(filename)?;
    let res = run_app(&mut terminal, state);

    exit_editor(&mut terminal)?;
    handle_editor_result(res);

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut state: EditorState) -> io::Result<()> {
    loop {
        let size = terminal
            .size()
            .map_err(|e| io::Error::other(e.to_string()))?;

        let height = size.height.saturating_sub(UI_HEIGHT_DEDUCTION) as usize;
        let width = size.width.saturating_sub(UI_WIDTH_DEDUCTION) as usize;
        state.scroll_into_view(width, height);

        terminal
            .draw(|f| ui(f, &state))
            .map_err(|e| io::Error::other(e.to_string()))?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match state.mode {
                Mode::Normal => {
                    if handle_normal_input(&mut state, key) {
                        return Ok(());
                    }
                }
                Mode::Insert => handle_insert_input(&mut state, key),
            }
        }
    }
}
