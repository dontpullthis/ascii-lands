use std::sync::Mutex;

use crate::engine::Engine;

lazy_static! {
    pub static ref ENGINE: Mutex<Engine> = Mutex::new(Engine::new());
}
