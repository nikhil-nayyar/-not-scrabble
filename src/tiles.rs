

use core::fmt;
use std::{char::from_u32};
use std::collections::HashMap;
use std::thread::sleep;
use rand::Rng;
use crate::constants::LETTER_COUNT;
use crate::constants::LETTER_VALUE;

pub struct Tile{
    letter: char,
    value: u8,
    subscript: char,
    status: TileStatus,
}

pub enum TileStatus{
    Board,
    Rack,
    Bag
}

impl Tile{

    pub fn new(letter: char, value: u8, status: TileStatus) -> Self{
        Tile{
            letter,
            value,
            subscript: Self::generate_subscript(value),
            status
        }
    }
    
    pub fn get_letter(&self)->char{
        self.letter
    }

    const UNICODE_SUBSCRIPT_BASE: u32 = 0x2080;

    fn generate_subscript(point_value: u8) -> char {

        let result: char;

        if point_value==10{
           result = 'ₓ'
        } else{
            let subscript_unicode = Self::UNICODE_SUBSCRIPT_BASE + point_value as u32;
            result = from_u32(subscript_unicode).unwrap();
        }

        result

    }

    pub fn set_status(&mut self, new_status: TileStatus){
        self.status = new_status;
    }


}

impl std::fmt::Display for Tile{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}{}", self.letter, self.subscript)
    }

}


pub struct TileBag{
    data: HashMap<char,Vec<Tile>>,
    num: u8,
}

impl TileBag{
    pub fn new() -> Self{

        let mut generated: HashMap<char,Vec<Tile>> = HashMap::with_capacity(26);
        let mut num: u8 = 0;

        for (letter, count) in LETTER_COUNT.iter(){
            let mut letter_vector: Vec<Tile> = Vec::new();
            num+=count;
            for _ in 0..*count{
                letter_vector.push(
                    Tile::new(*letter, LETTER_VALUE[letter], TileStatus::Bag)
                )
            }
            generated.insert( *letter, letter_vector);
        }

        TileBag { data: generated, num}

    }

    pub fn draw_tile(&mut self) -> Option<Tile>{

        let mut rng = rand::thread_rng();
        let mut result: Option<Tile> = None;

        if self.num == 0 {
            result = None;
        } 
        else{
            let mut found = false;
            while !found{
                let letter_num: u8 = rng.gen_range(0..26);
                let letter_char: char = (65+letter_num) as char;
                if !self.data[&letter_char].is_empty() {
                    found = true;
                    result = self.data.get_mut(&letter_char).unwrap().pop();
                    self.num -= 1;
                }
                sleep(std::time::Duration::from_millis(10));
            }
        }

        result
        
    }

    pub fn is_empty(&self) -> bool{
        self.num == 0
    }

}

impl std::fmt::Display for TileBag{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{

        let mut result = String::new();

        let mut vector: Vec<(&char, &Vec<Tile>)> = self.data.iter().collect();
        vector.sort_by(|a,b| a.0.cmp(b.0));

        for (letter,tile_vector) in &vector{
            result.push(**letter);
            result.push_str(" : ");
            
            for tile in tile_vector.iter(){
                result = result + &tile.to_string();
            }

            result.push('\n')

        }

       write!(f,"{}", result)
    }
}


pub struct TileRack{
    data: HashMap<char,Vec<Tile>>,
    num: u8,
}

impl std::fmt::Display for TileRack{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let mut result = String::new();

        for (_, vector) in &self.data {
            for tile in vector {
                result += &tile.to_string();
            }
        }
        write!(f,"{}", result)
    }

}

impl TileRack{

    pub fn new(bag: &mut TileBag) -> Self{

        let mut rack = TileRack{
            data: HashMap::new(),
            num: 0
        };

        rack.draw_tiles(bag);

        rack

    }

    fn store_tile(&mut self, tile: Tile){

        let t_char = &tile.get_letter();

        if self.data.contains_key(t_char) {
            let vector = self.data.get_mut(&tile.get_letter()).unwrap();
            vector.push(tile);
        } else{
            let mut vector = vec![];
            vector.push(tile);
            self.data.insert(*t_char, vector);
        }

        self.num+=1;

    }

    fn draw_tiles(&mut self, bag: &mut TileBag){

        while self.num < 7 {
            let mut t = bag.draw_tile().unwrap();
            t.set_status(TileStatus::Rack);
            self.store_tile(t);
        }

    }

    pub fn retrieve_tiles_as_string(&self) -> Vec<String>{

        let mut result: Vec<String> = Vec::with_capacity(7);

        for tiles in self.data.values(){
            for tile in tiles{
                result.push(tile.to_string());
            }

        }

        result
    }

    pub fn pickup_tile(&mut self, letter: &char) -> Option<Tile>{

        let result = self.data.get(letter);

        match result{
            Some(_c) => {
                let vec = self.data.get_mut(letter).unwrap();
                let tile = vec.pop();

                if vec.is_empty(){
                    self.data.remove(letter);
                }

                tile
            }
            None => {None}
        }

    }

    pub fn place_tile(&mut self, tile: Tile){
        self.store_tile(tile);
    }

    pub fn get_count(&self) -> u8{
        self.num
    }

}