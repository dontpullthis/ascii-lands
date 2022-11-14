use std::io::stdout;

use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Colors, Print, SetColors};
use crossterm::event::Event;
use crossterm::terminal::{Clear, ClearType};

use crate::engine::actions::Action;
use crate::engine::ui::scenes::Scene;

use crate::constants::skills_stats;

const PADDING_LEFT          : u16 = 2;
const PADDING_LEFT_VALUE    : u16 = PADDING_LEFT + 15 + 2;

const LABEL_NAME:               &str = "Name";
const LABEL_WORLD_SEED:               &str = "World seed";
const LABEL_HINT_WORLD_SEED:    &str = "Keep empty to generate a random seed";

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
    fn on_show(&self) {
        execute!(
            stdout(),
            SetColors(Colors::new(Color::Green, Color::Black)),
            Clear(ClearType::All),
        ).unwrap();
    }

    fn render(&self) {
        execute!(
            stdout(),

	        MoveTo(PADDING_LEFT, 2),
            SetColors(Colors::new(Color::Green, Color::Black)),
            Print(format!("{: >15}:", LABEL_WORLD_SEED.to_string())),

	        MoveTo(PADDING_LEFT_VALUE, 4),
            SetColors(Colors::new(Color::Grey, Color::Black)),
            Print(LABEL_HINT_WORLD_SEED.to_string()),

	        MoveTo(PADDING_LEFT, 7),
            SetColors(Colors::new(Color::Green, Color::Black)),
            Print(format!("{: >15}:", LABEL_NAME)),

	        MoveTo(PADDING_LEFT, 11),
            SetColors(Colors::new(Color::Green, Color::Black)),
            Print(format!("{: >15}:", skills_stats::STAT_STRENGTH)),

	        MoveTo(PADDING_LEFT, 13),
            SetColors(Colors::new(Color::Green, Color::Black)),
            Print(format!("{: >15}:", skills_stats::STAT_DEXTERITY.to_string())),

	        MoveTo(PADDING_LEFT, 15),
            SetColors(Colors::new(Color::Green, Color::Black)),
            Print(format!("{: >15}:", skills_stats::STAT_VITALITY.to_string())),

	        MoveTo(PADDING_LEFT, 17),
            SetColors(Colors::new(Color::Green, Color::Black)),
            Print(format!("{: >15}:", skills_stats::STAT_INTELLIGENCE.to_string())),

	        MoveTo(PADDING_LEFT, 19),
            SetColors(Colors::new(Color::Green, Color::Black)),
            Print(format!("{: >15}:", skills_stats::STAT_SPIRITUALITY.to_string())),


            MoveTo(0, 0),
        ).unwrap();
    }

    fn event_handler(&mut self, event: &Event) -> Action {
        match event {
            _ => {},
        }

        Action::None
    }
}
