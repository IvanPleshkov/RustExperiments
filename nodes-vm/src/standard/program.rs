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

pub struct Program {
    pub inputs_len: u32,
    pub outputs_len: u32,
    pub registry_size: u32,
    pub instructions: Vec<Instruction>,
}

struct ProgramFactory {}

impl Program {
    pub fn clear(&mut self) {
        self.inputs_len = 0;
        self.outputs_len = 0;
        self.registry_size = 0;
        self.instructions.clear();
    }

    fn deserialize_impl(&mut self, vm: &Vm, json: &serde_json::Value) -> Option<()> {
        self.clear();
        let map = json.as_object()?;
        self.inputs_len = map.get("inputs_len")?.as_u64()? as u32;
        self.outputs_len = map.get("inputs_len")?.as_u64()? as u32;
        self.registry_size = map.get("registry_size")?.as_u64()? as u32;

        let json_instructions_array = json.get("instructions")?.as_array()?;
        for instruction_json in json_instructions_array {
            let instruction_json = instruction_json.as_object()?;
            let instruction_type = instruction_json.get("type")?.as_str()?;

            let instruction = if instruction_type == "copy" {
                let from_index = instruction_json.get("from_index")?.as_u64()? as usize;
                let to_index = instruction_json.get("to_index")?.as_u64()? as usize;
                Instruction::Copy(from_index, to_index)
            } else if instruction_type == "move_from_input" {
                let input_index = instruction_json.get("input_index")?.as_u64()? as usize;
                let registry_index = instruction_json.get("registry_index")?.as_u64()? as usize;
                Instruction::MoveFromInput(input_index, registry_index)
            } else if instruction_type == "move_to_output" {
                let registry_index = instruction_json.get("registry_index")?.as_u64()? as usize;
                let output_index = instruction_json.get("output_index")?.as_u64()? as usize;
                Instruction::MoveToOutput(registry_index, output_index)
            } else if instruction_type == "jump_if" {
                let registry_index = instruction_json.get("registry_index")?.as_u64()? as usize;
                let true_position = instruction_json.get("true_position")?.as_u64()? as usize;
                let false_position = instruction_json.get("false_position")?.as_u64()? as usize;
                Instruction::JumpIf(registry_index, true_position, false_position)
            } else if instruction_type == "jump" {
                let position = instruction_json.get("position")?.as_u64()? as usize;
                Instruction::Jump(position)
            } else if instruction_type == "invoke" {
                let inputs_start = instruction_json.get("inputs_start")?.as_u64()? as usize;
                let outputs_start = instruction_json.get("outputs_start")?.as_u64()? as usize;
                let function_json = instruction_json.get("function_data")?;
                let function = vm.functions_factory.deserialize_function(vm, function_json)?;
                Instruction::Invoke(function, inputs_start, outputs_start)
            } else {
                return None;
            };
            self.instructions.push(instruction);
        }
        Some(())
    }
}

impl FunctionFactory for ProgramFactory {
    fn create(&self) -> Rc<dyn Function> {
        Rc::new(Program {
            registry_size: 0,
            inputs_len: 0,
            outputs_len: 0,
            instructions: Vec::new(),
        })
    }

    fn deserialize(&self, vm: &Vm, json: &serde_json::Value) -> Result<Rc<dyn Function>, ()> {
        let mut program = Program {
            registry_size: 0,
            inputs_len: 0,
            outputs_len: 0,
            instructions: Vec::new(),
        };
        program.deserialize(vm, json)?;
        Ok(Rc::new(program))
    }

    fn library_name(&self) -> String {
        String::from("standard")
    }

    fn function_name(&self) -> String {
        String::from("program")
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
                        },
                        _ => {
                            return Err(Exception {
                                message: String::from("Instruction::JumpIf: value is not bool"),
                            });
                        },
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

    fn library_name(&self) -> String {
        String::from("standard")
    }

    fn name(&self) -> String {
        String::from("program")
    }

    fn version(&self) -> semver::Version {
        semver::Version::parse("0.0.1").unwrap()
    }

    fn inputs_len(&self) -> usize {
        self.inputs_len as usize
    }

    fn outputs_len(&self) -> usize {
        self.outputs_len as usize
    }

    fn serialize(&self) -> serde_json::Value {
        use serde_json::*;

        let mut instructions_json : Vec<Value> = Vec::new();
        for instruction in &self.instructions {
            let mut map : Map<String, Value> = Map::new();
            match instruction {
                Instruction::Copy(from_index, to_index) => {
                    map.insert(String::from("type"), Value::String(String::from("copy")));
                    map.insert(String::from("from_index"), to_value(from_index).unwrap());
                    map.insert(String::from("to_index"), to_value(to_index).unwrap());
                },
                Instruction::MoveFromInput(input_index, registry_index) => {
                    map.insert(String::from("type"), Value::String(String::from("move_from_input")));
                    map.insert(String::from("input_index"), to_value(input_index).unwrap());
                    map.insert(String::from("registry_index"), to_value(registry_index).unwrap());
                },
                Instruction::MoveToOutput(registry_index, output_index) => {
                    map.insert(String::from("type"), Value::String(String::from("move_to_output")));
                    map.insert(String::from("registry_index"), to_value(registry_index).unwrap());
                    map.insert(String::from("output_index"), to_value(output_index).unwrap());
                },
                Instruction::JumpIf(registry_index, true_position, false_position) => {
                    map.insert(String::from("type"), Value::String(String::from("jump_if")));
                    map.insert(String::from("registry_index"), to_value(registry_index).unwrap());
                    map.insert(String::from("true_position"), to_value(true_position).unwrap());
                    map.insert(String::from("false_position"), to_value(false_position).unwrap());
                },
                Instruction::Jump(position) => {
                    map.insert(String::from("type"), Value::String(String::from("jump")));
                    map.insert(String::from("position"), to_value(position).unwrap());
                },
                Instruction::Invoke(function, inputs_start, outputs_start) => {
                    map.insert(String::from("type"), Value::String(String::from("invoke")));
                    map.insert(String::from("inputs_start"), to_value(inputs_start).unwrap());
                    map.insert(String::from("outputs_start"), to_value(outputs_start).unwrap());
                    map.insert(String::from("function_data"), function.serialize());
                },
            };
            instructions_json.push(Value::Object(map));
        }

        let mut map : Map<String, Value> = Map::new();
        map.insert(String::from("version"), to_value(format!("{}", self.version())).unwrap());
        map.insert(String::from("instructions"), Value::Array(instructions_json));
        map.insert(String::from("name"), to_value(self.name()).unwrap());
        map.insert(String::from("library"), to_value(self.library_name()).unwrap());
        map.insert(String::from("inputs_len"), to_value(self.inputs_len).unwrap());
        map.insert(String::from("outputs_len"), to_value(self.outputs_len).unwrap());
        map.insert(String::from("registry_size"), to_value(self.registry_size).unwrap());
        serde_json::Value::Object(map)
    }

    fn deserialize(&mut self, vm: &Vm, json: &serde_json::Value) -> Result<(), ()> {
        if let Some(_) = self.deserialize_impl(vm, json) {
            Ok(())
        } else {
            Err(())
        }
    }
}
