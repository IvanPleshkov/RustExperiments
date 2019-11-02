use crate::function::Function;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub enum Variable {
    Null,
    Number(f64),
    String(String),
    Boolean(bool),
    Vector(Vec<Variable>),
    Object(HashMap<String, Variable>),
    Function(Rc<dyn Function>),
}

impl std::default::Default for Variable {
    fn default() -> Self {
        Variable::Null
    }
}
