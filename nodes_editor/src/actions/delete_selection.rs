use crate::NodesEditable;
use editor::{ Action, HandleEventResult };
use std::any::Any;

pub struct DeleteSelection {
}

impl Action<NodesEditable> for DeleteSelection {

    fn get_name(&self) -> &str {
        "Delete selection"
    }

    fn handle_event(&mut self, _editable: &mut NodesEditable, _event: &dyn Any) -> HandleEventResult {
        unimplemented!();
    }

    fn undo(&self, _editable: &mut NodesEditable) {
        unimplemented!();
    }

    fn redo(&self, _editable: &mut NodesEditable) {
        unimplemented!();
    }
}
