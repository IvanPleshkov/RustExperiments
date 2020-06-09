use nalgebra::Vector2;

pub struct Camera {

}

impl Camera {
    pub fn pos_to_screen(&self, pos: &Vector2<f32>) -> Vector2<f32> {
        *pos
    }

    pub fn scale(&self) -> f32 {
        1.
    }
}
