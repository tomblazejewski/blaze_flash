use blaze_explorer_lib::{action::Action, create_plugin_action};

use crate::flash_commands::JumpToLetter;
use blaze_explorer_lib::plugin::plugin_action::PluginAction;
pub fn create_flash_jump_to_letter(letter: char) -> Action {
    create_plugin_action!(JumpToLetter, letter)
}

