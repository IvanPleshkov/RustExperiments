use crate::function::Exception;
use crate::function::Function;
use crate::functions_library::FunctionFactory;
use crate::variable::Variable;
use crate::vm::Vm;
use semver;
use std::rc::Rc;

pub enum Instruction {
    Copy(usize, usize),
    MoveFromInput(usize, usize),
    MoveToOutput(usize, usize),
    JumpIf(usize, usize, usize),
    Jump(usize),
    Invoke(Rc<dyn Function>, usize, usize),
}

struct Program {
    pub name: String,
    pub inputs_len: u32,
    pub outputs_len: u32,
    pub registry_size: u32,
    pub instructions: Vec<Instruction>,
}

struct ProgramFactory {}

impl FunctionFactory for ProgramFactory {
    fn create(&self) -> Rc<dyn Function> {
        Rc::new(Program {
            name: String::new(),
            registry_size: 0,
            inputs_len: 0,
            outputs_len: 0,
            instructions: Vec::new(),
        })
    }

    fn namespace(&self) -> String {
        String::from("standard")
    }

    fn function_name(&self) -> String {
        String::from("Custom Program")
    }

    fn version(&self) -> semver::Version {
        semver::Version::parse("0.0.1").unwrap()
    }
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
        let mut registry: Vec<Variable> = Vec::new();
        registry.resize_with(self.registry_size as usize, || Variable::Null);

        let mut cursor_position: usize = 0;
        while cursor_position < self.instructions.len() {
            match &self.instructions[cursor_position] {
                Instruction::Copy(from_index, to_index) => {
                    if *from_index >= registry.len() || *to_index >= registry.len() {
                        return Err(Exception {
                            message: String::from("Wrong program in Instruction::Copy"),
                        });
                    }
                    registry[*to_index] = registry[*from_index].clone();
                    cursor_position += 1;
                }
                Instruction::MoveFromInput(input_index, registry_index) => {
                    if *input_index >= inputs.len() || *registry_index >= registry.len() {
                        return Err(Exception {
                            message: String::from("Wrong program in Instruction::MoveFromInput"),
                        });
                    }
                    registry[*registry_index] = inputs[*input_index].clone();
                    cursor_position += 1;
                }
                Instruction::MoveToOutput(registry_index, output_index) => {
                    if *output_index >= outputs.len() || *registry_index >= registry.len() {
                        return Err(Exception {
                            message: String::from("Wrong program in Instruction::MoveToOutput"),
                        });
                    }
                    outputs[*output_index] = registry[*registry_index].clone();
                    cursor_position += 1;
                }
                Instruction::JumpIf(registry_index, true_position, false_position) => {
                    if *registry_index >= registry.len() {
                        return Err(Exception {
                            message: String::from("Wrong program in Instruction::JumpIf"),
                        });
                    }
                    match &registry[*registry_index] {
                        Variable::Boolean(value) => {
                            if *value {
                                cursor_position = *true_position;
                            } else {
                                cursor_position = *false_position;
                            }
                        }
                        _ => {
                            return Err(Exception {
                                message: String::from("Instruction::JumpIf: value is not bool"),
                            });
                        }
                    };
                }
                Instruction::Jump(position) => {
                    cursor_position = *position;
                }
                Instruction::Invoke(function, inputs_start, outputs_start) => {
                    if *inputs_start >= registry.len() || *outputs_start >= registry.len() {
                        return Err(Exception {
                            message: String::from("Wrong program in Instruction::Invoke"),
                        });
                    }

                    let mut input: Vec<Variable> = Vec::new();
                    for i in 0..function.inputs_len() {
                        input.push(registry[inputs_start + i].clone());
                    }

                    let mut output: Vec<Variable> = Vec::new();
                    output.resize_with(function.outputs_len(), || Variable::Null);
                    function.run(vm, input.as_mut_slice(), output.as_mut_slice())?;

                    for i in 0..function.outputs_len() {
                        outputs[outputs_start + i] = outputs[i].clone();
                    }
                    cursor_position += 1;
                }
            };
        }

        Ok(())
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn inputs_len(&self) -> usize {
        self.inputs_len as usize
    }

    fn outputs_len(&self) -> usize {
        self.outputs_len as usize
    }

    fn serialize(&self) -> serde_json::Value {
        std::unimplemented!();
    }

    fn deserialize(&mut self, json: &serde_json::Value) {
        std::unimplemented!();
    }
}
