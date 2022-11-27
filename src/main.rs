mod board;
mod constants;
mod tiles;
mod app;

use tiles::{TileBag, Tile};
use board::Board;
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders, Cell, Row, Table, Paragraph, Tabs, BorderType},
    layout::{Layout,Constraint,Direction, Alignment},
    Frame,
    Terminal, text::{Span, Spans}, style::{Style, Modifier}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration, cell};
use app::App;

fn main() -> Result<(),io::Error> {

    let mut terminal = preset_terminal()?;

    let bag = TileBag::new();
    let board = Board::new(15,15);
    let app = App::new(board, bag, 1);

    let res = run_app(&mut terminal,app);

    postset_terminal(terminal)?;

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
            match key.code{
                KeyCode::Char('q') => return Ok(()),
                _ => (),
            }
        }

    }

}

fn ui <B: Backend>(f: &mut Frame<B>, app: &mut App){

    let main_layout = Layout::default()
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
        .split(main_layout[0]);

    let score = Block::default().title("score").borders(Borders::ALL);
    f.render_widget(score, gameboard_layout[0]);

    let board = Block::default().title("board").borders(Borders::ALL);
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

    let player_tiles = app.racks[0].get_tiles().iter().map(|t| {Spans::from(t.get_tile())}).collect();
    let rack_tile_style = Style::default().bg(tui::style::Color::LightYellow).fg(tui::style::Color::Black).add_modifier(Modifier::BOLD);
    let tile_rack = Tabs::new(player_tiles).block(Block::default().title("rack").border_type(BorderType::Double)).style(rack_tile_style).divider(" ");
    f.render_widget(tile_rack, gameboard_layout[2]);

    let block2 = Block::default().title("game_log").borders(Borders::ALL);
    f.render_widget(block2, main_layout[1]);

}