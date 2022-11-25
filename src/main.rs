mod board;
mod constants;
mod tiles;

use crate::tiles::TileBag;
use crate::tiles::TileRack;

fn main() {

    let mut sample_bag:TileBag = TileBag::new();

    let rack1 = TileRack::new(&mut sample_bag);
    let rack2 = TileRack::new(&mut sample_bag);

    println!("{}", rack1);
    println!("{}", rack2);
    
    ();
}