use std::collections::HashMap;

use blaze_explorer_lib::action::PopupAction;
use blaze_explorer_lib::mode::Mode;
use blaze_explorer_lib::plugin::plugin_action::PluginAction;
use blaze_explorer_lib::plugin::plugin_commands::{PluginDropSearchChar, PluginPushSearchChar};
use blaze_explorer_lib::{
    action::{Action, AppAction},
    create_plugin_action,
    plugin::plugin_commands::PluginQuit,
};
use blaze_explorer_lib::{custom_action, insert_binding};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::flash_commands::{launch_flash_jump, launch_flash_open};
use blaze_explorer_lib::input_machine::input_machine_helpers::convert_str_to_events;

pub const PLUGIN_NAME: &str = "Flash";

//Default popup action
pub fn default_popup_action(key_event: KeyEvent) -> Option<Action> {
    match key_event.code {
        KeyCode::Char(ch) => Some(create_plugin_action!(PluginPushSearchChar, ch)),
        _ => Some(Action::PopupAct(PopupAction::Quit)),
    }
}

//Default functionalities
pub fn get_functionalities() -> HashMap<String, Action> {
    let mut functionality_map = HashMap::new();
    functionality_map.insert("FlashJump".to_string(), custom_action!(launch_flash_jump));
    functionality_map.insert("FlashOpen".to_string(), custom_action!(launch_flash_open));
    functionality_map.insert("FlashQuit".to_string(), create_plugin_action!(PluginQuit));
    functionality_map.insert(
        "FlashDropSearchChar".to_string(),
        create_plugin_action!(PluginDropSearchChar),
    );

    functionality_map
}

//Default bindings
pub fn get_default_bindings() -> HashMap<(Mode, Vec<KeyEvent>), String> {
    let mut bindings_map = HashMap::new();
    insert_binding!(bindings_map, Mode::PopUp, "<Esc>", "FlashQuit");
    insert_binding!(bindings_map, Mode::PopUp, "<BS>", "FlashDropSearchChar");
    insert_binding!(bindings_map, Mode::Normal, "m", "FlashJump");
    insert_binding!(bindings_map, Mode::Normal, "M", "FlashOpen");
    bindings_map
}
