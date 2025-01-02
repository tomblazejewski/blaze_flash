//Struct representing the plugin used to jump to a chosen filename
//Aims to send and request data from the explorer table in order to send an action requesting to
//jump to a specific file

use blaze_explorer_lib::construct_plugin;
use blaze_explorer_lib::plugin::plugin_action::PluginAction;
use blaze_explorer_lib::{action::PopupAction, create_plugin_action};
use std::collections::HashMap;

use blaze_explorer_lib::{
    action::Action,
    app::App,
    command::{Command, ResetStyling},
    components::explorer_table::GlobalStyling,
    mode::Mode,
    plugin::{Plugin, plugin_popup::PluginPopUp},
};
use color_eyre::eyre::Result;
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
};

use crate::{
    flash_commands::{JumpAndClose, JumpAndOpen},
    flash_defaults::{
        PLUGIN_NAME, default_popup_action, get_default_bindings, get_functionalities,
    },
};
const JUMP_KEYS: [char; 25] = [
    'q', 'w', 'e', 'r', 't', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'z',
    'x', 'c', 'v', 'b', 'n', 'm',
];

///Removes a char from a key_list. The char to be removed can be specified or picked from the top
///of the list, depending on the `ch` argument
pub fn pop_char(key_list: &mut Vec<char>, ch: Option<char>) -> char {
    match ch {
        Some(ch) => {
            key_list.retain(|k| *k != ch);
            ch
        }
        None => key_list.pop().unwrap(),
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FlashJumpPopUp {
    keymap: HashMap<(Mode, Vec<KeyEvent>), Action>,
    pub should_quit: bool,
    pub query: String,
    pub should_open: bool,
    jump_map: HashMap<char, usize>,
}

impl FlashJumpPopUp {
    pub fn new_with_open(keymap: HashMap<(Mode, Vec<KeyEvent>), Action>) -> Self {
        FlashJumpPopUp {
            keymap,
            should_quit: false,
            query: "".to_string(),
            should_open: true,
            jump_map: HashMap::new(),
        }
    }
    pub fn new(keymap: HashMap<(Mode, Vec<KeyEvent>), Action>) -> Self {
        FlashJumpPopUp {
            keymap,
            should_quit: false,
            query: "".to_string(),
            should_open: false,
            jump_map: HashMap::new(),
        }
    }
    fn obtain_keymap(
        &self,
        jump_map: HashMap<char, usize>,
    ) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        let mut keymap = self.keymap.clone();
        for (ch, u) in jump_map.iter() {
            keymap.insert(
                (Mode::PopUp, vec![KeyEvent::new(
                    KeyCode::Char(*ch),
                    KeyModifiers::NONE,
                )]),
                match self.should_open {
                    false => create_plugin_action!(JumpAndClose, *u),
                    true => create_plugin_action!(JumpAndOpen, *u),
                },
            );
        }

        keymap
    }
    pub fn update_interface(&mut self, app: &mut App) {
        let mut explorer_manager = app.explorer_manager.clone();
        if !&self.query.is_empty() {
            let resulting_file_data = explorer_manager.find_elements(&self.query);
            //If the query gives no result, end immediately
            if resulting_file_data.is_empty() {
                self.quit();
                return;
            }
            let mut new_map = HashMap::new();
            let mut key_list = JUMP_KEYS.to_vec();
            let current_map_reverted = self
                .jump_map
                .iter()
                .map(|(k, v)| (*v, *k))
                .collect::<HashMap<usize, char>>();
            if resulting_file_data.len() > JUMP_KEYS.len() {
                self.jump_map = HashMap::new();
            } else {
                //if an id already exists in the map, it should have the same char
                for file_data in resulting_file_data {
                    let id = file_data.id;
                    if let Some(ch) = current_map_reverted.get(&id) {
                        let ch = pop_char(&mut key_list, Some(*ch));
                        new_map.insert(ch, id);
                    } else {
                        let ch = pop_char(&mut key_list, None);
                        new_map.insert(ch, id);
                    }
                }
                self.jump_map = new_map;
            }
        } else {
            if !self.jump_map.is_empty() {
                self.quit();
                return;
            }
            self.jump_map = HashMap::new();
        };
        let new_keymap = self.obtain_keymap(self.jump_map.clone());
        app.input_machine.attach_from_hashmap(new_keymap);
        explorer_manager.set_styling(GlobalStyling::HighlightJump(
            self.query.clone(),
            self.jump_map.clone(),
        ));
        app.explorer_manager = explorer_manager;
    }
}

impl PluginPopUp for FlashJumpPopUp {
    fn draw(&mut self, _frame: &mut Frame, _area: Rect) -> Result<()> {
        Ok(())
    }

    fn push_search_char(&mut self, ch: char) -> Option<Action> {
        self.query.push(ch);
        Some(Action::PopupAct(PopupAction::UpdatePopup))
    }

    fn drop_search_char(&mut self) -> Option<Action> {
        self.query.pop();
        Some(Action::PopupAct(PopupAction::UpdatePopup))
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn get_search_query(&self) -> String {
        self.query.clone()
    }

    fn destruct(&self) -> Option<Box<dyn Command>> {
        Some(Box::new(ResetStyling::new()))
    }

    fn erase_text(&mut self) -> Option<Action> {
        Some(Action::PopupAct(PopupAction::UpdatePopup))
    }

    fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn display_details(&self) -> String {
        format!(
            "{}{}",
            match self.should_open {
                true => String::from("Open"),
                false => String::from("Jump"),
            },
            {
                match self.query.is_empty() {
                    true => String::new(),
                    false => format!(": {}", &self.query),
                }
            }
        )
    }

    fn get_own_keymap(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        self.keymap.clone()
    }

    fn get_default_action(&self) -> Box<fn(KeyEvent) -> Option<Action>> {
        Box::new(default_popup_action)
    }

    fn update_app(&mut self, app: &mut App) {
        self.update_interface(app);
    }
}
#[derive(PartialEq, Clone, Debug)]
pub struct FlashPlugin {
    plugin_bindings: HashMap<(Mode, Vec<KeyEvent>), String>,
    popup_bindings: HashMap<(Mode, Vec<KeyEvent>), String>,
    functionality_map: HashMap<String, Action>,
}
impl FlashPlugin {
    pub fn new(custom_bindings_map: HashMap<(Mode, Vec<KeyEvent>), String>) -> Self {
        construct_plugin!(
            get_functionalities,
            get_default_bindings,
            custom_bindings_map
        )
    }
}
impl Plugin for FlashPlugin {
    fn display_details(&self) -> String {
        PLUGIN_NAME.to_string()
    }

    fn get_plugin_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String> {
        self.plugin_bindings.clone()
    }

    fn get_popup_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String> {
        self.popup_bindings.clone()
    }

    fn get_functionality_map(&self) -> HashMap<String, Action> {
        self.functionality_map.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_pop_char() {
        let mut key_list = JUMP_KEYS.to_vec();
        let to_remove = Some('a');
        let result = pop_char(&mut key_list, to_remove);
        assert_eq!(result, 'a');
        let mut expected_key_list = JUMP_KEYS.to_vec();
        expected_key_list.retain(|k| *k != 'a');
        assert_eq!(key_list, expected_key_list);
        pop_char(&mut key_list, None);
        assert_eq!(key_list.len(), expected_key_list.len() - 1);
    }

    #[test]
    fn test_popup_obtain_keymap() {
        let mut key_map = HashMap::new();
        key_map.insert(
            (Mode::PopUp, vec![KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::NONE,
            )]),
            Action::PopupAct(PopupAction::Quit),
        );
        let popup = FlashJumpPopUp::new(key_map.clone());
        let mut jump_map = HashMap::new();
        jump_map.insert('b', 2);
        jump_map.insert('c', 8);

        let final_keymap = popup.obtain_keymap(jump_map);
        let mut expected_keymap = key_map.clone();
        expected_keymap.insert(
            (Mode::PopUp, vec![KeyEvent::new(
                KeyCode::Char('b'),
                KeyModifiers::NONE,
            )]),
            create_plugin_action!(JumpAndClose, 2),
        );
        expected_keymap.insert(
            (Mode::PopUp, vec![KeyEvent::new(
                KeyCode::Char('c'),
                KeyModifiers::NONE,
            )]),
            create_plugin_action!(JumpAndClose, 8),
        );
        assert_eq!(final_keymap, expected_keymap);
    }

    #[test]
    fn test_update_interface() {
        let mut app = App::new().unwrap();
        let current_path = env::current_dir().unwrap();
        let test_path = current_path.join("tests");
        app.update_path(test_path, None);
        let mut flash = FlashJumpPopUp::new(HashMap::new());
        flash.query = "fold".to_string();
        //get hold of the explorer table before updating
        let mut et = app.explorer_manager.clone();
        flash.update_interface(&mut app);
        let jump_map = flash.jump_map.clone();
        assert_eq!(jump_map.len(), 2);
        let apparent_jump_map = HashMap::from([('m', 0), ('n', 1)]);
        et.set_styling(GlobalStyling::HighlightJump(
            flash.query.clone(),
            apparent_jump_map.clone(),
        ));
        assert_eq!(et, app.explorer_manager);
        //Get the root folder
        let _ = env::set_current_dir(current_path);
    }
}
