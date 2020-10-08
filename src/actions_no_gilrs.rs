use std::collections::HashMap;
use macroquad::{KeyCode, is_key_down, is_key_pressed};


pub enum ActionInput {
    Keys(KeyCode),

}

pub enum ActionState {
    Down,
    Pressed,
    Released,
    Up,
}

pub struct Action {
    pub input: ActionInput,
    pub state: ActionState,
}


impl Action {
    pub fn new(input: ActionInput) -> Self {
        Self {
            input,
            state: ActionState::Up,
        }
    }
}

pub struct Actions {
    pub actions: HashMap<String, Vec<Action>>,
}

impl Actions {
    pub fn new()-> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }

    pub fn add_action(&mut self, name: &str, input: ActionInput) {
        let key = name.to_owned();
        if self.actions.contains_key(&key) {
            self.actions.get_mut(&key).unwrap().push(Action::new(input));
        } else {
            let mut actions: Vec<Action> = Vec::new();
            actions.push(Action::new(input));
            self.actions.insert(key, actions);
        }
        
    }

    
    pub fn update(&mut self) {
        

        

        for actions in self.actions.values_mut() {
            for action in actions {
                match action.input {
                    ActionInput::Keys(key) => {
                        if is_key_pressed(key) {
                            action.state = ActionState::Pressed;
                        } else if is_key_down(key) {
                            action.state = ActionState::Down;
                        } else {
                            match action.state {
                                ActionState::Down | ActionState::Pressed => {
                                    action.state = ActionState::Released;
                                }
                                _ => {
                                    action.state = ActionState::Up;
                                }
                            }
                        }
                    },
                    
    
                }
            }
            
        }

    }

    pub fn is_action_down(&self, name: &str) -> bool {
        if let Some(actions) = self.actions.get(name) {
            for action in actions {
                match action.state {
                    ActionState::Down | ActionState::Pressed => return true,
                    _=> (),
                }
            }
            
        }
        false
    }

    pub fn is_action_just_pressed(&self, name: &str) -> bool {
        if let Some(actions) = self.actions.get(name) {
            for action in actions {
                match action.state {
                    ActionState::Pressed => return true,
                    _=> (),
                }
            }
           
        }
        false
    }

    pub fn is_action_released(&self, name: &str) -> bool {
        if let Some(actions) = self.actions.get(name) {
            for action in actions {
                match action.state {
                    ActionState::Up | ActionState::Released => return true,
                    _=> (),
                }
            }
            
        }
        false
    }

    pub fn is_action_just_released(&self, name: &str) -> bool {
        if let Some(actions) = self.actions.get(name) {
            for action in actions {
                match action.state {
                    ActionState::Released => return true,
                    _=> (),
                }
            }
            
        }
        false
    }
}
