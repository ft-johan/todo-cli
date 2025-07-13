use color_eyre::eyre::{Ok ,Result};
use ratatui::{crossterm::event::{self, Event}, layout::{Constraint, Layout}, widgets::{Paragraph, Widget}, DefaultTerminal, Frame};


#[derive(Debug , Default)]
struct AppState{
    items:Vec<TodoItem>
}}

#[derive(Debug, Default)]
struct TodoItem{
    is_done:bool,
    description:String,
}
fn main() -> Result<()>
  {
    let mut state =AppState::default();
    println!("Hello, world!");
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal,&mut state) ;
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) ->Result<()>
 {
    loop {
terminal.draw(|f|render(f,app_state))?;

        if let Event ::Key(key)= event::read()? {
            match key.code {
                event::KeyCode::Esc =>{
                    break;
                }
                _ => {}
            }
            
        }
    }
    Ok(())
}


fn render(frame:&mut Frame,  app_state:&AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill((1))]).margin(1).areas(frame.area()) ;
    Paragraph::new("Hello from application").render(frame.area(), frame.buffer_mut());
}