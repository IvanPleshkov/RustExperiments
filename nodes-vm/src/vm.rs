use crate::function::Exception;
use crate::function::Function;
use crate::variable::Variable;

pub struct Vm {}

impl Vm {
    pub fn run(
        &mut self,
        function: &dyn Function,
        inputs: &[Variable],
    ) -> Result<Vec<Variable>, Exception> {


        //function.run(self, inputs: &mut [Variable], outputs: &mut [Variable])
        Ok(vec![])
    }
}
