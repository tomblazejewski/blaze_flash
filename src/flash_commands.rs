use blaze_explorer_lib::{
    action::{Action, ExplorerAction},
    app::App,
    command::Command,
    plugin::plugin_helpers::{PluginFetchResult, access_plugin},
};

use crate::{flash_defaults::PLUGIN_NAME, flash_plugin::FlashJumpPopUp};

//Plugin functions
pub fn launch_flash_jump(app: &mut App) -> Option<Action> {
    let result = access_plugin(app, PLUGIN_NAME);
    let plugin = match result {
        PluginFetchResult::Err(action) => return action,
        PluginFetchResult::Ok(plugin) => plugin,
    };
    let popup_keymap = plugin.get_popup_keymap();
    let popup = Box::new(FlashJumpPopUp::new(popup_keymap));
    app.attach_popup(popup);

    None
}
pub fn launch_flash_open(app: &mut App) -> Option<Action> {
    let result = access_plugin(app, PLUGIN_NAME);
    let plugin = match result {
        PluginFetchResult::Err(action) => return action,
        PluginFetchResult::Ok(plugin) => plugin,
    };
    let popup_keymap = plugin.get_popup_keymap();
    let popup = Box::new(FlashJumpPopUp::new_with_open(popup_keymap));
    app.attach_popup(popup);

    None
}

//Popup functions
#[derive(Clone, PartialEq, Debug)]
pub struct JumpAndClose {
    id: usize,
}

impl JumpAndClose {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl Command for JumpAndClose {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &mut app.popup {
            None => {}
            &mut Some(ref mut popup) => popup.quit(),
        }
        Some(Action::ExplorerAct(ExplorerAction::JumpToId(self.id)))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct JumpAndOpen {
    id: usize,
}

impl JumpAndOpen {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl Command for JumpAndOpen {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &mut app.popup {
            None => {}
            &mut Some(ref mut popup) => popup.quit(),
        }
        app.explorer_manager.jump_to_id(self.id);
        Some(Action::ExplorerAct(ExplorerAction::SelectDirectory))
    }
}
