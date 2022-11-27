use crate::tiles::Tile;
use std::fmt;
use tui::{text::Text, widgets::Cell};
pub struct Board{
    row: u8,
    col: u8,
    pub board: Vec<Vec<Space>>,
}

impl Board{
    
    pub fn new(row: u8, col: u8) -> Self{

        let mut board: Vec<Vec<Space>> = Vec::new();
        
        for r in 0..row{
            let mut row_vector: Vec<Space> = Vec::new();
            for c in 0..col{
                row_vector.push(
                    Space { row: r, col: c, modifier: SpaceModifier::Normal, tile: Option::None }
                )
            }
            board.push(row_vector);
        }

        Board{
            row,
            col,
            board,
        }

    }
}

impl std::fmt::Display for Board{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
       let mut result: String = "".to_string();

       for r in 0..self.row{
        for c in 0..self.col{
            let cell: &Space = self.board.get(r as usize).unwrap().get(c as usize).unwrap();
            result.push_str(&cell.to_string());
        }
        result.push('\n');
       }

       write!(f,"{}", result)
    }

}

pub struct Space{
    pub row: u8,
    pub col: u8,
    modifier: SpaceModifier,
    pub tile: Option<Tile>,
}


enum SpaceModifier{
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
    Normal,
}

impl Space{
    fn new(row: u8, col: u8, modifier: SpaceModifier)->Self{
        Space { row: row, col: col, modifier: modifier, tile: Option::None,}
    }
}

impl std::fmt::Display for Space{

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
