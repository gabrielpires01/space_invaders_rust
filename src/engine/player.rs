#[derive(Clone, Copy)]
pub struct Player {
   pub pos: [usize; 2], 
   pub max_x: usize,
}

impl Player {
    pub fn move_left(&mut self) {
        if self.pos[0] == 3 {
            return 
        }
        self.pos[0] = self.pos[0] - 1
    }

    pub fn move_right(&mut self) {
        if self.pos[0] == self.max_x {
            return 
        }
        self.pos[0] = self.pos[0] + 1
    }

    pub fn render(&self, col: usize, row: usize) -> &str {
        if self.pos[1] == row && col >= 2 {
            if self.pos[0] == col - 2 {
                "\\"
            }else if self.pos[0] == col - 1 {
                "I" 
            } else if self.pos[0] == col {
                "/"
            }else {
                " "
            }
        } else {
            " "
        }
    }
}
