use blaze_explorer_lib::{
    action::Action,
    app::App,
    plugin::plugin_helpers::{PluginFetchResult, access_plugin},
};

use crate::flash_plugin::FlashJumpPopUp;

//Plugin functions
pub fn launch_flash_jump(app: &mut App) -> Option<Action> {
    let result = access_plugin(app, "Telescope");
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
    let result = access_plugin(app, "Telescope");
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
//Can use all the standard commands - no new ones needed
