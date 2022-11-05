pub mod engine;
pub mod ui;

use std::io::stdout;

use crate::ui::scenes::main_menu::MainMenuScene;
use crate::engine::Engine;
use crate::ui::scenes::defs;

use crossterm::Result;
use crossterm::{
    cursor::Hide,
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), DisableMouseCapture, Hide)?; // disable mouse capture and hide cursor

    let mut engine = Engine::new();
    engine.add_scene(defs::SCENE_MAIN, Box::from(MainMenuScene::new()));
    engine.set_current_scene(defs::SCENE_MAIN);

    engine.run();
    disable_raw_mode()?;

    Ok(())
}
