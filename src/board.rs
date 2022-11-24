/*
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
*/