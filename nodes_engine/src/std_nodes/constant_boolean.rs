use crate::{ Node, NodeError, Variant, VariantType, NodeInputSlotInfo, NodeOutputSlotInfo };

pub struct ConstantBoolean {
    _value: bool,
}

impl ConstantBoolean {
    pub fn new() -> Box<dyn Node> {
        Box::new(
            ConstantBoolean { 
                _value: false
            }
        )
    }
}

impl Node for ConstantBoolean {

    fn get_name(&self) -> String {
        String::from("Constant Boolean")
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
