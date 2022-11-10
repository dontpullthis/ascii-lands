use std::io::stdout;
use std::process::exit;

use crossterm::execute;
use crossterm::style::{Color, Colors, Print, SetColors};
use crossterm::event::Event;
use crossterm::cursor::MoveTo;
use crossterm::event::KeyCode;
use crossterm::terminal::{Clear, ClearType, size};

use crate::engine::ui::scenes::Scene;
use crate::ui::scenes::defs::SCENE_NEW_GAME;

use crate::globals;

const LABEL_NEW_GAME: &str  = "  New Game  ";
const LABEL_LOAD_GAME: &str = "  Load Game ";
const LABEL_QUIT: &str      = "    Quit    ";

/// Main menu. Displays in application start
pub struct MainMenuScene {
    active_item: u8,
}

impl MainMenuScene {
    pub fn new() -> MainMenuScene {
        MainMenuScene {
            active_item: 0
        }
    }
}

impl Scene for MainMenuScene {
    fn render(&self) {
        let (w, h) = size().unwrap();
        let h_start = ((h - 4) / 2) as u16;

        let bg_colors: Vec<u8> = (0..=2).collect();
        let bg_colors: Vec<Color> = bg_colors.iter()
            .map(|i| if *i == self.active_item { Color::Blue } else { Color::Black })
            .collect();
        execute!(
            stdout(),
            SetColors(Colors::new(Color::Green, Color::Black)),
            Clear(ClearType::All),

            
            MoveTo(((w as usize - LABEL_NEW_GAME.len()) / 2) as u16, h_start),            
            SetColors(Colors::new(Color::Green, bg_colors[0])),
            Print(LABEL_NEW_GAME.to_string()),
            MoveTo(((w as usize - LABEL_LOAD_GAME.len()) / 2) as u16, h_start + 1),
            SetColors(Colors::new(Color::Green, bg_colors[1])),
            Print(LABEL_LOAD_GAME.to_string()),
            MoveTo(((w as usize - LABEL_QUIT.len()) / 2) as u16, h_start + 3),
            SetColors(Colors::new(Color::Green, bg_colors[2])),
            Print(LABEL_QUIT.to_string()),

            SetColors(Colors::new(Color::Green, Color::Black)),
        ).unwrap();
    }

    fn event_handler(&mut self, event: &Event) {
        match event {
            Event::Key(e) => {

                if e.code == KeyCode::Up && self.active_item > 0 {
                    self.active_item -= 1;
                } else if e.code == KeyCode::Down && self.active_item < 2 {
                    self.active_item += 1;
                } else if e.code == KeyCode::Enter {
                    match self.active_item {
                        0 => {
                            println!("Main menu :: BEFORE engine lock");
                            let mut engine = globals::ENGINE.lock().unwrap();
                            println!("Main menu :: AFTER engine lock");
                            engine.set_current_scene(SCENE_NEW_GAME);
                            println!("Main menu :: AFTER set current scene");
                        },
                        2 => exit(0),
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }
}