use crate::NodesEditable;
use editor::{ Action, HandleEventResult };
use std::any::Any;

pub struct SelectNode {
}

impl Action<NodesEditable> for SelectNode {

    fn get_name(&self) -> &str {
        "Select node"
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
