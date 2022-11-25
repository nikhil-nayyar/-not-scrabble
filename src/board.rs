use crate::tiles::Tile;
use std::fmt;
pub struct Board{
    row: u8,
    col: u8,
    board: Vec<Vec<Cell>>,
}

impl Board{
    
    pub fn new(row: u8, col: u8) -> Self{

        let mut board: Vec<Vec<Cell>> = Vec::new();
        
        for r in 0..row{
            let mut row_vector: Vec<Cell> = Vec::new();
            for c in 0..col{
                row_vector.push(
                    Cell { row: r, col: c, modifier: CellModifier::Normal, tile: Option::None }
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
            let cell: &Cell = self.board.get(r as usize).unwrap().get(c as usize).unwrap();
            result.push_str(&cell.to_string());
        }
        result.push('\n');
       }

       write!(f,"{}", result)
    }

}

struct Cell{
    row: u8,
    col: u8,
    modifier: CellModifier,
    tile: Option<Tile>,
}


enum CellModifier{
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
    Normal,
}

impl Cell{
    fn new(row: u8, col: u8, modifier: CellModifier)->Self{
        Cell { row: row, col: col, modifier: modifier, tile: Option::None,}
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