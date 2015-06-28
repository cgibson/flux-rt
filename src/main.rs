use math::vec3::Vec3;
use math::mat4::Mat4;
use math::spectrum::Spectrum;
use math::ray::Ray;

pub mod math;


fn main() {
    let v = Vec3::new(0.0, 2.0, 0.0);
    println!("Hello, world! {}", v.to_string());

    let v2 =  Vec3::new_normalized(&v);
    println!("Hello, world! {}", v2.to_string());

    let c = Spectrum::new(0.0, 1.0, 0.0, 0.0);
    println!("Hello, world! {}", c.g);

    let mat = Mat4::new();
    println!("{}", mat.to_string() );

    let ray = Ray::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0));

    println!("{}", ray.to_string());
}
