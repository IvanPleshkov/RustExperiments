use crate::function;

pub struct ExecutionEnvironment {

}

pub trait Instruction {

    fn execute(env: &mut ExecutionEnvironment, );
}
