

use core::fmt;
use std::{char::from_u32};
use std::collections::HashMap;
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
}

impl TileBag{
    pub fn new() -> Self{

        let mut generated: HashMap<char,Vec<Tile>> = HashMap::with_capacity(26);

        for (letter, count) in LETTER_COUNT.iter(){
            let mut letter_vector: Vec<Tile> = Vec::new();
            for _ in 0..*count{
                letter_vector.push(
                    Tile::new(*letter, LETTER_VALUE[letter])
                )
            }
            generated.insert( *letter, letter_vector);
        }

        return TileBag { data: generated }

    }
}

/*

impl std::fmt::Display for TileBag{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{

        let mut result = String::new();

        for (letter,tile_vector) in &self.data{
            result.push(*letter);
            result.push_str(" : ");
            
            for tile in tile_vector.iter(){
                result = result + &tile.to_string();
            }

            result.push('\n')

        }

       write!(f,"{}", result)
    }

}
*/

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