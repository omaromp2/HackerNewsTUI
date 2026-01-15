mod api;
mod app;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::stdout;
use std::panic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    panic::set_hook(Box::new(|info| {
        disable_raw_mode().ok();
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture).ok();
        println!("{}", info);
    }));

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        app.load_stories().await;
    });

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.next_story();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.prev_story();
                    }
                    KeyCode::Char(' ') => {
                        app.next_story_type();
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(async {
                            app.load_stories().await;
                        });
                    }
                    KeyCode::Char('d') => {
                        app.toggle_details();
                    }
                    KeyCode::Char('o') => {
                        if let Some(url) = app.selected_story_url() {
                            let _ = open::that(url);
                        }
                    }
                    KeyCode::Char('m') => {
                        if app.can_load_more() {
                            let rt = tokio::runtime::Runtime::new().unwrap();
                            rt.block_on(async {
                                app.load_more_stories().await;
                            });
                        }
                    }
                    KeyCode::Char('r') => {
                        if matches!(app.state, app::AppState::Error(_)) {
                            let rt = tokio::runtime::Runtime::new().unwrap();
                            rt.block_on(async {
                                app.load_stories().await;
                            });
                        }
                    }
                    KeyCode::PageDown => {
                        app.page_down();
                    }
                    KeyCode::PageUp => {
                        app.page_up();
                    }
                    KeyCode::Home => {
                        app.selected_index = 0;
                        app.scroll_offset = 0;
                    }
                    KeyCode::End => {
                        if !app.stories.is_empty() {
                            app.selected_index = app.stories.len() - 1;
                            app.update_scroll();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
