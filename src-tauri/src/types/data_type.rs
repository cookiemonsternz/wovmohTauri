use super::*;

pub enum PinValue {
    Number(f64),
    Boolean(bool),
    Color(color::Color),
    Vector3(vector::Vec3),
    Point3(vector::Point3)
}