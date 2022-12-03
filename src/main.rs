mod board;
mod constants;
mod tiles;
mod app;

use log::debug;
use log::error;
use log::info;
use log::warn;

use tiles::{TileBag};
use board::Board;
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Borders, BorderType, Block},
    layout::{Layout,Constraint,Direction},
    Frame,
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, char};
use app::{App, TuiState};

fn main() -> Result<(),io::Error> {

    env_logger::init();
    info!("krible version 0.0.1");
    info!("author: Nikhil Nayyar");
    info!("last updated: Dec 2, 2022");

    let mut terminal = preset_terminal()?;

    let bag = TileBag::new();
    let board = Board::new(15,15);
    let app = App::new(board, bag, 1);

    let res = run_app(&mut terminal,app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()>{

    loop{

        terminal.draw(|f|{ui(f, &mut app);})?;

        if let Event::Key(key) = event::read()?{
            info!("logged key {}", format!("{key:#?}"));
            match key.code{
                KeyCode::Char('q') => {return Ok(())}
                _ => {}
            }

            let prev_state = &app.state.clone();
            info!("previous state {}", format!("{prev_state:#?}"));
            match &app.state{
                TuiState::Board => {
                    match key.code{
                        KeyCode::Char('t') => {app.state=TuiState::Transition}
                        _ => {}
                    }
                }
                TuiState:: Chat => {
                    match key.code{
                        KeyCode::Char('t') => {app.state=TuiState::Transition}
                        _ => {}
                    }
                }
                TuiState::Rack => {
                    match key.code{
                        KeyCode::Char('t') => {app.state=TuiState::Transition}
                        _ => {}
                    }                    
                }
                TuiState::Transition => {
                    match key.code{
                        KeyCode::Char('b') => {app.selected=TuiState::Board;}
                        KeyCode::Char('c') => {app.selected=TuiState::Chat;}
                        KeyCode::Char('r') => {app.selected=TuiState::Rack;}
                        KeyCode::Char('t') => {app.state = app.selected;}
                        _ => {}
                    }
                }
            }

            let new_state = &app.state.clone();
            info!("new state {}", format!("{new_state:#?}"));
        }

    }

}

fn ui <B: Backend>(f: &mut Frame<B>, app: &mut App){

    let terminal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ].as_ref()
        )
        .split(f.size());

    let gameboard_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(75),
                Constraint::Percentage(15),
            ]
        )
        .split(terminal_layout[0]);

    let score = Block::default().title("score").borders(Borders::ALL);
    f.render_widget(score, gameboard_layout[0]);

    let border_selected_type = if matches!(app.state,TuiState::Transition) {BorderType::Double} else {BorderType::Thick};
    let border_unselected_type = BorderType::Plain;

    let board = Block::default().title("(b)oard").borders(Borders::ALL).border_type(if matches!(&app.selected, TuiState::Board) {border_selected_type} else{border_unselected_type});
    /*
    let rows = app.board.board.iter().map(|row| {
        let cells = row.iter().map(|c|{
                    //println!("Working on ({},{})", c.row, c.col);
                    if c.tile.is_none() {
                        Cell::from(Span::from("N/A"))
                    } 
                    else{
                        let c_str = c.tile.as_ref().unwrap().get_letter().to_string();
                        Cell::from(Span::from(c_str))
                    }
                }
            );
        Row::new(cells).height(1).bottom_margin(1)
    });
    let board_table = Table::new(rows).widths(&[Constraint::Percentage(100)]).block(Block::default().title("board").borders(Borders::ALL));
    */
    f.render_widget(board, gameboard_layout[1]);

    /*
    let player_tiles = app.racks[0].get_tiles().iter().map(|t| {Spans::from(t.get_tile())}).collect();
    let rack_tile_style = Style::default().bg(tui::style::Color::LightYellow).fg(tui::style::Color::Black).add_modifier(Modifier::BOLD);
    let tile_rack = Tabs::new(player_tiles).block(Block::default().title("rack").border_type(BorderType::Double)).style(rack_tile_style).divider(" ");
    */
    let tile_rack = Block::default().title("(r)ack").borders(Borders::ALL).border_type(if matches!(&app.selected, TuiState::Rack) {border_selected_type} else{border_unselected_type});
    f.render_widget(tile_rack, gameboard_layout[2]);

    let block2 = Block::default().title("(c)hat").borders(Borders::ALL).border_type(if matches!(&app.selected, TuiState::Chat) {border_selected_type} else{border_unselected_type});
    f.render_widget(block2, terminal_layout[1]);

}