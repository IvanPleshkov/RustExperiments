use crate::{ Variant, VariantType };

pub trait Node {

    fn get_name(&self) -> String;

    fn invoke(&self, input: &mut[Variant], output: &mut[Variant]) -> Result<(), NodeError>;

    fn get_inputs(&self) -> Result<Vec<NodeInputSlotInfo>, NodeError>;

    fn determine(&self, input: &[VariantType]) -> Result<Vec<NodeOutputSlotInfo>, NodeError>;
}

pub struct NodeOutputSlotInfo {

    pub name: String,
}

pub struct NodeInputSlotInfo {

    pub name: String,
}

pub struct NodeError {

    pub message: String,

    pub failed_inputs: Vec<(usize, String)>,

    pub failed_outputs: Vec<(usize, String)>,
}
