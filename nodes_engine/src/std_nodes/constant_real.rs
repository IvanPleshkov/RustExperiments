use crate::{ Node, NodeError, Variant, VariantType, NodeInputSlotInfo, NodeOutputSlotInfo };

pub struct ConstantReal {
    _value: f64,
}

impl ConstantReal {
    pub fn new() -> Box<dyn Node> {
        Box::new(
            ConstantReal { 
                _value: 0.0
            }
        )
    }
}

impl Node for ConstantReal {

    fn get_name(&self) -> String {
        String::from("Constant Real")
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
