use crate::Node;
use nalgebra::Vector2;
use std::collections::HashMap;

pub use u64 as NodeHandler;
pub use u64 as ConnectionHandler;

pub trait NodesDocument {

    fn load(&mut self) -> std::io::Result<()>;

    fn save(&self) -> std::io::Result<()>;

    fn get_nodes_list(&self) -> Vec<NodeHandler>;

    fn add_node(&mut self, node: Box<dyn Node>) -> NodeHandler;

    fn add_node_with_handler(&mut self, node: Box<dyn Node>, handler: NodeHandler);

    fn remove_node(&mut self, _handler: NodeHandler) -> Result<(), ()>;

    fn set_node_position(&mut self, handler: NodeHandler, position: Vector2<f32>);

    fn get_node_position(&self, handler: NodeHandler) -> Vector2<f32>;

    fn add_connection(&mut self, connection: NodeDocumentConnection) -> ConnectionHandler;

    fn add_connection_with_handler(&mut self, connection: NodeDocumentConnection, handler: ConnectionHandler);

    fn get_connection(&self, handler: ConnectionHandler) -> Option<NodeDocumentConnection>;

    fn get_input_slot_connection(&self, slot: &NodeDocumentSlot) -> Option<ConnectionHandler>;

    fn get_output_slot_connections(&self, slot: &NodeDocumentSlot) -> Vec<ConnectionHandler>;
}

#[derive(Clone, PartialEq)]
pub struct NodeDocumentSlot {
    pub node: NodeHandler,
    pub slot_name: String,
    pub slot_index: u64,
}

#[derive(Clone, PartialEq)]
pub struct NodeDocumentConnection {
    pub from: NodeDocumentSlot,
    pub to: NodeDocumentSlot,
}

pub struct NodesDocumentImpl {
    node_handler_counter: NodeHandler,
    cells: HashMap<NodeHandler, NodeDocumentCell>,
    connection_handler_counter: ConnectionHandler,
    connections: HashMap<ConnectionHandler, NodeDocumentConnection>,
}

struct NodeDocumentCell {
    pub node: Box<dyn Node>,
    pub position: Vector2<f32>,
    pub input_connections: Vec<ConnectionHandler>,
    pub output_connections: Vec<ConnectionHandler>,
}

impl NodesDocumentImpl {

    pub fn new() -> NodesDocumentImpl {
        NodesDocumentImpl {
            node_handler_counter: 0,
            cells: HashMap::new(),
            connection_handler_counter: 0,
            connections: HashMap::new(),
        }
    }
}

impl NodesDocument for NodesDocumentImpl {

    fn load(&mut self) -> std::io::Result<()> {
        unimplemented!();
    }

    fn save(&self) -> std::io::Result<()> {
        unimplemented!();
    }

    fn get_nodes_list(&self) -> Vec<NodeHandler> {
        let mut result = Vec::new();
        for pair in &self.cells {
            result.push(*pair.0);
        };
        result
    }

    fn add_node(&mut self, node: Box<dyn Node>) -> NodeHandler {
        let handler = self.node_handler_counter + 1;
        self.node_handler_counter = self.node_handler_counter + 1;
        self.add_node_with_handler(node, handler);
        handler
    }

    fn add_node_with_handler(&mut self, node: Box<dyn Node>, handler: NodeHandler) {
        if handler >= self.node_handler_counter {
            panic!("manual handler should be used by node and deleted");
        }
        if self.cells.contains_key(&handler) {
            panic!("manual handler should be unused");
        }
        let cell = NodeDocumentCell {
            node: node,
            position: Vector2::new(0., 0.),
            input_connections: Vec::new(),
            output_connections: Vec::new(),
        };
        self.cells.insert(handler, cell);
    }

    fn remove_node(&mut self, _handler: NodeHandler) -> Result<(), ()> {
        unimplemented!();
    }

    fn set_node_position(&mut self, handler: NodeHandler, position: Vector2<f32>) {
        if let Some(cell) = self.cells.get_mut(&handler) {
            cell.position = position;
        } else {
            panic!("try to change position of unexisting node");
        }
    }

    fn get_node_position(&self, handler: NodeHandler) -> Vector2<f32> {
        if let Some(cell) = self.cells.get(&handler) {
            cell.position
        } else {
            panic!("try to get position of unexisting node")
        }
    }

    fn add_connection(&mut self, connection: NodeDocumentConnection) -> ConnectionHandler {
        let handler = self.connection_handler_counter + 1;
        self.connection_handler_counter = self.connection_handler_counter + 1;
        self.add_connection_with_handler(connection, handler);
        handler
    }

    fn add_connection_with_handler(&mut self, connection: NodeDocumentConnection, handler: ConnectionHandler) {
        if handler >= self.connection_handler_counter {
            panic!("manual handler should be used and deleted");
        }
        if self.connections.contains_key(&handler) {
            panic!("manual handler should be unused");
        }
        if !self.cells.contains_key(&connection.from.node) || !self.cells.contains_key(&connection.to.node) {
            panic!("connection with unexist node");
        }
        if self.get_input_slot_connection(&connection.to).is_some() {
            panic!("input connection is already exists");
        }
        if let Some(cell) = self.cells.get_mut(&connection.from.node) {
            cell.output_connections.push(handler);
        }
        if let Some(cell) = self.cells.get_mut(&connection.to.node) {
            cell.input_connections.push(handler);
        }
        self.connections.insert(handler, connection);
    }

    fn get_connection(&self, handler: ConnectionHandler) -> Option<NodeDocumentConnection> {
        if let Some(connection) = self.connections.get(&handler) {
            Some(connection.clone())
        } else {
            None
        }
    }

    fn get_input_slot_connection(&self, slot: &NodeDocumentSlot) -> Option<ConnectionHandler> {
        if let Some(cell) = self.cells.get(&slot.node) {
            for connection_handler in &cell.input_connections {
                if let Some(connection) = self.connections.get(connection_handler) {
                    if &connection.to == slot {
                        return Some(*connection_handler);
                    }
                }
            };
            None
        } else {
            None
        }
    }

    fn get_output_slot_connections(&self, slot: &NodeDocumentSlot) -> Vec<ConnectionHandler> {
        let mut result = Vec::new();
        if let Some(cell) = self.cells.get(&slot.node) {
            for connection_handler in &cell.output_connections {
                if let Some(connection) = self.connections.get(connection_handler) {
                    if &connection.to == slot {
                        result.push(*connection_handler);
                    }
                }
            };
        };
        result
    }
}
