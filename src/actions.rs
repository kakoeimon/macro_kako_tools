use std::collections::HashMap;
use macroquad::{KeyCode, is_key_down, is_key_pressed};
use gilrs::{Gilrs, Button, Event, EventType, Axis};


pub enum ActionInput {
    Keys(KeyCode),
    PadButton((usize, Button)),
    PadAxis((usize, Axis, i32)),

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
    pub gilrs: Gilrs,
    pub dead_zone: f32,
}

impl Actions {
    pub fn new()-> Self {
        Self {
            actions: HashMap::new(),
            gilrs: Gilrs::new().unwrap(),
            dead_zone: 0.3,
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
        
        let mut gamepad_button: HashMap<(usize, Button), f32> = HashMap::new();
        let mut gamepad_axis: HashMap<(usize, Axis, i32), f32> = HashMap::new();
        
        while let Some(Event { id, event, time: _ }) = self.gilrs.next_event() {
            //println!("{:?} New event from {}: {:?}", time, id, event);
            match event {
                EventType::ButtonChanged(button, pow, _) => {
                    gamepad_button.insert((id.into(), button), pow);
                },
                EventType::AxisChanged(axis, pow, _) => {
                    if pow == 0.0 {
                        gamepad_axis.insert((id.into(), axis, -1), 0.0);
                        gamepad_axis.insert((id.into(), axis, 1), 0.0);
                    } else if pow > 0.0 {
                        gamepad_axis.insert((id.into(), axis, 1), pow);
                        gamepad_axis.insert((id.into(), axis, -1), 0.0);
                    } else {
                        gamepad_axis.insert((id.into(), axis, -1), pow);
                        gamepad_axis.insert((id.into(), axis, 1), 0.0);
                    }
                }
                _=>(),
            }
        }

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
                    ActionInput::PadButton(e) => {
                        match action.state {
                            ActionState::Pressed => action.state = ActionState::Down,
                            ActionState::Released => action.state = ActionState::Up,
                            _=> (),
                        }
                        if let Some(pow) = gamepad_button.get(&e) {
                            if *pow != 0.0 {
                                action.state = ActionState::Pressed;
                            } else {
                                action.state = ActionState::Released;
                            }
                        }
                    },
                    ActionInput::PadAxis(e) => { 
                        match action.state {
                            ActionState::Pressed => action.state = ActionState::Down,
                            ActionState::Released => action.state = ActionState::Up,
                            _=> (),
                        }
                        if let Some(pow) = gamepad_axis.get(&e) {
                            if (*pow).abs() > self.dead_zone {
                                match action.state {
                                    ActionState::Pressed => {
                                        action.state = ActionState::Down;
                                    },
                                    _ => {
                                        action.state = ActionState::Pressed;
                                    }
                                }
                            } else {
                                match action.state {
                                    ActionState::Released => {
                                        action.state = ActionState::Up;
                                    },
                                    _ => {
                                        action.state = ActionState::Released;
                                    }
                                }
                                
                            }
                        }
                        
                    }
    
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
