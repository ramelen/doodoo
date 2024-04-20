use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        disable_raw_mode().unwrap();
        crossterm::execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        original_hook(panic);
    }));

    let mut todos = vec![
        "Help menu",
        "Adding items",
        "Editing items",
        "Removing items",
        "Completing items",
    ];

    let mut list_state = ListState::default();
    list_state.select(Some(0));

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
            frame.render_stateful_widget(list, area, &mut list_state);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('k') => {
                            select_previous(&mut list_state);
                        }
                        KeyCode::Char('j') => {
                            select_next(&mut list_state, &todos);
                        }
                        KeyCode::Char('d') if list_state.selected().is_none() => {}
                        KeyCode::Char('d') => {
                            todos.remove(list_state.selected().unwrap_or(todos.len() + 1));
                            if todos.len() != 0 {
                                select_previous(&mut list_state);
                            } else {
                                list_state.select(None);
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

fn select_next(list_state: &mut ListState, todos: &Vec<&str>) {
    if let Some(selection) = list_state.selected() {
        list_state.select(Some(if selection >= todos.len() - 1 {
            todos.len() - 1
        } else {
            selection + 1
        }))
    }
}

fn select_previous(list_stace: &mut ListState) {
    if let Some(selection) = list_stace.selected() {
        list_stace.select(Some(if selection == 0 { 0 } else { selection - 1 }))
    } else {
        return;
    }
}
