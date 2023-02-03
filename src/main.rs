mod board;
mod constants;
mod tiles;
mod app;
mod ui;
mod utils;

use log::info;

use tiles::{TileBag};
use board::Board;

use tui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ui::render_ui;

use std::{io};
use app::{App, TuiState};

use crate::app::InputType;

fn main() -> Result<(),io::Error> {

    env_logger::init();
    info!("krible version 0.0.1");
    info!("author: Nikhil Nayyar");
    info!("last updated: Feb 1, 2023");

    let file = "/home/nayyar/Development/krible/src/resouces/board.json";

    let mut terminal = preset_terminal()?;

    let mut app = App::new(
        Board::new(15,15, file), 
        TileBag::new(), 
        1
    );

    let _ = run_app(&mut terminal,&mut app);

    info!("returning terminal to original state");
    postset_terminal(terminal)?;
    info!("exiting krible");

    Ok(())
}

fn postset_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn preset_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, std::io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>{

    loop{

        terminal.draw(|f|{render_ui(f, app);})?;

        if let Event::Key(key) = event::read()?{
            
            let input_type = App::process_input(&key);

            let old_state = &app.state.clone();
            match input_type{
                InputType::Action=>{
                    match &app.state{
                        TuiState::Game => {
                            app.handle_game(&key)
                        }
                        TuiState::Chat => {
                            {}
                        }
                    }
                }
                InputType::Transition=>{
                    match key.code{
                        KeyCode::Char('1') => {app.state=TuiState::Game;}
                        KeyCode::Char('2') => {app.state=TuiState::Chat;}
                        _ => {}
                    }
                }
                InputType::Quit=>{
                    return Ok(())
                }
            }
            let new_state = &app.state.clone();

            if new_state == old_state{
                info!("new state {}", format!("{new_state:#?}"));
            }

        }

    }

}



