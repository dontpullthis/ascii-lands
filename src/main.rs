pub mod engine;
pub mod ui;

use std::io::stdout;

use crate::ui::scenes::main_menu::MainMenuScene;
use crate::engine::Engine;
use crate::ui::scenes::defs;

use crossterm::{execute, Result, terminal::SetSize};

fn main() -> Result<()> {
    // let (cols, rows) = size()?;
    // Do something with the terminal
    execute!(
        stdout(),
        SetSize(20, 20),
        // ScrollUp(5)
    )?;

    let mut engine = Engine::new();
    engine.add_scene(defs::SCENE_MAIN, Box::from(MainMenuScene::new()));
    engine.set_current_scene(defs::SCENE_MAIN);

    engine.run();

    Ok(())
}
