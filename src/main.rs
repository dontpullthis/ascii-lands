pub mod engine;
pub mod ui;

use std::io::stdout;
use std::sync::{Arc, Mutex};

use crate::ui::scenes::main_menu::MainMenuScene;
use crate::ui::scenes::new_game::NewGameScene;
use crate::engine::Engine;
use crate::engine::game_state::GameState;
use crate::ui::scenes::defs;

use crossterm::Result;
use crossterm::{
    cursor::Hide,
    event::DisableMouseCapture,
    execute,
    terminal::enable_raw_mode,
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), DisableMouseCapture, Hide)?; // disable mouse capture and hide cursor

    let scene_main_menu = Arc::new(Mutex::new(MainMenuScene::new()));
    let game_state = GameState::new(scene_main_menu.clone());
    let mut engine = Engine::new(game_state);
    engine.add_scene(defs::SCENE_MAIN_MENU, scene_main_menu);
    engine.add_scene(defs::SCENE_NEW_GAME, Arc::new(Mutex::new(NewGameScene::new())));
    engine.set_current_scene(defs::SCENE_MAIN_MENU);

    engine.run();

    Ok(())
}
