use std::{io::stdout, time::Duration};

use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use engine::{enemy::Enemy, events::handle_events};

use crate::engine::*;

pub mod engine;

fn main() {
    enable_raw_mode().unwrap();
    let enemy = Enemy::new(30, 20);
    let mut board = board::Board { rows: 20, cols: 30, bullets: vec![], enemies: vec![enemy] };
    let player_len = 3;
    let mut player = player::Player { x: board.cols / 2, y: board.rows - 1, max_x: board.cols - player_len };

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let mut leave_game = false;
    let frame_duration = Duration::from_millis(1000/30);
    while !leave_game {
        board.render(&player ,&mut stdout, frame_duration);
        leave_game = handle_events(&mut player, &mut board).unwrap();
    }
    execute!(stdout, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
