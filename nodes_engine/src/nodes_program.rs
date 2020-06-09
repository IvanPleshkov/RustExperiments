use crate::{ Variant, VariantType, Node, NodeError, NodeInputSlotInfo, NodeOutputSlotInfo };

pub struct NodesProgram {
    name: String,
}

impl Node for NodesProgram {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn invoke(&self, _inputs: &mut [Variant], _outputs: &mut [Variant]) -> Result<(), NodeError> {
        unimplemented!()
    }

    fn get_inputs(&self) -> Result<Vec<NodeInputSlotInfo>, NodeError> {
        unimplemented!()
    }

    fn determine(&self, _inputs: &[VariantType]) -> Result<Vec<NodeOutputSlotInfo>, NodeError> {
        unimplemented!()
    }
}

impl NodesProgram {
    pub fn new() -> NodesProgram {
        NodesProgram {
            name: String::new(),
        }
    }
}
