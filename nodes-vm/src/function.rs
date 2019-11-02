use crate::variable::Variable;
use crate::vm::Vm;
use serde_json;
use semver;

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

    fn library_name(&self) -> String;

    fn version(&self) -> semver::Version;

    fn inputs_len(&self) -> usize;

    fn outputs_len(&self) -> usize;

    fn serialize(&self) -> serde_json::Value;

    fn deserialize(&mut self, vm: &Vm, json: &serde_json::Value) -> Result<(), ()>;
}
