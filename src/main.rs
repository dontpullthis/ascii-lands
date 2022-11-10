#[macro_use]
extern crate lazy_static;

pub mod engine;
pub mod globals;
pub mod ui;

use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::ui::scenes::main_menu::MainMenuScene;
use crate::ui::scenes::new_game::NewGameScene;
use crate::engine::Engine;
use crate::ui::scenes::defs;


use crossterm::Result;
use crossterm::{
    cursor::Hide,
    execute,
    terminal::enable_raw_mode,
    terminal::disable_raw_mode,
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(),  Hide)?;

    {
        let mut engine = globals::ENGINE.lock().unwrap();
        engine.add_scene(defs::SCENE_MAIN_MENU, Arc::new(Mutex::new(MainMenuScene::new())));
        engine.add_scene(defs::SCENE_NEW_GAME, Arc::new(Mutex::new(NewGameScene::new())));
        engine.set_current_scene(defs::SCENE_MAIN_MENU);
        engine.run();
    }

    loop {
        thread::sleep(Duration::from_millis(100));
    }
    

    disable_raw_mode()?;
    execute!(stdout(),  Hide)?;

    Ok(())
}
