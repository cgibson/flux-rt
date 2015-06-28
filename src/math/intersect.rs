use math::vec3::Vec3;
use math::ray::Ray;

pub struct Intersect {
    pos: Vec3,
    normal: Vec3,
    mat: Material,
    ray: Ray
}