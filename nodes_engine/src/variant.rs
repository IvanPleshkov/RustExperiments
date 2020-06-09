use nalgebra;

pub enum Variant {
    Null,
    Boolean(bool),
    IntegerNumber(i64),
    RealNumber(f64),
    Vector2(nalgebra::Vector2<f64>),
    Vector3(nalgebra::Vector3<f64>),
    Vector4(nalgebra::Vector4<f64>),
    RowVector2(nalgebra::Vector2<f64>),
    RowVector3(nalgebra::Vector3<f64>),
    RowVector4(nalgebra::Vector4<f64>),
    Matrix2(nalgebra::Matrix2<f64>),
    Matrix3(nalgebra::Matrix3<f64>),
    Matrix4(nalgebra::Matrix4<f64>),
    Quaternion(nalgebra::Quaternion<f64>),
}

pub enum VariantType {
    Null,
    Boolean,
    IntegerNumber,
    RealNumber,
    Vector2,
    Vector3,
    Vector4,
    RowVector2,
    RowVector3,
    RowVector4,
    Matrix2,
    Matrix3,
    Matrix4,
    Quaternion,
}

impl Variant {
    pub fn get_type(&self) -> VariantType {
        match self {
            Variant::Null => VariantType::Null,
            Variant::Boolean(_) => VariantType::Boolean,
            Variant::IntegerNumber(_) => VariantType::IntegerNumber,
            Variant::RealNumber(_) => VariantType::RealNumber,
            Variant::Vector2(_) => VariantType::Vector2,
            Variant::Vector3(_) => VariantType::Vector3,
            Variant::Vector4(_) => VariantType::Vector4,
            Variant::RowVector2(_) => VariantType::RowVector2,
            Variant::RowVector3(_) => VariantType::RowVector3,
            Variant::RowVector4(_) => VariantType::RowVector4,
            Variant::Matrix2(_) => VariantType::Matrix2,
            Variant::Matrix3(_) => VariantType::Matrix3,
            Variant::Matrix4(_) => VariantType::Matrix4,
            Variant::Quaternion(_) => VariantType::Quaternion,
        }
    }
}
