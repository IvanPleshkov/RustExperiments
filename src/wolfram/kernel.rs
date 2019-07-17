use wolfram::expression::Expression;
use std::sync::mpsc;

pub enum Message {
    Pause,
    Resume,
    Stop,
}

pub trait Kernel {
    fn evaluate(&mut self, e: &Expression) -> Expression;

    fn get_message_sender(&self) -> mpsc::Sender<Message>;

    fn send_message(&self, message: Message);

    fn last_message(&mut self) -> Message;
    
    fn wait_for_message(&mut self) -> Message;

    fn is_stopped(&mut self) -> bool {
        if let Message::Stop = self.last_message() {
            true
        } else {
            false
        }
    }
}
