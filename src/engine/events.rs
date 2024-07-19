use std::io::Result;
use crossterm::{event::{Event, KeyCode}, *};
use crate::{board::Board, bullet::Bullet, player::Player, FPS};

pub fn handle_events(player: &mut Player, board: &mut Board, timer: u64) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(1000/FPS))? {
        if let Event::Key(key) = event::read().expect("Read key") {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => {
                        if c.to_string() == "d" {
                            player.move_right();
                        } else if c.to_string() == "a" {
                            player.move_left();
                        } else if c.to_string() == "w" {
                            board.bullets.push(Bullet {speed: 1, x: player.x, y: player.y - 1, alive: true, created: timer})
                        } else if c.to_string() == "q" {
                            return Ok(true)
                        } 
                    },
                    KeyCode::Esc => {
                        return Ok(true)
                    },
                    _ => {}
                }
            }
        }
    }


    Ok(false)
}
