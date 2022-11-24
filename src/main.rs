extern crate lazy_static;

use core::fmt;
use std::{collections::HashMap};
use lazy_static::lazy_static;

const BOARD_DIMENSION:usize = 21;
const ALPHABET_SIZE: usize = 26;

lazy_static!{
    static ref LETTER_COUNT: HashMap<char,u8> = {
        let mut m = HashMap::with_capacity(26);
        m.insert('A',9);
        m.insert('B',2);
        m.insert('C',2);
        m.insert('D',4);
        m.insert('E',12);
        m.insert('F',2);
        m.insert('G',3);
        m.insert('H',2);
        m.insert('I',9);
        m.insert('J',1);
        m.insert('K',1);
        m.insert('L',4);
        m.insert('M',2);
        m.insert('N',6);
        m.insert('O',8);
        m.insert('P',2);
        m.insert('Q',1);
        m.insert('R',6);
        m.insert('S',4);
        m.insert('T',6);
        m.insert('U',4);
        m.insert('V',2);
        m.insert('W',2);
        m.insert('X',1);
        m.insert('Y',2);
        m.insert('Z',1);
        m
    };
}

lazy_static!{
    static ref LETTER_VALUE: HashMap<char,u8> = {
        let mut m = HashMap::with_capacity(26);
        m.insert('A',1);
        m.insert('B',3);
        m.insert('C',3);
        m.insert('D',2);
        m.insert('E',1);
        m.insert('F',4);
        m.insert('G',2);
        m.insert('H',4);
        m.insert('I',1);
        m.insert('J',8);
        m.insert('K',5);
        m.insert('L',1);
        m.insert('M',3);
        m.insert('N',1);
        m.insert('O',1);
        m.insert('P',3);
        m.insert('Q',10);
        m.insert('R',1);
        m.insert('S',1);
        m.insert('T',1);
        m.insert('U',1);
        m.insert('V',4);
        m.insert('W',4);
        m.insert('X',8);
        m.insert('Y',4);
        m.insert('Z',10);
        m
    };
}

fn main() {

    let board = Board::new(BOARD_DIMENSION,'□');
    let bag = TileBag::new();
    println!("{}",bag);


}


#[derive(Clone)]
#[allow(dead_code)]
struct Tile{
    letter: char,
    value: u8,
}

impl std::fmt::Display for Tile{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}", self.letter)
    }

}

struct Board{

    board: Vec<Vec<Tile>>,
    dimension: usize,
    
}

impl Board{
    fn new(board_size: usize, empty_char: char) -> Self{
        Self{
            board: vec![vec![Tile{letter:empty_char ,value: 0}; board_size]; board_size], 
            dimension: board_size, 
        }
    }
}

impl std::fmt::Display for Board{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
       let mut result: String = "".to_string();

       for r in 0..self.dimension{
        for c in 0..self.dimension{
            result.push(self.board[r][c].letter);
            result.push(' ');
        }
        result.push('\n');
       }

       write!(f,"{}", result)
    }

}

struct TileBag{
    data: HashMap<char,Vec<Tile>>,
}

impl TileBag{
    fn new() -> Self{

        let mut generated: HashMap<char,Vec<Tile>> = HashMap::with_capacity(26);

        for (key, value) in LETTER_COUNT.iter(){
            let mut letter_vector: Vec<Tile> = Vec::new();
            for _ in 0..*value{
                letter_vector.push(
                    Tile{letter: *key, value:LETTER_VALUE[key]}
                )
            }
            generated.insert(*key, letter_vector);
        }

        return TileBag { data: generated }

    }
}

impl std::fmt::Display for TileBag{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{

        let mut result = String::new();

        for (letter,tile_vector) in &self.data{
            result.push(*letter);
            result.push_str(" : ");
            result.push_str(&tile_vector.len().to_string()[..]);
            result.push('\n');
        }

       write!(f,"{}", result)
    }

}

struct TileStand{
    data: [Tile; 7],
}