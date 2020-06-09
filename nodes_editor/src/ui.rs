use crate::nodes_editor::{ NodesEditable };
// use nodes_engine::NodesDocument;
use editor::Editor;
use nalgebra::Vector2;

pub struct UiState {

}

pub fn nodes_editor_ui(ui: &imgui::Ui, ui_state: &mut UiState, editor: &mut Editor<NodesEditable>, size: Vector2<f32>) {
    let draw_list = ui.get_window_draw_list();
    let canvas_pos = ui.cursor_screen_pos();
    let canvas_pos = Vector2::new(canvas_pos[0], canvas_pos[1]);

    draw_cell_field(&draw_list, ui_state, editor, size, canvas_pos);
    draw_connections(&draw_list, ui_state, editor, size, canvas_pos);
    draw_nodes(&draw_list, ui_state, editor, size, canvas_pos);
    draw_editor_border(&draw_list, ui_state, editor, size, canvas_pos);

    ui.invisible_button(imgui::im_str!("canvas"), [ size.x, size.y ]);
}

fn draw_cell_field(
    draw_list: &imgui::WindowDrawList,
    _ui_state: &mut UiState,
    _editor: &mut Editor<NodesEditable>,
    size: Vector2<f32>,
    pos: Vector2<f32>,
) {
    const CANVAS_CORNER_COLOR1: [f32; 3] = [0.2, 0.2, 0.2];
    const CANVAS_CORNER_COLOR2: [f32; 3] = [0.2, 0.2, 0.24];
    const CANVAS_CORNER_COLOR3: [f32; 3] = [0.24, 0.24, 0.27];
    const CANVAS_CORNER_COLOR4: [f32; 3] = [0.2, 0.2, 0.24];
    draw_list.add_rect_filled_multicolor(
        [
            pos[0],
            pos[1],
        ],
        [
            pos[0] + size[0],
            pos[1] + size[1],
        ],
        CANVAS_CORNER_COLOR1,
        CANVAS_CORNER_COLOR2,
        CANVAS_CORNER_COLOR3,
        CANVAS_CORNER_COLOR4,
    );
}

fn draw_connections(
    _draw_list: &imgui::WindowDrawList,
    _ui_state: &mut UiState,
    _editor: &mut Editor<NodesEditable>,
    _size: Vector2<f32>,
    _pos: Vector2<f32>,
) {

}

fn draw_nodes(
    draw_list: &imgui::WindowDrawList,
    _ui_state: &mut UiState,
    editor: &mut Editor<NodesEditable>,
    ui_size: Vector2<f32>,
    ui_pos: Vector2<f32>,
) {
    const CANVAS_CORNER_COLOR1: [f32; 3] = [0.2, 0.2, 0.2];
    const CANVAS_CORNER_COLOR2: [f32; 3] = [0.2, 0.2, 0.24];
    const CANVAS_CORNER_COLOR3: [f32; 3] = [0.24, 0.24, 0.27];
    const CANVAS_CORNER_COLOR4: [f32; 3] = [0.2, 0.2, 0.24];

    let node_handlers_list = editor.get().document.get_nodes_list();
    for node_handler in node_handlers_list {
        let nodes_document = &editor.get().document;
        let camera = &editor.get().camera;

        let node_pos = nodes_document.get_node_position(node_handler);
        let node_size = Vector2::new(1., 1.);

        let node_pos = camera.pos_to_screen(&Vector2::new(node_pos[0], node_pos[1]));
        let node_pos = node_pos + Vector2::new(ui_size[0] / 2., ui_size[1] / 2.);
        let node_size = node_size * camera.scale();

        draw_list.add_rect_filled_multicolor(
            [
                node_pos[0] + ui_pos[0] + ui_size[0] / 2.,
                node_pos[1] + ui_pos[1] + ui_size[1] / 2.,
            ],
            [
                node_pos[0] + node_size[0],
                node_pos[1] + node_size[1],
            ],
            CANVAS_CORNER_COLOR1,
            CANVAS_CORNER_COLOR2,
            CANVAS_CORNER_COLOR3,
            CANVAS_CORNER_COLOR4,
        );
    }
}

fn draw_editor_border(
    draw_list: &imgui::WindowDrawList,
    _ui_state: &mut UiState,
    _editor: &mut Editor<NodesEditable>,
    size: Vector2<f32>,
    pos: Vector2<f32>,
) {
    const CANVAS_BORDER_COLOR: [f32; 3] = [1.0, 1.0, 1.0];
    draw_list
        .add_rect(
            [
                pos[0],
                pos[1],
            ],
            [
                pos[0] + size[0],
                pos[1] + size[1],
            ],
            CANVAS_BORDER_COLOR,
        ).build();
}
