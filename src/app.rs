use crate::{board::Board};
use crate::tiles::{TileRack, TileBag, Tile};

#[derive(Debug, Clone, Copy)]
pub enum TuiState{
    Board,
    Chat,
    Rack,
    Transition
}

pub struct App{
    pub board: Board,
    bag: TileBag,
    pub racks: Vec<TileRack>,
    pub state: TuiState,
    pub selected: TuiState,
}

impl App{

    pub fn new(board: Board, mut bag: TileBag, num_players: u8) -> Self{

        let mut racks:Vec<TileRack> = Vec::new();
        for _ in 0..num_players{
            racks.push(TileRack::new(&mut bag));
        }
        App { board:board, bag: bag, racks: racks, state: TuiState::Chat, selected: TuiState::Chat}
    }

}