//use std::num;
use std::clone::Clone;
use math::vec3::Vec3;

pub struct Vec4
{
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32,
}

impl Vec4
{
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4
    { Vec4 { x:x, y:y, z:z, w:w } }

    pub fn new_from_vec3(vec: Vec3, w: f32) -> Vec4
    { Vec4 { x:vec.x, y:vec.y, z:vec.z, w:w } }

    pub fn to_string(&self) -> String
    {
        format!("({}, {}, {}, {})",
            self.x,
            self.y,
            self.z,
            self.w)
    }
}

impl Clone for Vec4
{
    fn clone(&self) -> Vec4
    {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w
        }
    }
}