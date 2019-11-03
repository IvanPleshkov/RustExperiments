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
        inputs: &mut [&mut Variable],
        outputs: &mut [&mut Variable],
    ) -> Result<(), Exception>;

    fn name(&self) -> String;

    fn library_name(&self) -> String;

    fn version(&self) -> semver::Version;

    fn inputs_len(&self) -> usize;

    fn outputs_len(&self) -> usize;

    fn serialize(&self) -> serde_json::Value;

    fn deserialize(&mut self, vm: &Vm, json: &serde_json::Value) -> Result<(), ()>;

    fn serialize_common_function_data(&self) -> serde_json::Value {
        use serde_json::*;
        let mut map : Map<String, Value> = Map::new();
        map.insert(String::from("version"), to_value(format!("{}", self.version())).unwrap());
        map.insert(String::from("name"), to_value(self.name()).unwrap());
        map.insert(String::from("library"), to_value(self.library_name()).unwrap());
        serde_json::Value::Object(map)
    }
}
