pub mod ui;

use std::collections::BTreeMap;

use ui::scenes::Scene;

pub struct Engine {
    current_scene: Option<(usize, Box<dyn Scene>)>,
    scenes: BTreeMap<usize, Box<dyn Scene>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            current_scene: None,
            scenes: BTreeMap::new(),
        }
    }

    pub fn add_scene(&mut self, id: usize, scene: Box<dyn Scene>) {
        self.scenes.insert(id, scene);
    }

    pub fn set_current_scene(&mut self, id: usize) {
        match self.current_scene.take() {
            None => {},
            Some((i, s)) => self.add_scene(i, s),
        }
        match self.scenes.remove(&id) {
            None => return,
            Some(s) => self.current_scene = Some((id, s)),
        }
    }

    pub fn run(&mut self) {
        let (_, s) = match &self.current_scene {
            None => return,
            Some((i, s)) => (i, s),
        };

        s.render();
    }
}