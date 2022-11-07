pub mod ui;

use std::collections::BTreeMap;
use std::time::Duration;
use std::process::exit;

use ui::scenes::Scene;

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
};

struct GameState {
    scene: Option<(usize, Box<dyn Scene>)>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            scene: None,
        }
    }
}

pub struct Engine {
    state: GameState,
    scenes: BTreeMap<usize, Box<dyn Scene>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            state: GameState::new(),
            scenes: BTreeMap::new(),
        }
    }

    pub fn add_scene(&mut self, id: usize, scene: Box<dyn Scene>) {
        self.scenes.insert(id, scene);
    }

    pub fn set_current_scene(&mut self, id: usize) {
        match self.state.scene.take() {
            None => {},
            Some((i, s)) => self.add_scene(i, s),
        }
        match self.scenes.remove(&id) {
            None => return,
            Some(s) => self.state.scene = Some((id, s)),
        }
    }

    pub fn run(&mut self) {
        let (_, s) = match self.state.scene.as_mut() {
            None => return, // TODO: handle an error
            Some((i, s)) => (i, s),
        };

        loop {
            match poll(Duration::from_millis(100)) {
                Ok(e) => e,
                Err(_) => continue,
            };

            s.render();

            match read() {
                Ok(e) => {
                    event_handler(&e);
                    s.event_handler(&e);
                },
                Err(_) => continue,
            }
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