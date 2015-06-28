use math::vec3::Vec3;
use math::spectrum::Spectrum;

pub struct Ray
{
    pub pos: Vec3,
    pub dir: Vec3,
    pub far: f32,
    pub transmittance: Spectrum,
}

impl Ray
{
    pub fn new(pos: Vec3, dir: Vec3) -> Ray
    {
        Ray { 
            pos: pos, 
            dir: dir, 
            far: 1000.0, 
            transmittance: Spectrum::black()
        }
    }

    pub fn to_string(&self) -> String
    {
        format!(
            "Ray\n\
            \tPos: {}\n\
            \tDir: {}",
            self.pos.to_string(),
            self.dir.to_string())
    }
}