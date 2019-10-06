pub struct TraceMessage {
    pub exit_message: &'static str,
}

impl TraceMessage {
    pub fn new(enter_message: &'static str, exit_message: &'static str) -> Self {
        println!("{}", enter_message);
        TraceMessage {
            exit_message: exit_message,
        }
    }
}

impl Drop for TraceMessage {
    fn drop(&mut self) {
        println!("{}", self.exit_message);
    }
}

#[macro_export]
macro_rules! trace {
    // `()` indicates that the macro takes no argument.
    ($struct_name:expr, $method_name:expr) => {
        let _trace_variable = TraceMessage::new(
            concat!(
                "ENTER: ",
                std::file!(),
                "(",
                std::line!(),
                "), ",
                $struct_name,
                "::",
                $method_name
            ),
            concat!(
                "LEAVE: ",
                std::file!(),
                "(",
                std::line!(),
                "), ",
                $struct_name,
                "::",
                $method_name
            ),
        );
    };
}
