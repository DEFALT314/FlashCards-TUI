mod app;
mod model;
mod ui;

use crate::app::{App, CurrentScreen, CurrentlyEditing};
use crate::model::Collection;
use anyhow::Result;
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::crossterm::{event, execute};
use std::io;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let collection = Collection::new()?;
    let mut app = App::new(collection);
    run_app(&mut terminal, &mut app)?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;

    app.collection.save()?;

    Ok(())
}
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    while app.current_screen != CurrentScreen::Exiting {
        terminal.draw(|f| ui::draw(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    KeyCode::Enter => app.toggle_study(),
                    KeyCode::Char('a') => {
                        app.current_screen = CurrentScreen::AddingDeck;
                        app.toggle_editing();
                    }
                    _ => {}
                },
                CurrentScreen::Studying => match key.code {
                    KeyCode::Esc => app.stop_studying(),
                    KeyCode::Char(' ') => app.toggle_answer(),
                    KeyCode::Char('1') => app.rate_current_card(1),
                    KeyCode::Char('2') => app.rate_current_card(2),
                    KeyCode::Char('3') => app.rate_current_card(3),
                    KeyCode::Char('4') => app.rate_current_card(4),
                    KeyCode::Char('5') => app.rate_current_card(5),
                    _ => {}
                },

                CurrentScreen::AddingDeck if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Name => {
                                    app.currently_editing = Some(CurrentlyEditing::Path);
                                }
                                CurrentlyEditing::Path => {
                                    app.add_new_deck();
                                    app.current_screen = CurrentScreen::Main;
                                    app.currently_editing = None;

                                    app.name_input.clear();
                                    app.path_input.clear();
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Name => {
                                    app.name_input.pop();
                                }
                                CurrentlyEditing::Path => {
                                    app.path_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Tab => {
                        app.toggle_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Name => {
                                    app.name_input.push(value);
                                }
                                CurrentlyEditing::Path => {
                                    app.path_input.push(value);
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}
