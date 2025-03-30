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

#[derive(Clone, PartialEq, Debug)]
pub struct JumpToLetter {
    letter: char,
}

impl JumpToLetter {
    pub fn new(letter: char) -> Self {
        Self { letter }
    }
}

impl Command for JumpToLetter {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &mut app.popup {
            None => {}
            &mut Some(ref mut popup) => popup.quit(),
        }
        let path_list = app
            .explorer_manager
            .find_elements("")
            .iter()
            .map(|x| x.filename.clone())
            .collect::<Vec<String>>();

        let letter_path = self.letter.to_string();

        // check how many paths are alphabetically earlier than the letter_path
        let mut count = path_list.iter().filter(|x| *x < &letter_path).count();
        if count == path_list.len() {
            count -= 1;
        }
        Some(Action::ExplorerAct(ExplorerAction::JumpToId(count)))
    }
}

#[cfg(test)]
mod tests {
    use blaze_explorer_lib::{
        plugin::plugin_helpers::DummyPluginPopUp, testing_utils::create_testing_folder,
    };

    use super::*;

    #[test]
    fn test_jump_and_close() {
        let mut app = App::new().unwrap();
        let mut jump_command = JumpAndClose::new(2);
        let popup = DummyPluginPopUp::new();
        app.popup = Some(Box::new(popup));
        let result = jump_command.execute(&mut app);

        assert_eq!(
            result,
            Some(Action::ExplorerAct(ExplorerAction::JumpToId(2)))
        );
    }
    #[test]
    fn test_jump_and_open() {
        let mut app = App::new().unwrap();
        let mut jump_command = JumpAndOpen::new(2);
        let popup = DummyPluginPopUp::new();
        app.popup = Some(Box::new(popup));
        let result = jump_command.execute(&mut app);

        assert_eq!(
            result,
            Some(Action::ExplorerAct(ExplorerAction::SelectDirectory))
        );
    }

    #[test]
    fn test_jump_to_letter() {
        let mut app = App::new().unwrap();
        let temp_dir = create_testing_folder().unwrap();
        let file_2 = temp_dir.file_list[1].clone();
        app.explorer_manager.show_in_folder(file_2);
        let mut jump_command = JumpToLetter::new('a');
        let result = jump_command.execute(&mut app);
        assert_eq!(
            result,
            Some(Action::ExplorerAct(ExplorerAction::JumpToId(0)))
        );
        let mut jump_command = JumpToLetter::new('f');
        let result = jump_command.execute(&mut app);
        assert_eq!(
            result,
            Some(Action::ExplorerAct(ExplorerAction::JumpToId(0)))
        );
        let mut jump_command = JumpToLetter::new('s');
        let result = jump_command.execute(&mut app);
        assert_eq!(
            result,
            Some(Action::ExplorerAct(ExplorerAction::JumpToId(3)))
        );
        let mut jump_command = JumpToLetter::new('z');
        let result = jump_command.execute(&mut app);
        assert_eq!(
            result,
            Some(Action::ExplorerAct(ExplorerAction::JumpToId(3)))
        );
    }
}
