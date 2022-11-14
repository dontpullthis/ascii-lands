pub mod actions;
pub mod game_state;
pub mod ui;

use std::collections::BTreeMap;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use ui::scenes::Scene;

use crate::engine::actions::Action;
use crate::engine::game_state::GameState;

use crossterm::{
    event::{read, poll, Event, KeyCode, KeyModifiers},
};

pub struct Engine {
    is_running: bool,
    state: Arc<Mutex<GameState>>,
    scenes: BTreeMap<usize, Arc<Mutex<dyn Scene>>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            is_running: true,
            state: Arc::new(Mutex::new(GameState::new())),
            scenes: BTreeMap::new(),
        }
    }

    pub fn add_scene(&mut self, id: usize, scene: Arc<Mutex<dyn Scene>>) {
        self.scenes.insert(id, scene);
    }

    pub fn set_current_scene(&mut self, id: usize) {
        {
            let found_scene = match self.scenes.get(&id) {
                None => return,
                Some(s) => s,
            };
    
            let mut state = self.state.lock().unwrap();
            state.scene = found_scene.clone();
            state.scene.lock().unwrap().on_show();
        }
        self.handle_action(Action::Render);
    }

    pub fn run(&mut self) {
        self.handle_action(Action::Render);
        while self.is_running {
            self.handle_event();
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn handle_event(&mut self) {
        match poll(Duration::from_millis(100)) {
            Ok(_) => {},
            Err(_) => return,
        };

        match read() {
            Ok(e) => {
                self.handle_action(event_handler(&e));
                let action = self.state.lock().unwrap().scene.lock().unwrap().event_handler(&e);
                self.handle_action(action);
            },
            Err(_) => {},
        };
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::None => {},
            Action::Render => self.state.lock().unwrap().scene.lock().unwrap().render(),
            Action::SetScene(id) => {
                self.set_current_scene(id);
            },
            Action::Quit => self.is_running = false,
        }
    }
}

/// A global event handler
fn event_handler(event: &Event) -> Action {
    match event {
        Event::Key(e) => {
            if e.modifiers == KeyModifiers::CONTROL && e.code == KeyCode::Char('c') {
                return Action::Quit
            }
        },
        _ => {},
    }

    Action::None
}