use crate::{Action, ActionFabric};
use std::any::Any;

pub struct Editor<T> {
    editable: T,
    action_fabric: Box<dyn ActionFabric<T>>,
    current_action: Option<Box<dyn Action<T>>>,
}

impl<T> Editor<T> {
    pub fn new(editable: T, action_fabric: Box<dyn ActionFabric<T>>) -> Editor<T> {
        Editor {
            editable: editable,
            action_fabric: action_fabric,
            current_action: None,
        }
    }

    pub fn get(&self) -> &T {
        &self.editable
    }

    pub fn handle_event(&mut self, event: &dyn Any) {
        loop {
            if self.current_action.is_none() {
                self.current_action = self.action_fabric.start_action(event);
            }

            if let Some(action) = &mut self.current_action {
                let handle_result = action.handle_event(&mut self.editable, event);
                if handle_result.action_finished {
                    self.current_action = None;
                }
                if handle_result.event_handled {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn undo(&mut self) {
        unimplemented!();
    }

    pub fn redo(&mut self) {
        unimplemented!();
    }
    
    pub fn is_modified(&self) -> bool {
        unimplemented!();
    }

	pub fn reset_modified_status(&mut self) {
        unimplemented!();
    }
}
