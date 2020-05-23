pub struct TracePoint {
    pub exit_message: &'static str,
}

impl TracePoint {
    pub fn new(enter_message: &'static str, exit_message: &'static str) -> Self {
        println!("{}", enter_message);
        TracePoint {
            exit_message: exit_message,
        }
    }
}

impl Drop for TracePoint {
    fn drop(&mut self) {
        println!("{}", self.exit_message);
    }
}

#[macro_export]
macro_rules! trace_point {
    ($function_name: expr, $trait_name: expr, $struct_name: expr) => {
        let __trace_point = TracePoint::new(
            concat!(
                "ENTER: ",
                std::file!(),
                ":",
                std::line!(),
                ", ",
                $function_name,
                ", ",
                $trait_name,
                ", ",
                struct_name,
            ),
            concat!(
                "LEAVE: ",
                std::file!(),
                ":",
                std::line!(),
                ", ",
                $function_name,
                ", ",
                $trait_name,
                ", ",
                struct_name,
            ),
        );
    };
}
