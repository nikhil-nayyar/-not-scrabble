
use log::debug;
use tui::{layout::{Rect, Layout, Constraint, Direction}, backend::Backend, Frame, style::{Color, Style}, widgets::{Borders, BorderType, Block}, text::Span};
use crate::{utils::ui::closest_multiple, app::TuiState};

use board::CellModifier;
use tui::widgets::Paragraph;

use crate::{board::{Cell, self}, app::App};


pub fn render_ui <B: Backend>(f: &mut Frame<B>, app: &mut App){

    // OPTIONS
    let cell_width = 4;
    let cell_height = 2;

    // STYLES
    let selected = Style::default().fg(Color::Rgb(250, 220, 70));
    let _temp = Style::default().fg(Color::Rgb(255, 255, 255));
    let unselected = Style::default().fg(Color::Rgb(167, 167, 167));

    // Terminal 
    let terminal_rect = f.size();
    debug!("Terminal Rect: {} {}", terminal_rect.width, terminal_rect.height);

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .title("(Not) Scrabble")
        .border_type(BorderType::Rounded);
    f.render_widget(outer_block, terminal_rect);

    let terminal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ].as_ref()
        )
        .split(terminal_rect);

    // Chat Placholder
    let text_style = if matches!(app.state, TuiState::Chat) {selected} else {unselected};
    let text = Span::styled("chat", text_style);
    let chat_rect = terminal_layout[1];
    let chat_block = Block::default()
        .borders(Borders::LEFT)
        .title(text)
        .border_type(BorderType::Rounded);
    f.render_widget(chat_block, chat_rect);

    // Gameboard Layout
    let text_style = if matches!(app.state, TuiState::Game) {selected} else {unselected};
    let text = Span::styled("game", text_style);
    let game_rect = terminal_layout[0];
    let game_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
        )
        .split(game_rect);
    let game_block = Block::default()
        .borders(Borders::NONE)
        .title(text)
        .border_type(BorderType::Rounded);
    f.render_widget(game_block, game_layout[1]);

    // Score Placholder 
    let score_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Rounded);
    f.render_widget(score_block, game_layout[0]);
    
    // Implemented Render Engine
    render_gameboard(game_layout[1], f, app, cell_width, cell_height);
    render_tilerack(game_layout[2], f, app, cell_width, cell_height);

}


pub fn render_gameboard<B:Backend>(outer_rect: Rect, f: &mut Frame<B>,app: &mut App, cell_width: u16, cell_height: u16){
    debug!("gameboard outer dimensions: ({},{})", outer_rect.width, outer_rect.height);

    let selected = Style::default()
        .fg(Color::Black)
        .bg(Color::White);

    let unselected = Style::default();

    let padding_rect = Rect::new(
        0,
        0,
        (outer_rect.width - (15 * cell_width)) / 2,
        (outer_rect.height - (15 * cell_height)) / 2,
    );
    debug!("gameboard padding dimensions: ({},{})", padding_rect.width, padding_rect.height);

    let inner_rect = Rect::new(
        0,
        0,
        closest_multiple(outer_rect.width - (2* padding_rect.width), 15),
        closest_multiple(outer_rect.height - (2*padding_rect.height), 15),
    );

    debug!("gameboard inner dimensions: ({},{})", inner_rect.width, inner_rect.height);

    let board_layout_vertical = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
        [
            Constraint::Max(padding_rect.height),
            Constraint::Max(inner_rect.height),
            Constraint::Max(padding_rect.height),
        ]
    )
    .vertical_margin(0)
    .horizontal_margin(0)
    .split(outer_rect);

    let board_layout_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Max(padding_rect.width),
                Constraint::Max(inner_rect.width),
                Constraint::Max(padding_rect.width),
            ]
        )
        .vertical_margin(0)
        .horizontal_margin(0)
        .split(board_layout_vertical[1]);

    let rows_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            vec![Constraint::Max(cell_height); 15]
        )
        .vertical_margin(0)
        .horizontal_margin(0)
        .split(board_layout_horizontal[1]);

    let mut board_layout: Vec<Vec<Rect>> = Vec::new();

    for row in 0..15{
        let cols_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                vec![Constraint::Max(cell_width); 15]
            )
            .vertical_margin(0)
            .horizontal_margin(0)
            .split(rows_layout[row]);
        board_layout.push(cols_layout);
    }

    let board = &app.board;

    for row in 0..15{
        for col in 0..15{
            debug!("generating tile ({},{}) of size ({},{})", row, col, board_layout[row][col].width,  board_layout[row][col].height);

            let style = if app.board.cursor.0 as usize == col && app.board.cursor.1 as usize == row {selected} else {unselected};

            // determine cell text
            let mut txt:String = "".into();
            let cell: &Cell = board.get_cell(row,col);
            if !cell.has_tile(){
                match cell.modifier {
                    CellModifier::Normal=>txt="".into(),
                    CellModifier::TripleWord=>txt="TW".into(),
                    CellModifier::DoubleWord=>txt="DW".into(),
                    CellModifier::TripleLetter=>txt="TL".into(),
                    CellModifier::DoubleLetter=>txt="DL".into(),
                }
            } else {
                txt = cell.get_tile_text();
            }

            // determine borders
            let border = Borders::LEFT | Borders::TOP;
            let border_style = if cell.has_tile() {Style::default().fg(Color::Yellow)} else {Style::default()};
            let border_type = if cell.has_tile() {BorderType::Double} else {BorderType::Plain};
            // if row == 0 {border |= Borders::TOP;}
            // if col == 0 {border |= Borders::LEFT;}

            let cell = Paragraph::new(txt.to_string())
                .block(Block::default()
                .borders(border)
                .border_type(border_type)
                .border_style(border_style)
                .style(style)
            );
            f.render_widget(cell, board_layout[row][col]);
        }
    }


}

pub fn render_tilerack<B:Backend>(outer_rect: Rect, f: &mut Frame<B>, app: &mut App, cell_width: u16, cell_height: u16){
    
    debug!("tilerack outer dimensions: ({},{})", outer_rect.width, outer_rect.height);

    let padding_rect = Rect::new(
        0,
        0,
        (outer_rect.width - (7 * cell_width)) / 2,
        (outer_rect.height - (cell_height)) / 2,
    );
    debug!("tilerack padding dimensions: ({},{})", padding_rect.width, padding_rect.height);

    let inner_rect = Rect::new(
        0,
        0,
        closest_multiple(outer_rect.width - (2* padding_rect.width), 7),
        outer_rect.height - (2*padding_rect.height),
    );
    debug!("tilerack inner dimensions: ({},{})", inner_rect.width, inner_rect.height);

    let tile_layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Max(padding_rect.height),
                Constraint::Max(inner_rect.height),
                Constraint::Max(padding_rect.height),
            ]
        )
        .vertical_margin(0)
        .horizontal_margin(0)
        .split(outer_rect);

    let tile_layout_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Max(padding_rect.width),
                Constraint::Max(inner_rect.width),
                Constraint::Max(padding_rect.width),
            ]
        )
        .vertical_margin(0)
        .horizontal_margin(0)
        .split(tile_layout_vertical[1]);

    let tiles_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Max(cell_width);7])
        .vertical_margin(0)
        .horizontal_margin(0)
        .split(tile_layout_horizontal[1]);

    let racks = &app.retrieve_racks_as_string();
    let tiles = &racks[0];
    for i in 0..tiles.len(){
        let tile_layout = tiles_layout[i];
        debug!("Generating tile {} with text {} of size ({},{})", i, tiles[i], tile_layout.width, tile_layout.height);

        let tile = Paragraph::new(tiles[i].to_string())
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
            );

        f.render_widget(tile, tiles_layout[i]);
    }

}