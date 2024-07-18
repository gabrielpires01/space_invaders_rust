use rand::Rng;

use crate::board::{Field, EMPTY_VAL, ENEMY_VAL};

fn rand_enemy_pos(cols: usize) -> [usize; 2] {
    let x = rand::thread_rng().gen_range(2..cols-2);
    let y = rand::thread_rng().gen_range(0..2);

    [x,y]        
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub alive: bool,
    pub created: u64,
    pub speed: usize,
    pub freq: usize,

    max_x: usize,
    max_y: usize,
}

impl Enemy {
    pub fn move_down(&mut self) {
        if self.y >= self.max_y - self.speed {
            self.alive = false;
            return 
        }
        self.y = self.y + self.speed
    }

    pub fn move_left(&mut self) {
        if self.x <= 3 || self.x < self.speed {
            return 
        }
        self.x = self.x - self.speed
    }

    pub fn move_right(&mut self) {
        if self.x + 3 >= self.max_x  || self.max_x < self.x + self.speed {
            return 
        }
        self.x = self.x + self.speed
    }

    pub fn get_value(&self, col: usize, row: usize) -> u8 {
        if !self.alive {
            return EMPTY_VAL
        }
        if self.y == row && col >= 2 {
            if self.x == col - 2  || self.x == col - 1 || self.x == col {
                ENEMY_VAL
            } else {
                EMPTY_VAL
            }
        } else {
            EMPTY_VAL
        }
    }

    pub fn die(&mut self) {
        self.alive = false;
    }

    pub fn render(&self, col: usize, row: usize) -> &str {
        if !self.alive {
            return " "
        }
        if self.y == row && col >= 2 {
            if self.x == col - 2 {
                "/"
            }else if self.x == col - 1 {
                "@" 
            } else if self.x == col {
                "\\"
            }else {
                " "
            }
        } else {
            " "
        }
    }

    pub fn rand_mov(&mut self, _board: &Field) {
        if !self.alive {
            return
        }
        let signal: i8 = rand::thread_rng().gen_range(-1..=1);
        let y_move: bool = rand::thread_rng().gen();
         
        if y_move == false {
            if signal < 0 {
                self.move_left();
            } else if signal > 0 {
                self.move_right();
            } else {}
        } else {
            if signal > 0 {
                self.move_down();
            } else {}
        }
        
    }
    pub fn new(cols: usize, rows: usize, time: u64) -> Enemy {
        let [x,y] = rand_enemy_pos(cols);

        Enemy { x, y, alive: true, max_x: cols, max_y: rows, speed: 1, created: time, freq: 4 }
    }
}
