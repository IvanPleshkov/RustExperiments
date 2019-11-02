use crate::variable::Variable;
use crate::vm::Vm;
use serde_json;

pub struct Exception {
    pub message: String,
}

pub trait Function {
    fn run(
        &self,
        vm: &mut Vm,
        inputs: &mut [Variable],
        outputs: &mut [Variable],
    ) -> Result<(), Exception>;

    fn name(&self) -> String;

    fn inputs_len(&self) -> usize;

    fn outputs_len(&self) -> usize;

    fn serialize(&self) -> serde_json::Value;

    fn deserialize(&mut self, json: &serde_json::Value);
}
