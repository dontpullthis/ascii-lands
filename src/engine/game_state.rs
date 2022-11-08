use std::sync::{Arc, Mutex};

use crate::engine::ui::scenes::Scene;

/// Stores the game state and shares it across threads
pub struct GameState {
    pub scene: Arc<Mutex<dyn Scene>>
}

impl GameState {
    pub fn new(scene: Arc<Mutex<dyn Scene>>) -> GameState {
        GameState {
            scene
        }
    }
}