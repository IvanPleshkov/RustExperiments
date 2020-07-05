use crate::nodes_editor::{ NodesEditable };
use crate::style::Style;
use nodes_engine::NodesDocument;
use editor::Editor;
use nalgebra::Vector2;

pub struct UiState {
    pub style: Style,
}

pub fn nodes_editor_ui(
    ui: &imgui::Ui,
    ui_state: &mut UiState,
    editor: &mut Editor<NodesEditable>,
    canvas_pos: Vector2<f32>,
    canvas_size: Vector2<f32>,
) {
    let draw_list = ui.get_window_draw_list();

    draw_cell_field(&draw_list, ui_state, editor, canvas_pos, canvas_size);
    draw_connections(&draw_list, ui_state, editor, canvas_pos, canvas_size);
    draw_nodes(&draw_list, ui_state, editor, canvas_pos, canvas_size);
    draw_editor_border(&draw_list, ui_state, editor, canvas_pos, canvas_size);

    ui.invisible_button(imgui::im_str!("canvas"), [ canvas_size.x, canvas_size.y ]);
}

fn draw_cell_field(
    draw_list: &imgui::WindowDrawList,
    ui_state: &mut UiState,
    _editor: &mut Editor<NodesEditable>,
    pos: Vector2<f32>,
    size: Vector2<f32>,
) {
    let canvas_color: [f32; 3] = [
        ui_state.style.canvas_color[0],
        ui_state.style.canvas_color[1],
        ui_state.style.canvas_color[2],
    ];
    draw_list.add_rect_filled_multicolor(
        [
            pos[0],
            pos[1],
        ],
        [
            pos[0] + size[0],
            pos[1] + size[1],
        ],
        canvas_color,
        canvas_color,
        canvas_color,
        canvas_color,
    );
}

fn draw_connections(
    _draw_list: &imgui::WindowDrawList,
    _ui_state: &mut UiState,
    _editor: &mut Editor<NodesEditable>,
    _pos: Vector2<f32>,
    _size: Vector2<f32>,
) {

}

fn draw_nodes(
    draw_list: &imgui::WindowDrawList,
    ui_state: &mut UiState,
    editor: &Editor<NodesEditable>,
    ui_pos: Vector2<f32>,
    ui_size: Vector2<f32>,
) {
    let nodes_document = &editor.get().document;
    let camera = &editor.get().camera;
    for node_handler in nodes_document.get_nodes_list() {
        let node_pos = nodes_document.get_node_position(node_handler);
        let node_pos = camera.pos_to_screen(&Vector2::new(node_pos[0], node_pos[1]));
        let node_pos = node_pos + Vector2::new(ui_pos[0], ui_pos[1]);
        let node_pos = node_pos + Vector2::new(ui_size[0] / 2., ui_size[1] / 2.);

        let node_size = Vector2::new(100., 100.);
        let node_size = node_size * camera.scale();

        draw_node_border(draw_list, ui_state, editor, node_pos, node_size);
    }
}

fn draw_node_border(
    draw_list: &imgui::WindowDrawList,
    ui_state: &mut UiState,
    _editor: &Editor<NodesEditable>,
    node_pos: Vector2<f32>,
    node_size: Vector2<f32>,
) {
    draw_list
        .add_rect(
            [
                node_pos[0],
                node_pos[1],
            ],
            [
                node_pos[0] + node_size[0],
                node_pos[1] + node_size[1],
            ],
            [
                ui_state.style.node_border_color[0],
                ui_state.style.node_border_color[1],
                ui_state.style.node_border_color[2],
            ],
        )
        .thickness(ui_state.style.node_border_thickness)
        .build();
}

fn draw_editor_border(
    draw_list: &imgui::WindowDrawList,
    ui_state: &mut UiState,
    _editor: &mut Editor<NodesEditable>,
    pos: Vector2<f32>,
    size: Vector2<f32>,
) {
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
            [
            ui_state.style.canvas_border_color[0],
            ui_state.style.canvas_border_color[1],
            ui_state.style.canvas_border_color[2],
        ],
        ).build();
}
