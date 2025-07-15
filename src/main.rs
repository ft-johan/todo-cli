use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        event::{self, Event, KeyEvent},
        style::Color,
    },
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}
enum FormAction {
    None,
    Submit,
    Escape,
}
fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_new = false;

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Finish applications"),
    });
    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Finish applications"),
    });
    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Finish applications"),
    });
    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Finish applications"),
    });

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;

        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                match handle_add_new(key, app_state) {
                    FormAction::Submit => {
                        app_state.is_add_new = false;
                        app_state.items.push(TodoItem {
                            is_done: false,
                            description: app_state.input_value.clone(),
                        });
                    }
                    FormAction::None => {},

                    FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_value.clear();
                    }
                }
            } else {
                if handle_key(key, app_state) {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        event::KeyCode::Enter => {
            return FormAction::Submit;
        }
        event::KeyCode::Esc => {
            FormAction::Escape;
        }
        _ => {}
    }
    FormAction::None
}
fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Char(char) => match char {
            'A' => {
                app_state.is_add_new = true;
            }
            'D' => {
                if let Some(index) = app_state.list_state.selected() {
                    app_state.items.remove(index);
                }
            }
            'k' => {
                app_state.list_state.select_previous();
            }
            'j' => {
                app_state.list_state.select_next();
            }
            _ => {}
        },

        _ => {}
    }
    false
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    if app_state.is_add_new {
        Paragraph::new(&*app_state.input_value.as_str())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded),
            )
            .render(frame.area(), frame.buffer_mut());
    } else {
        let [border_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(frame.area());
        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        Block::bordered()
            .border_type(BorderType::Rounded)
            .fg(Color::Yellow)
            .render(border_area, frame.buffer_mut());
        let list = List::new(
            app_state
                .items
                .iter()
                .map(|x| ListItem::from(x.description.as_str())),
        )
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::Black.into()));
        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }
}
