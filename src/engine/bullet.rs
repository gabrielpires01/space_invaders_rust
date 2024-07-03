#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    pub speed: usize,
    pub position: [usize; 2],
    pub alive: bool,
}

impl Bullet {
    pub fn render(&self, col: usize, row: usize) -> &str {
        if self.position[1] == row && self.position[0] == col {
            return "*"
        }
        " "
    }

    pub fn move_up(&mut self) {
        if self.hit() {
            return
        }
        self.position[1] -= self.speed; 
    }

    fn hit(&mut self) -> bool {
        // Change to hit enemies
        if self.position[1] < self.speed {
           self.alive = false; 
           return true
        };

        false
    }

}
