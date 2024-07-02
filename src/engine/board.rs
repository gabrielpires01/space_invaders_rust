use core::time;
use std::{io::{Stdout, Write}, thread};

use crate::player::Player;

type Field = Vec<Vec<u8>>;
type DisplayField = Vec<String>;

pub struct Board {
    pub rows: usize,
    pub cols: usize,
}

// Enemies = /@\
// player = /I\
impl Board {
    pub fn render(&self, player: &Player, stdout: &mut Stdout) -> Field {
        let mut board: Field = vec![];
        let mut display_board: DisplayField = vec![]; 

        let player_pos = player.pos;
        let player_col = player_pos[0];
        let player_row = player_pos[1];
        for r in 0..self.rows {
            let mut row: Vec<u8> = vec![];
            let mut display_row: Vec<String> = vec![];
            for c in 0..self.cols {
                let value = 0;
                let mut display_value = " ";
                if player_row == r && c > 2 {
                    if player_col == c - 2{
                        display_value = "\\";
                    }else if player_col == c - 1 {
                        display_value = "I"; 
                    } else if player_col == c {
                        display_value = "/";
                    }
                }

                row.push(value);
                display_row.push(display_value.to_string());
            }
            board.push(row);
            display_board.push(display_row.join(""));
        };

        assert_eq!(board.len(), self.rows);
        
        let board_string = display_board.join("\n");

        stdout.write(board_string.as_bytes()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));

        std::process::Command::new("clear").status().unwrap();
        board
    }
}
