use rand::Rng;

use crate::board::{Field, EMPTY_VAL, ENEMY_VAL};

fn rand_enemy_pos(cols: usize, rows: usize) -> [usize; 2] {
    let half_map = rows/3;
    let x = rand::thread_rng().gen_range(2..cols);
    let y = rand::thread_rng().gen_range(0..half_map);

    [x,y]        
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub alive: bool,

    max_x: usize,
    max_y: usize,
    speed: usize,
}

impl Enemy {
    pub fn move_up(&mut self) {
        if self.y == 0 {
            return 
        }
        self.y = self.y - self.speed
    }

    pub fn move_down(&mut self) {
        if self.y == self.max_y - 1 {
            self.alive = false;
            return 
        }
        self.y = self.y + self.speed
    }

    pub fn move_left(&mut self) {
        if self.x == 3 {
            return 
        }
        self.x = self.x - self.speed
    }

    pub fn move_right(&mut self) {
        if self.x + 2 == self.max_x {
            return 
        }
        self.x = self.x + self.speed
    }

    pub fn get_value(&self, col: usize, row: usize) -> u8 {
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

    pub fn render(&self, col: usize, row: usize) -> &str {
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
        let signal: i8 = rand::thread_rng().gen_range(-1..=1);
        let y_move: bool = rand::thread_rng().gen();
         
        if y_move == false {
            if signal < 0 {
                self.move_left();
            } else if signal > 0 {
                self.move_right();
            } else {}
        } else {
//            if signal > 0 {
//                self.move_down();
//            } else {}
        }
        
    }
    pub fn new(cols: usize, rows: usize) -> Enemy {
        let [x,y] = rand_enemy_pos(cols, rows);

        Enemy { x, y, alive: true, max_x: cols, max_y: rows, speed: 1 }
    }
}
