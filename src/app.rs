use crossterm::event::{KeyEvent, KeyCode};
use log::info;

use crate::{board::Board};
use crate::tiles::{TileRack, TileBag};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiState{
    Game,
    Chat,
}

pub enum InputType{
    Action,
    Transition,
    Quit,
}

pub struct App{
    pub board: Board,
    pub bag: TileBag,
    pub racks: Vec<TileRack>,
    pub state: TuiState,
}

impl App{

    pub fn new(board: Board, mut bag: TileBag, num_players: u8) -> Self{

        let mut racks:Vec<TileRack> = Vec::new();
        for _ in 0..num_players{
            racks.push(TileRack::new(&mut bag));
        }
        App { board, bag, racks, state:TuiState::Chat}
    }
    
    pub fn process_input(key_event: &KeyEvent) -> InputType{
        info!("logged key {}", format!("{key_event:#?}"));
        match key_event.code{
            KeyCode::Char('1') | KeyCode::Char('2') => {InputType::Transition}
            KeyCode::Esc => {InputType::Quit}
            _ => {InputType::Action}
        }
    }


    pub fn handle_game(&mut self, key_event: &KeyEvent ){

        let _key_code = key_event.code;
    
        match key_event.code{
            KeyCode::Left => {self.board.move_cursor(-1,0);}
            KeyCode::Right => {self.board.move_cursor(1,0);}
            KeyCode::Down => {self.board.move_cursor(0,1);}
            KeyCode::Up => {self.board.move_cursor(0,-1);}
            KeyCode::Char(mut c) => {
                c.make_ascii_uppercase();
                let r = &mut self.racks[0];
                let t = r.pickup_tile(&c);
                match t{
                    Some(tile) => {
                        info!("Found character {}", &c);
                        self.board.place_tile(tile);
                    }
                    None => {
                        info!("Did not find character {}", &c);
                    }
                }
            }
            KeyCode::Backspace | KeyCode::Delete => {
                let r = &mut self.racks[0];
                let t = self.board.pickup_tile();
                match t {
                    Some(tile) => {r.place_tile(tile);}
                    None => {}
                }
            }
            _ => {}
        }
    
    }

    pub fn retrieve_racks_as_string(&self) -> Vec<Vec<String>>{

        let mut result: Vec<Vec<String>> = Vec::new();

        for rack in self.racks.iter(){
            result.push(rack.retrieve_tiles_as_string());
        }

        result

    }

}
