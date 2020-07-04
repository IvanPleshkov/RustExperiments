use crate::camera::Camera;
use nodes_engine::NodesDocumentImpl;
use std::any::Any;

pub struct NodesEditable {
    pub document: NodesDocumentImpl,
    pub selection: NodesSelection,
    pub camera: Camera,
}

pub struct NodesSelection {

}

pub struct NodesEditorActionFabric {}

impl editor::ActionFabric<NodesEditable> for NodesEditorActionFabric {
    fn start_action(&self, _event: &dyn Any) -> Option<Box<dyn editor::Action<NodesEditable>>> {
        None
    }
}
