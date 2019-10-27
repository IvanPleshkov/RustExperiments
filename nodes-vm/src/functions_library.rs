use crate::function::Function;
use std::rc::Rc;

pub trait FunctionFactory {
    fn create(&self) -> Rc<dyn Function>;

    fn namespace(&self) -> String;

    fn function_name(&self) -> String;
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

    pub fn create_function(&self, namespace: &str, function_name: &str) -> Option<Rc<dyn Function>> {
        for factory in &self.factories {
            if factory.namespace() == namespace && factory.function_name() == function_name {
                return Some(factory.create());
            }
        }
        None
    }

    pub fn add_factory(&mut self, factory: Box<dyn FunctionFactory>) {
        self.factories.push(factory);
    }
}
