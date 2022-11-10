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
    event::{read, poll, Event, KeyCode, KeyModifiers},
};

pub struct Engine {
    state: Arc<Mutex<GameState>>,
    scenes: BTreeMap<usize, Arc<Mutex<dyn Scene>>>,
}

fn thread_render(state: Arc<Mutex<GameState>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            {
                println!("thread_render :: Acquired BEFORE state lock");
                let mut state_mutex = state.lock().unwrap();
                println!("thread_render :: Acquired AFTRER state lock");
                let mut scene_mutex = state_mutex.scene.lock().unwrap();
                scene_mutex.render();
                println!("thread_render :: Released state lock");
            }
            thread::sleep(Duration::from_millis(100));
        }
    })
}
 
fn thread_event_handler(state: Arc<Mutex<GameState>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            match poll(Duration::from_millis(100)) {
                Ok(_) => {},
                Err(_) => continue,
            };

            match read() {
                Ok(e) => {
                    event_handler(&e);
                    {
                        println!("thread_event :: Acquired BEFORE state lock");
                        let mut state_mutex = state.lock().unwrap();
                        println!("thread_event :: Acquired AFTER state lock / BEFORE scene lock");
                        let mut scene_mutex = state_mutex.scene.lock().unwrap();
                        println!("thread_event :: Acquired AFTER scene lock");
                        scene_mutex.event_handler(&e);
                        println!("thread_event :: Released state lock");
                    }
                },
                Err(_) => continue,
            };
            thread::sleep(Duration::from_millis(100));
        }
    })
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            state: Arc::new(Mutex::new(GameState::new())),
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

        println!("thread_set_scene :: Acquired BEFORE state lock");
        let mut state = self.state.lock().unwrap();
        println!("thread_set_scene :: Acquired AFTER state lock");
        state.scene = found_scene.clone();
        println!("thread_set_scene :: Released state lock");
    }

    pub fn run(&mut self) {
        thread_render(self.state.clone());
        thread_event_handler(self.state.clone());
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