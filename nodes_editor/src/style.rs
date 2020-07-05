use nalgebra::Vector3;

pub struct Style {
    pub node_border_color: Vector3<f32>,
    pub node_border_thickness: f32,

    pub canvas_border_color: Vector3<f32>,
    pub canvas_color: Vector3<f32>,
}

impl Style {
    pub fn new() -> Style {
        Style {
            node_border_color: Vector3::new(1., 1., 1.),
            node_border_thickness: 3.,
            canvas_border_color: Vector3::new(1., 1., 1.),
            canvas_color: Vector3::new(0.2, 0.2, 0.2),
        }
    }
}
