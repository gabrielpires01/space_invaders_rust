use std::io::Result;
use crossterm::{event::{Event, KeyCode}, *};
use crate::{board::Board, bullet::Bullet, player::Player};

pub fn handle_events(player: &mut Player, board: &mut Board) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(1000/30))? {
        if let Event::Key(key) = event::read().expect("Read key") {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => {
                        if c.to_string() == "d" {
                            player.move_right();
                        } else if c.to_string() == "a" {
                            player.move_left();
                        } else if c.to_string() == "w" {
                            board.bullets.push(Bullet {speed: 1, position: [player.pos[0], player.pos[1] - 1], alive: true})
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
