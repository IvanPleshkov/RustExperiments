use std::any::Any;

pub struct HandleEventResult {
    pub event_handled: bool,
    pub action_finished: bool,
}

pub trait Action<T> {
    fn get_name(&self) -> &str;

    fn handle_event(&mut self, editable: &mut T, event: &dyn Any) -> HandleEventResult;

    fn undo(&self, editable: &mut T);

    fn redo(&self, editable: &mut T);

    fn need_push_to_undo_stack(&self);

    fn need_set_modified_flag(&self);
}

pub trait ActionFabric<T> {
    fn start_action(&self, event: &dyn Any) -> Option<Box<dyn Action<T>>>;
}
