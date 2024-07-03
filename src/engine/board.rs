use core::time;
use std::{io::{Stdout, Write}, thread, time::Instant};

use crossterm::{cursor, style::Print, terminal, ExecutableCommand};

use crate::{bullet::Bullet, player::Player};

type Field = Vec<Vec<u8>>;
type DisplayField = Vec<String>;

pub struct Board {
    pub rows: usize,
    pub cols: usize,

    pub bullets: Vec<Bullet>
}

// Enemies = /@\
// player = /I\
impl Board {
    pub fn render(&mut self, player: &Player, stdout: &mut Stdout, frame_duration: time::Duration) -> Field {
        let start = Instant::now();
        let mut board: Field = vec![];
        let mut display_board: DisplayField = vec![]; 

        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();

        for r in 0..self.rows {
            let mut row: Vec<u8> = Vec::with_capacity(self.cols);
            let mut display_row: Vec<&str> = Vec::with_capacity(self.cols);
            for c in 0..self.cols {
                let mut value = 0;
                let mut display_value = player.render(c, r);
                if display_value == " " {
                    for b in self.bullets.iter() {
                        if b.alive {
                            display_value = b.render(c, r);
                            if display_value != " " {
                                value = 1;
                                break
                            }
                        }
                    }
                }

                row.push(value);
                display_row.push(display_value);
            }
            board.push(row);
            display_board.push(display_row.join(""));
        };
         
//        println!("{:?}\n", self.bullets);
//        println!("{:?}\n", player.pos);
//        println!("{:?}\n", display_board);
        self.bullets = self.bullets.clone().into_iter().filter(|b| b.alive).collect();
        self.bullets.iter_mut().for_each(|b| b.move_up());
        for (i, line) in display_board.iter().enumerate() {
            stdout.execute(Print(line)).unwrap();
            stdout.execute(Print("\n")).unwrap();
            stdout.execute(cursor::MoveTo(0, i.try_into().unwrap())).unwrap();
        }

        stdout.flush().unwrap();

        let elapsed = start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }

        board
    }
}
