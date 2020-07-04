use crate::{ Node, NodeError, Variant, VariantType, NodeInputSlotInfo, NodeOutputSlotInfo };

pub struct ConstantInteger {
    _value: i64,
}

impl ConstantInteger {
    pub fn new() -> Box<dyn Node> {
        Box::new(
            ConstantInteger { 
                _value: 0
            }
        )
    }
}

impl Node for ConstantInteger {

    fn get_name(&self) -> String {
        String::from("Constant Integer")
    }

    fn invoke(&self, _input: &mut[Variant], _output: &mut[Variant]) -> Result<(), NodeError> {
        unimplemented!();
    }

    fn get_inputs(&self) -> Result<Vec<NodeInputSlotInfo>, NodeError> {
        unimplemented!();
    }

    fn determine(&self, _input: &[VariantType]) -> Result<Vec<NodeOutputSlotInfo>, NodeError> {
        unimplemented!();
    }
}
