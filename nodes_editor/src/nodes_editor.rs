use crate::camera::Camera;
use nodes_engine::NodesDocumentImpl;

pub struct NodesEditable {
    pub document: NodesDocumentImpl,
    pub selection: NodesSelection,
    pub camera: Camera,
}

pub struct NodesSelection {
    pub set: std::collections::HashSet<nodes_engine::NodeHandler>,
}
