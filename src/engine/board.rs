use core::time;
use std::{collections::BTreeMap, io::{Stdout, Write}, thread, time::Instant};

use crossterm::{cursor, style::Print, terminal, ExecutableCommand};

use crate::{bullet::Bullet, enemy::Enemy, player::Player, FPS};

#[derive(PartialEq, Debug)]
pub enum Entity {
    ENEMY(Enemy),
    PLAYER(Player),
    BULLET(Bullet),
    EMPTY
}

pub type Field = Vec<Vec<Entity>>;

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub enemy_freq: u64
}

// Enemies = /@\
// player = /I\

pub const PLAYER_VAL: u8 = 3;
pub const BULLET_VAL: u8 = 2;
pub const ENEMY_VAL: u8 = 1;
pub const EMPTY_VAL: u8 = 0;


// Create ids for enemies and bullets
impl Board {
    pub fn new(rows: usize, cols:usize) -> Board {
        let enemy_freq = 2*FPS; // 3s spawn new enemy
        Board { enemies: vec![], bullets: vec![], rows, cols, enemy_freq }
    }

    fn cell_entity(&self, player: &Player, x: usize, y: usize) -> Entity {
        let value = player.get_value(x, y);
        if value == PLAYER_VAL {
            return Entity::PLAYER(player.to_owned());
        } 
        for e in self.enemies.iter() {
            if e.alive {
                let render_value = e.get_value(x, y);
                if render_value != EMPTY_VAL {
                    return Entity::ENEMY(e.to_owned());
                };
            };
        };
        for b in self.bullets.iter() {
            if b.alive {
                let render_value = b.get_value(x, y);
                if render_value != EMPTY_VAL {
                    return Entity::BULLET(b.to_owned());
                }
            }
        };
        Entity::EMPTY
    }

    fn cell_value(&self, entity: &Entity) -> u8 {
        match entity {
            Entity::EMPTY => EMPTY_VAL,
            Entity::ENEMY(_) => ENEMY_VAL,
            Entity::PLAYER(_) => PLAYER_VAL,
            Entity::BULLET(_) => BULLET_VAL
        }
    }

    fn cell_render(&self, entity: &Entity, col: usize, row: usize) -> String {
        match entity {
            Entity::EMPTY => " ".to_string(),
            Entity::ENEMY(e) => e.render(col, row).to_string(),
            Entity::PLAYER(p) => p.render(col, row).to_string(),
            Entity::BULLET(b) => b.render(col, row).to_string()
        }
    }

    fn get_board(&self, player: &Player) -> Field {
        let mut board: Field = vec![];
        for r in 0..self.rows {
            let mut row: Vec<Entity> = Vec::with_capacity(self.cols);
            for c in 0..self.cols {
                let entity = self.cell_entity(&player, c, r);

                row.push(entity);
            }
            board.push(row);
        };
        board
    }

    pub fn render(&mut self, player: &Player, stdout: &mut Stdout, frame_duration: time::Duration, timer: u64) -> Field {
        let start = Instant::now();
        if timer % self.enemy_freq == 0 {
            let enemy = Enemy::new(self.cols, self.rows, timer);
            self.enemies.push(enemy);
        }

        let mut enemy_map: BTreeMap<_,_> = self.enemies.iter().enumerate().map(|(i, e)| (i, e)).collect();
        let mut bullet_map: BTreeMap<_,_> = self.bullets.iter().enumerate().map(|(i, b)| (i, b)).collect();

        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        let board = self.get_board(player);
        
        let mut remove_bullets = vec![];
        let mut remove_enemies = vec![];
        for (i_bullet, b) in bullet_map.clone().into_iter() {
            for (i_enemy, _) in enemy_map.clone().into_iter() {
                let pos_entity = &board[b.y][b.x];
                let entity_value = self.cell_value(pos_entity);
                if entity_value == ENEMY_VAL{
                    remove_bullets.push(i_bullet);
                    remove_enemies.push(i_enemy);
                    break
                }
            }
        }

        remove_enemies.iter().for_each(|e| {
           enemy_map.remove(e);
        });
        remove_bullets.iter().for_each(|e| {
           bullet_map.remove(e);
        });

        let bullets = bullet_map.into_iter().map(|(_,b)| b.to_owned()).collect::<Vec<_>>();
        self.bullets = bullets.into_iter().filter(|b| b.alive).collect();
        self.bullets.iter_mut().for_each(|b| b.move_up());

        let enemies = enemy_map.into_iter().map(|(_,e)| e.to_owned()).collect::<Vec<_>>();
        self.enemies = enemies.into_iter().filter(|e| e.alive).collect();
        self.enemies.iter_mut().for_each(|e| {
            if (timer - e.created) % e.freq.try_into().unwrap_or(1) == 0 { 
                e.rand_mov(&board)
            }
        });

        for (y, line) in board.iter().enumerate() {
            let mut display_line = vec![];
            for (x, entity) in line.into_iter().enumerate() {
                display_line.push(self.cell_render(entity, x, y))
            };
            let string_line = display_line.into_iter().collect::<String>();
            stdout.execute(Print(string_line)).unwrap();
            stdout.execute(Print("\n")).unwrap();
            stdout.execute(cursor::MoveTo(0, y.try_into().unwrap())).unwrap();
        }

        stdout.flush().unwrap();

        let elapsed = start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }

        board
    }
}
