use crate::NodesEditable;
use nodes_engine::NodeDocumentSlot;
use editor::{ Action, HandleEventResult };
use std::any::Any;

pub struct ConnectSlots {
    pub from: NodeDocumentSlot,
    pub to: Option<NodeDocumentSlot>,
}

impl Action<NodesEditable> for ConnectSlots {
    fn get_name(&self) -> &str {
        "Connect slot"
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
