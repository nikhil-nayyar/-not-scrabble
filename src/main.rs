mod board;
mod constants;
mod tiles;

use crate::tiles::Tile;
use crate::tiles::TileBag;

fn main() {

    let sample_bag:TileBag = TileBag::new();
    println!("{}", sample_bag);
    
    ();
}