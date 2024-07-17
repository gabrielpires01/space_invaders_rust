use crate::board::BULLET_VAL;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bullet {
    pub x: usize,
    pub y: usize,

    pub speed: usize,
    pub alive: bool,
}

impl Bullet {
    pub fn get_value(&self, col: usize, row: usize) -> u8 {
        if self.y == row && self.x == col {
            return BULLET_VAL
        }
        0
    }
    pub fn render(&self, col: usize, row: usize) -> &str {
        if self.y == row && self.x == col {
            return "*"
        }
        " "
    }

    pub fn move_up(&mut self) {
        if self.hit() {
            return
        }
        self.y -= self.speed; 
    }

    fn hit(&mut self) -> bool {
        // Change to hit enemies
        if self.y < self.speed {
           self.alive = false; 
           return true
        };

        false
    }

}
