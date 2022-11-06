use std::io::stdout;

use crossterm::execute;
use crossterm::style::{Color, Colors, SetColors};
use crossterm::event::Event;
use crossterm::terminal::{Clear, ClearType};

use crate::engine::ui::scenes::Scene;

/// New game screen
pub struct NewGameScene {
    
}

impl NewGameScene {
    pub fn new() -> NewGameScene {
        NewGameScene {
            
        }
    }
}

impl Scene for NewGameScene {
    fn render(&self) {

        execute!(
            stdout(),
            SetColors(Colors::new(Color::Green, Color::Black)),
            Clear(ClearType::All),
        ).unwrap();
    }

    fn event_handler(&mut self, event: &Event) {
        match event {
            _ => {},
        }
    }
}