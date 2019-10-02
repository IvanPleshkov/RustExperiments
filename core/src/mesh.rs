use cgmath::BaseFloat;
use cgmath::Point2;
use cgmath::Point3;
use cgmath::Vector3;

pub struct Mesh<T: BaseFloat> {
    pub triangles: Vector3<i32>,
    pub positions: Point3<T>,
    pub normals: Vector3<T>,
    pub binormals: Vector3<T>,
    pub tangents: Vector3<T>,
    pub uv: Point2<T>,
    pub uv2: Point2<T>,
    pub uv3: Point2<T>,
    pub uv4: Point2<T>,
}

// pub use Mesh<f32> as Mesh32;
// pub use Mesh<f64> as Mesh64;
