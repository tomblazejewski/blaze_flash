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
        // check how many paths are alphabetically earlier than the letter_path
        let smaller = path_list
            .iter()
            .filter(|x| x.chars().next().unwrap().to_ascii_lowercase() < self.letter)
            .collect::<Vec<_>>();
        let mut count = smaller.len();
        if count == path_list.len() {
            count -= 1;
        }
        Some(Action::ExplorerAct(ExplorerAction::JumpToId(count)))
    }
}

#[cfg(test)]
mod tests {
    use blaze_explorer_lib::{
        plugin::plugin_helpers::DummyPluginPopUp, testing_utils::create_custom_testing_folder,
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
        let file_list = vec![
            "aaa.txt",
            "aba.txt",
            "analysis/aaa.txt",
            "bbb.csv",
            "ccc.xlsx",
            "ddd.csv",
            "folder_1/aaa.txt",
            "ggg.csv",
            "mmm.log",
            "nnn.txt",
            "ppp.log",
            "rrr/",
            "sss.csv",
            "zzz.txt",
        ];
        let temp_dir = create_custom_testing_folder(file_list).unwrap();
        app.explorer_manager
            .update_path(temp_dir.root_dir.path().to_path_buf(), None);
        let mut jump_command = JumpToLetter::new('a');
        let result = jump_command.execute(&mut app);
        assert_eq!(
            result,
            Some(Action::ExplorerAct(ExplorerAction::JumpToId(0)))
        );
        let mut jump_command = JumpToLetter::new('b');
        let result = jump_command.execute(&mut app);
        assert_eq!(
            result,
            Some(Action::ExplorerAct(ExplorerAction::JumpToId(3)))
        );
        let mut jump_command = JumpToLetter::new('g');
        let result = jump_command.execute(&mut app);
        assert_eq!(
            result,
            Some(Action::ExplorerAct(ExplorerAction::JumpToId(7)))
        );
        let mut jump_command = JumpToLetter::new('z');
        let result = jump_command.execute(&mut app);
        assert_eq!(
            result,
            Some(Action::ExplorerAct(ExplorerAction::JumpToId(13)))
        );
    }
}
