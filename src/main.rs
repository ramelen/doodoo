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

    let todos = vec![
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
                            let selected = state.selected().unwrap_or_default();
                            state.select(Some(if selected == 0 { 0 } else { selected - 1 }))
                        }
                        KeyCode::Char('j') => {
                            let selected = state.selected().unwrap_or_default();
                            state.select(Some(if selected >= todos.len() - 1 {
                                todos.len() - 1
                            } else {
                                selected + 1
                            }))
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
