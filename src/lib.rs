use std::collections::HashMap;

use blaze_explorer_lib::{mode::Mode, plugin::Plugin};
use flash_plugin::FlashPlugin;
use ratatui::crossterm::event::KeyEvent;

pub mod flash_commands;
pub mod flash_defaults;
pub mod flash_plugin;
//Plugin getter
#[unsafe(no_mangle)]
pub extern "Rust" fn get_plugin(
    bindings_map: HashMap<(Mode, Vec<KeyEvent>), String>,
) -> Box<dyn Plugin> {
    Box::new(FlashPlugin::new(bindings_map))
}
