use crate::NodesEditable;
use std::any::Any;

pub struct NodesEditorActionFabric {}

impl editor::ActionFabric<NodesEditable> for NodesEditorActionFabric {
    fn start_action(&self, _event: &dyn Any) -> Option<Box<dyn editor::Action<NodesEditable>>> {
        None
    }
}
