

use core::fmt;
use std::{char::from_u32};
use std::collections::HashMap;
use std::thread::sleep;
use std::time;
use rand::Rng;
use crate::constants::LETTER_COUNT;
use crate::constants::LETTER_VALUE;

pub struct Tile{
    letter: char,
    point_value: u8,
    point_subscript: char,
}

impl Tile{

    pub fn new(letter: char, point_value: u8) -> Self{
        let t:Self = Tile{
            letter: letter,
            point_value: point_value,
            point_subscript: Self::generate_subscript( point_value),
        };
        t
    }
    
    pub fn get_letter(&self)->char{
        self.letter
    }

    pub fn get_tile(&self)-> String{
        let mut result = String::new();
        result.push_str(" [");
        result.push_str(&self.letter.to_string());
        result.push_str(&self.point_subscript.to_string());
        result.push_str("] ");
        result
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


}

impl std::fmt::Display for Tile{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "[{}{}]", self.letter, self.point_subscript)
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
                    Tile::new(*letter, LETTER_VALUE[letter])
                )
            }
            generated.insert( *letter, letter_vector);
        }

        println!("Generated TileBag with {} tiles", num);

        return TileBag { data: generated, num: num}

    }

    pub fn draw_tile(&mut self) -> Option<Tile>{

        let mut rng = rand::thread_rng();
        let mut result: Option<Tile> = None;

        if self.num == 0 {
            println!("No tiles remaining\n");
            result = None;
        } 
        else{
            let mut found = false;
            while !found{
                let letter_num: u8 = rng.gen_range(0..26);
                let letter_char: char = (65+letter_num) as char;
                if self.data[&letter_char].len() != 0 {
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
    pub data: HashMap<char,Vec<Tile>>,
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

        rack.get_letters(bag);

        rack

    }

    fn get_letters(&mut self, bag: &mut TileBag) -> (){

        while self.num < 7 {
            let t = bag.draw_tile().unwrap();
            let t_char = &t.get_letter();

            if self.data.contains_key(t_char) {
                let vector = self.data.get_mut(&t.get_letter()).unwrap();
                vector.push(t);
            } else{
                let mut vector = vec![];
                vector.push(t);
                self.data.insert(*t_char, vector);
            }
            self.num+=1;
        }

    }

    pub fn get_tiles(&mut self) -> Vec<&Tile>{

        let mut result: Vec<&Tile> = Vec::with_capacity(7);
        for (letter, tiles) in self.data.iter(){

            for tile in tiles.iter(){
                result.push(tile);
            }

        }

        result
    }

    pub fn get_count(&self) -> u8{
        self.num
    }

}