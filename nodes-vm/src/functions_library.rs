use crate::function::Function;
use crate::vm::Vm;
use semver;
use std::rc::Rc;

pub trait FunctionFactory {
    fn create(&self) -> Rc<dyn Function>;

    fn deserialize(&self, vm: &Vm, json: &serde_json::Value) -> Result<Rc<dyn Function>, ()>;

    fn library_name(&self) -> String;

    fn function_name(&self) -> String;

    fn version(&self) -> semver::Version;
}

pub struct FunctionsLibrary {
    pub factories: Vec<Box<dyn FunctionFactory>>,
}

impl FunctionsLibrary {
    pub fn new() -> FunctionsLibrary {
        FunctionsLibrary {
            factories: Vec::new(),
        }
    }

    pub fn create_function(
        &self,
        library_name: &str,
        function_name: &str,
    ) -> Option<Rc<dyn Function>> {
        for factory in &self.factories {
            if factory.library_name() == library_name && factory.function_name() == function_name {
                return Some(factory.create());
            }
        }
        None
    }

    pub fn deserialize_function(
        &self,
        vm: &Vm,
        json: &serde_json::Value,
    ) -> Option<Rc<dyn Function>> {
        let name = json.get("name")?.as_str()?;
        let library = json.get("library")?.as_str()?;
        for factory in &self.factories {
            if factory.library_name() == library && factory.function_name() == name {
                if let Ok(function) = factory.deserialize(vm, json) {
                    return Some(function);
                }
            }
        }
        None
    }

    pub fn add_factory(&mut self, factory: Box<dyn FunctionFactory>) {
        self.factories.push(factory);
    }
}
