use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use ratatui::{prelude::*, widgets::*};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut todos = vec![
        "Help menu",
        "Adding items",
        "Editing items",
        "Removing items",
        "Completing items",
    ];

    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let list = List::new(todos.clone())
                .block(
                    Block::default()
                        .title("<Press q to quit>")
                        .borders(Borders::NONE),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Cyan));
            frame.render_stateful_widget(list, area, &mut state);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('k') => {
                            select_previous(&mut state);
                        }
                        KeyCode::Char('j') => {
                            select_next(&mut state, &todos);
                        }
                        KeyCode::Char('d') if state.selected().is_none() => {}
                        KeyCode::Char('d') => {
                            todos.remove(state.selected().unwrap_or(todos.len() + 1));
                            if todos.len() != 0 {
                                select_previous(&mut state);
                            } else {
                                state.select(None);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn select_next(state: &mut ListState, todos: &Vec<&str>) {
    if let Some(selection) = state.selected() {
        state.select(Some(if selection >= todos.len() - 1 {
            todos.len() - 1
        } else {
            selection + 1
        }))
    }
}

fn select_previous(state: &mut ListState) {
    if let Some(selection) = state.selected() {
        state.select(Some(if selection == 0 { 0 } else { selection - 1 }))
    } else {
        return;
    }
}
