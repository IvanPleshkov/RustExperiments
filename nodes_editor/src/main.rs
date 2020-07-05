use crate::nodes_editor::{ NodesEditable, NodesSelection, NodesEditorActionFabric };
use crate::camera::Camera;
use crate::style::Style;
use nodes_engine::{  NodesDocumentImpl, NodesDocument };
use imgui_window;
use editor::Editor;
use nalgebra::Vector2;

mod camera;
mod nodes_editor;
mod style;
mod ui;

fn new_nodes_document(document: &mut NodesDocumentImpl) {
    let handler = document.add_node(nodes_engine::std_nodes::constant_boolean::ConstantBoolean::new());
    document.set_node_position(handler, Vector2::new(0., 0.));
}

fn new_nodes_editor() -> Editor<NodesEditable> {
    let mut editable = NodesEditable {
        document: NodesDocumentImpl::new(),
        selection: NodesSelection{},
        camera: Camera{},
    };
    new_nodes_document(&mut editable.document);
    let action_fabric = NodesEditorActionFabric{};
    Editor::new(editable, Box::new(action_fabric))
}

fn gui_loop_tick(ui: &imgui::Ui, ui_state: &mut ui::UiState, editor: &mut Editor<NodesEditable>) {
    imgui::Window::new(imgui::im_str!("Hello world"))
    .size([800.0, 500.0], imgui::Condition::FirstUseEver)
    .build(ui, || {
        ui.text(imgui::im_str!("Nodes engine"));
        ui.text(imgui::im_str!("This...is...imgui-rs!"));
        ui.separator();
        let mouse_pos = ui.io().mouse_pos;
        ui.text(format!(
            "Mouse Position: ({:.1},{:.1})",
            mouse_pos[0], mouse_pos[1]
        ));

        let canvas_pos = ui.cursor_screen_pos();
        let canvas_pos = Vector2::new(canvas_pos[0], canvas_pos[1]);
        let canvas_size = {
            let mut canvas_size = ui.content_region_avail();
            if canvas_size[0] < 100.0 {
                canvas_size[0] = 100.0;
            }
            if canvas_size[1] < 100.0 {
                canvas_size[1] = 100.0;
            }
            Vector2::new(canvas_size[0], canvas_size[1])
        };
        ui::nodes_editor_ui(ui, ui_state, editor, canvas_pos, canvas_size);
    });
}

fn main() {
    let mut editor = new_nodes_editor();
    let mut nodes_ui_state = ui::UiState {
        style: Style::new(),
    };

    let system = imgui_window::init(file!());
    system.main_loop(|_, ui| {
        gui_loop_tick(ui, &mut nodes_ui_state, &mut editor);
    });
}
