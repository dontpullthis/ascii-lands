pub mod game_state;
pub mod ui;

use std::collections::BTreeMap;
use std::thread;
use std::time::Duration;
use std::process::exit;
use std::sync::{Arc, Mutex};

use ui::scenes::Scene;

use crate::engine::game_state::GameState;

use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
};

pub struct Engine {
    state: Arc<Mutex<GameState>>,
    scenes: BTreeMap<usize, Arc<Mutex<dyn Scene>>>,
}

fn thread_render(state: Arc<Mutex<GameState>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            state.lock().unwrap().scene.lock().unwrap().render();
            thread::sleep(Duration::from_millis(100));
        }
    })
}
 
fn thread_event_handler(state: Arc<Mutex<GameState>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            match read() {
                Ok(e) => {
                    event_handler(&e);
                    state.lock().unwrap().scene.lock().unwrap().event_handler(&e);
                },
                Err(_) => continue,
            }
        }
    })
}

impl Engine {
    pub fn new(game_state: GameState) -> Engine {
        Engine {
            state: Arc::new(Mutex::new(game_state)),
            scenes: BTreeMap::new(),
        }
    }

    pub fn add_scene(&mut self, id: usize, scene: Arc<Mutex<dyn Scene>>) {
        self.scenes.insert(id, scene);
    }

    pub fn set_current_scene(&mut self, id: usize) {
        let found_scene = match self.scenes.get(&id) {
            None => return,
            Some(s) => s,
        };

        self.state.lock().unwrap().scene = found_scene.clone();
    }

    pub fn run(&mut self) {
        let t1 = thread_render(self.state.clone());
        let t2 = thread_event_handler(self.state.clone());

        match t1.join() {
            _ => {},
        };
        match t2.join() {
            _ => {},
        }
    }
}

/// A global event handler
fn event_handler(event: &Event) {
    match event {
        Event::Key(e) => {
            if e.modifiers == KeyModifiers::CONTROL && e.code == KeyCode::Char('c') {
                exit(0);
            }
        },
        _ => {},
    }
}