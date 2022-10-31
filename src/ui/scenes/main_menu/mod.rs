use std::io::stdout;

use crossterm::execute;
use crossterm::style::{Color::{Green, Black}, Colors, Print, SetColors};
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};

use crate::engine::ui::scenes::Scene;

pub struct MainMenuScene {

}

impl MainMenuScene {
    pub fn new() -> MainMenuScene {
        MainMenuScene {

        }
    }
}

impl Scene for MainMenuScene {
    fn render(&self) {
        execute!(
            stdout(),
            Clear(ClearType::All),
            SetColors(Colors::new(Green, Black)),
            MoveTo(10, 10),
            
            Print("Hello, world!".to_string()),
            Print("Test!".to_string()),
        ).unwrap();
    }
}