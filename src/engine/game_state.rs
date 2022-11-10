use std::sync::{Arc, Mutex};

use crossterm::event::Event;

use crate::engine::actions::Action;
use crate::engine::ui::scenes::Scene;

/// Stores the game state and shares it across threads
pub struct GameState {
    pub scene: Arc<Mutex<dyn Scene>>
}

/// A placeholder scene to init the engine
struct DummyScene {

}

impl Scene for DummyScene {
    fn render(&self) {

    }

    fn event_handler(&mut self, _event: &Event) -> Action {
        Action::None
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            scene: Arc::new(Mutex::new(DummyScene{}))
        }
    }
}