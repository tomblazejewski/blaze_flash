//Struct representing the plugin used to jump to a chosen filename
//Aims to send and request data from the explorer table in order to send an action requesting to
//jump to a specific file
//

use blaze_explorer_lib::{
    action::PopupAction, input_machine::input_machine_helpers::convert_str_to_events,
};
use std::collections::HashMap;

use blaze_explorer_lib::{
    action::Action,
    app::App,
    app_context::AppContext,
    command::{Command, ResetStyling},
    components::{explorer_manager::ExplorerManager, explorer_table::GlobalStyling},
    flash_input_machine::FlashInputMachine,
    insert_binding,
    mode::Mode,
    plugin::{Plugin, plugin_popup::PluginPopUp},
};
use color_eyre::eyre::Result;
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
};
const JUMP_KEYS: [char; 25] = [
    'q', 'w', 'e', 'r', 't', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'z',
    'x', 'c', 'v', 'b', 'n', 'm',
];

pub fn pop_char(key_list: &mut Vec<char>, ch: Option<char>) -> char {
    match ch {
        Some(ch) => {
            key_list.retain(|k| *k != ch);
            ch
        }
        None => key_list.pop().unwrap(),
    }
}

///Creates a basic HashMap containing PopUp bindings regardless of query in action
fn create_basic_bindings() -> HashMap<(Mode, Vec<KeyEvent>), String> {
    let mut bindings_map = HashMap::new();
    insert_binding!(bindings_map, Mode::PopUp, "<Esc>", "FlashJumpQuit");
    insert_binding!(bindings_map, Mode::PopUp, "<BS>", "FlashJumpDropChar");
    bindings_map
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
                    false => Action::PopupAct(PopupAction::JumpAndClose(*u)),
                    true => Action::PopupAct(PopupAction::JumpAndOpen(*u)),
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

    fn push_search_char(&mut self, ch: char) {
        self.query.push(ch)
    }

    fn drop_search_char(&mut self) {
        self.query.pop();
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

    fn erase_text(&mut self) {}

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
                    false => format!(":{}", &self.query),
                }
            }
        )
    }

    fn get_own_keymap(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        self.keymap.clone()
    }
}
#[derive(PartialEq, Clone, Debug)]
pub struct FlashJump {
    plugin_bindings: HashMap<(Mode, Vec<KeyEvent>), String>,
    popup_bindings: HashMap<(Mode, Vec<KeyEvent>), String>,
    functionality_map: HashMap<String, Action>,
}
impl FlashJump {
    pub fn new(mut _ctx: AppContext, open: bool) -> Self {
        FlashJump {
            query: String::new(),
            input_machine: FlashInputMachine::new(open),
            should_quit: false,
            current_sequence: Vec::new(),
            jump_map: HashMap::new(),
            open_immediately: open,
        }
    }

    pub fn update_search_query(&mut self, query: String) {
        self.query = query;
    }

    pub fn update_interface(&mut self, explorer_manager: &mut ExplorerManager) {
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
        self.input_machine = FlashInputMachine::new(self.open_immediately);
        self.input_machine.merge_jump_actions(self.jump_map.clone());
        explorer_manager.set_styling(GlobalStyling::HighlightJump(
            self.query.clone(),
            self.jump_map.clone(),
        ));
    }
}
impl Plugin for FlashJump {
    fn display_details(&self) -> String {
        "Flash".to_string()
    }

    fn get_plugin_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String> {
        todo!()
    }

    fn get_popup_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String> {
        todo!()
    }

    fn get_functionality_map(&self) -> HashMap<String, Action> {
        todo!()
    }
}
