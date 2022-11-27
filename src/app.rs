use crate::{board::Board};
use crate::tiles::{TileRack, TileBag, Tile};

pub struct App{
    pub board: Board,
    bag: TileBag,
    pub racks: Vec<TileRack>,
}

impl App{

    pub fn new(board: Board, mut bag: TileBag, num_players: u8) -> Self{

        let mut racks:Vec<TileRack> = Vec::new();
        for _ in 0..num_players{
            racks.push(TileRack::new(&mut bag));
        }
        App { board:board, bag: bag, racks: racks }
    }

}