use std::io::stdout;

use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use engine::events::handle_events;

use crate::engine::*;

pub mod engine;

fn main() {
    enable_raw_mode().unwrap();
    let board = board::Board { rows: 20, cols: 30};
    let player_len = 3;
    let mut player = player::Player { pos: [board.cols / 2, board.rows - 1], max_x: board.cols - player_len };

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let mut leave_game = false;
    while !leave_game {
        leave_game = handle_events(&mut player).unwrap();
        board.render(&player ,&mut stdout);
    }
    execute!(stdout, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
