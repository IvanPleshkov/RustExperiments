use crate::function::Function;
use crate::function::Exception;
use crate::functions_library::FunctionFactory;
use crate::vm::Vm;
use crate::variable::Variable;
use std::rc::Rc;

enum ProgramInstruction {
    Copy(u32, u32),
    MoveFromInput(u32, u32),
    MoveToOutput(u32, u32),
    Invoke(Rc<dyn Function>),
    JumpIf(u32, u32, u32),
}

struct Program {
    pub name: String,
    pub registry_size: u32,
    pub instructions: Vec<ProgramInstruction>,
}

struct ProgramFactory {
}

impl FunctionFactory for ProgramFactory {
    fn create(&self) -> Rc<dyn Function> {
        Rc::new(Program{ 
            name: String::new(),
            registry_size: 0,
            instructions: Vec::new(),
        })
    }

    fn namespace(&self) -> String {
        String::from("standard")
    }

    fn function_name(&self) -> String {
        String::from("Custom Program")
    }
}

impl Program {

}

impl Clone for Program {
    fn clone(&self) -> Self {
        std::unimplemented!();
    }
}

impl Function for Program {
    fn run(
        &self,
        vm: &mut Vm,
        inputs: &mut [Variable],
        outputs: &mut [Variable],
    ) -> Result<(), Exception> {
        std::unimplemented!();
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn inputs_len(&self) -> usize {
        std::unimplemented!();
    }

    fn outputs_len(&self) -> usize {
        std::unimplemented!();
    }

    fn serialize(&self) -> serde_json::Value {
        std::unimplemented!();
    }

    fn deserialize(&mut self, json: &serde_json::Value) {
        std::unimplemented!();
    }
}
