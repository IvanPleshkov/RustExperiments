use nalgebra::{Vector2, Vector3, Vector4};

pub struct Mesh {
    pub indices: Vec<u32>,
    pub positions: Vec<Vector3<f32>>,
    pub normals: Vec<Vector3<f32>>,
    pub tangents: Vec<Vector3<f32>>,
    pub bitangents: Vec<Vector3<f32>>,
    pub colors: [Vec<Vector4<f32>>; 4],
    pub uvs: [Vec<Vector2<f32>>; 8],
}
