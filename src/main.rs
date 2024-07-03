use std::{io::stdout, time::Duration};

use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use engine::events::handle_events;

use crate::engine::*;

pub mod engine;

fn main() {
    enable_raw_mode().unwrap();
    let mut board = board::Board { rows: 20, cols: 30, bullets: vec![] };
    let player_len = 3;
    let mut player = player::Player { pos: [board.cols / 2, board.rows - 1], max_x: board.cols - player_len };

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
