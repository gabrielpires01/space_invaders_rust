use std::io::Result;
use crossterm::{event::{Event, KeyCode}, *};
use crate::player::Player;

pub fn handle_events(player: &mut Player) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read().expect("Read key") {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => {
                        if c.to_string() == "d" {
                            player.move_right();
                        }else if c.to_string() == "a" {
                            player.move_left();
                        }
                        else if c.to_string() == "q" {
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
