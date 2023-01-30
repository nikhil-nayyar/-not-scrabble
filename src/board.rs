use crate::tiles::{Tile};
use std::{io::Read, fmt};
use log::{debug, info};
use std::fs::File;


pub struct Board{
    rows: u8,
    cols: u8,
    data: Vec<Vec<Cell>>,
    pub cursor: (i8,i8), // (x-coord, y-coord) origin in top-left
}

impl Board{
    
    pub fn new(cols: u8, rows: u8, path: &str) -> Self{

        let mut data: Vec<Vec<Cell>> = Vec::new();

        // load board data from file
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let parsed: Vec<Vec<String>> = serde_json::from_str(&contents).unwrap();
        debug!("Board data:\n{}",contents);

        // populate board data
        for row in 0..rows{
            let mut row_vector: Vec<Cell> = Vec::new();
            for col in 0..cols{
                row_vector.push(
                    Cell::new(row, col, Cell::to_modifier(&parsed[row as usize][col as usize]), CellState::Empty)
                )
            }
            data.push(row_vector);
        }

        Board{
            rows,
            cols,
            data,
            cursor: (7,7),
        }

    }

    pub fn move_cursor(&mut self, delta_x: i8, delta_y: i8,){

            if delta_x + self.cursor.0 >= 0 && (delta_x + self.cursor.0) < self.rows as i8{
                self.cursor.0 += delta_x;
            }

            if delta_y + self.cursor.1 >= 0 && (delta_y + self.cursor.1) < self.cols as i8{
                self.cursor.1 += delta_y;
            }

            info!("New cursor position ({},{})", self.cursor.0, self.cursor.1);

        }

    pub fn place_tile(&mut self, tile: Tile){
        info!("Placing {} at ({},{})", &tile.get_letter(), self.cursor.0, self.cursor.1);
        self.data[self.cursor.1 as usize][self.cursor.0 as usize].set_tile(tile);
        // row = y = 1, col = x = 1
    }

    pub fn pickup_tile(&mut self)-> Option<Tile>{
        self.data[self.cursor.1 as usize][self.cursor.0 as usize].tile.take()
    }

    pub fn get_cell(&self, row: usize, col: usize) -> &Cell{
        &self.data[row][col]
    }

}

impl std::fmt::Display for Board{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
       let mut result: String = "".to_string();

        for r in 0..self.rows{
            for c in 0..self.cols{
                let cell: &Cell = self.data.get(r as usize).unwrap().get(c as usize).unwrap();
                result.push_str(&cell.to_string());
        }
        result.push('\n');
       }

       write!(f,"{}", result)
    }

}

pub struct Cell{
    pub row: u8,
    pub col: u8,
    pub modifier: CellModifier,
    pub state: CellState,
    pub tile: Option<Tile>,
}

impl Cell{
    fn new(row: u8, col: u8, modifier: CellModifier, state: CellState)->Self{
        Cell { row, col, modifier, state, tile: Option::None}
    }

    pub fn to_modifier(data: &String) -> CellModifier{

        let mut result: CellModifier = CellModifier::Normal;

        if data=="TW"{result = CellModifier::TripleWord}
        else if data=="DW"{result = CellModifier::DoubleWord}
        else if data=="TL"{result= CellModifier::TripleLetter}
        else if data=="DL"{result= CellModifier::DoubleLetter}

        result
    }

    fn set_tile(&mut self, tile: Tile){
        self.tile = Some(tile);
    }

    pub fn has_tile(&self) -> bool{
        match &self.tile{
            Some(_) => {true}
            None => {false}
        }
    }

    pub fn get_tile_text(&self) -> String{

        if self.has_tile(){
            self.tile.as_ref().unwrap().to_string()
        } else{
            "".to_string()
        }

    }

}

impl std::fmt::Display for Cell{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
       let mut result: String = "".to_string();

        if self.tile.is_none(){
            result += "[  ]";
        } else{
            result+= &self.tile.as_ref().unwrap().to_string();
        }

       write!(f,"{}", result)
    }

}

pub enum CellModifier{
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
    Normal,
}

pub enum CellState{
    Empty,
    Tiled,
    Selected
}