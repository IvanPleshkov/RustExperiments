use crate::function::Exception;
use crate::function::Function;
use crate::variable::Variable;
use common::trace;
use common::trace::*;

pub struct Vm {}

impl Vm {
    pub fn run(
        &mut self,
        function: &dyn Function,
        inputs: &[Variable],
    ) -> Result<Vec<Variable>, Exception> {
        trace!("Vm", "run");

        let mut inputs_copy: Vec<Variable> = Vec::new();
        for variable in inputs {
            inputs_copy.push(variable.clone());
        }
        let mut result: Vec<Variable> = Vec::new();
        result.resize_with(function.outputs_len(), || Variable::Null);
        function.run(self, inputs_copy.as_mut_slice(), result.as_mut_slice())?;
        Ok(result)
    }
}
