use std::result::Result;
use std::collections::HashSet;

#[derive(Debug)]
pub struct ChessPosition(i32, i32);

impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Result<ChessPosition, &'static str> {
        if x < 0 || y < 0 || x > 7 || y > 7 {
            Err("Invalid Position")
        } else {
            Ok(ChessPosition(x, y))
        }
    }
}

pub struct Queen(ChessPosition);

impl Queen {
    pub fn new(chess_position: ChessPosition) -> Queen {
        Queen(chess_position)
    }

    pub fn can_attack(&self, queen: &Queen) -> bool {
        //   0 1 2 3 4 5 6 7
        // 0 - - - - - - - -
        // 1 - - - - - - - -
        // 2 - - - - W - - -
        // 3 - - - - - - - -
        // 4 - - - - - - - -
        // 5 - - - - - - - -
        // 6 - - - - - - B -
        // 7 - - - - - - - -
        //
        (self.0).0 == (queen.0).0 || (self.0).1 == (queen.0).1 ||
        self.diagonal_moves_for_queen().contains(&((queen.0).0, (queen.0).1))
    }
    fn diagonal_moves_for_queen(&self) -> HashSet<(i32, i32)> {
        let mut v = Vec::new();

        // move to the upper left corner
        let mut x = (self.0).0;
        let mut y = (self.0).1;

        while x >= 0 && y >= 0 && x <= 7 && y <= 7 {
            v.push((x, y));
            x -= 1;
            y -= 1;
        }
        // move to the lower left corner
        x = (self.0).0;
        y = (self.0).1;

        while x >= 0 && y >= 0 && x <= 7 && y <= 7 {
            v.push((x, y));
            x -= 1;
            y += 1;
        }
        // move to the upper rigth corner
        x = (self.0).0;
        y = (self.0).1;

        while x >= 0 && y >= 0 && x <= 7 && y <= 7 {
            v.push((x, y));
            x += 1;
            y -= 1;
        }
        // move to the lower right corner
        x = (self.0).0;
        y = (self.0).1;

        while x >= 0 && y >= 0 && x <= 7 && y <= 7 {
            v.push((x, y));
            x += 1;
            y += 1;
        }

        v.into_iter().collect()
    }
}