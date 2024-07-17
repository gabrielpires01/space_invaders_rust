use crate::board::{EMPTY_VAL, PLAYER_VAL};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub max_x: usize,
}

impl Player {
    pub fn move_left(&mut self) {
        if self.x == 3 {
            return 
        }
        self.x = self.x - 1
    }

    pub fn move_right(&mut self) {
        if self.x == self.max_x {
            return 
        }
        self.x = self.x + 1
    }

    pub fn get_value(&self, col: usize, row: usize) -> u8 {
        if self.y == row && col >= 2 {
            if self.x == col - 2  || self.x == col - 1 || self.x == col {
                PLAYER_VAL
            } else {
                EMPTY_VAL
            }
        } else {
            EMPTY_VAL
        }
    }

    pub fn render(&self, col: usize, row: usize) -> &str {
        if self.y == row && col >= 2 {
            if self.x == col - 2 {
                "\\"
            }else if self.x == col - 1 {
                "I" 
            } else if self.x == col {
                "/"
            }else {
                " "
            }
        } else {
            " "
        }
    }
}
