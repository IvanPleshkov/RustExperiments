use std::any::Any;

pub struct HandleEventResult {
    pub event_handled: bool,
    pub action_finished: bool,
    pub set_modified_flag: bool,
    pub push_to_undo_stack: bool,
}

pub trait Action<T> {
    fn get_name(&self) -> &str;

    fn handle_event(&mut self, editable: &mut T, event: &dyn Any) -> HandleEventResult;

    fn undo(&self, editable: &mut T);

    fn redo(&self, editable: &mut T);
}

pub trait ActionFabric<T> {
    fn start_action(&self, event: &dyn Any) -> Option<Box<dyn Action<T>>>;
}
