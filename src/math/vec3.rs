//use std::num;
use std::clone::Clone;
use std::ops::{Mul, Sub, Add, Div};

pub struct Vec3
{
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Vec3
{
    pub fn new(nx: f32, ny: f32, nz: f32) -> Vec3
    { Vec3 { x: nx, y:ny, z:nz } }

    pub fn zero() -> Vec3
    { Vec3 { x: 0.0, y:0.0, z:0.0 } }

    pub fn up() -> Vec3
    { Vec3 { x: 0.0, y:1.0, z:0.0 } }

    pub fn left() -> Vec3
    { Vec3 { x: 1.0, y:0.0, z:0.0 } }

    pub fn forward() -> Vec3
    { Vec3 { x: 0.0, y:0.0, z:1.0 } }

    pub fn new_normalized(vec: &Vec3) -> Vec3
    {
        let mut n = vec.clone();
        n.normalize();

        n
    }

    pub fn to_string(&self) -> String
    {
        format!("({}, {}, {})",
            self.x,
            self.y,
            self.z)
    }

    pub fn length(&self) -> f32
    {
        ((((self.x * self.x) +
           (self.y * self.y) +
           (self.z * self.z)) as f64).sqrt() as f32)
    }

    pub fn normalize(&mut self)
    {
        let mag: f32 = self.length();

        if ((mag as f64) - 0.0).abs() < 1.0e-6
        { return; }

        self.x /=  mag;
        self.y /=  mag;
        self.z /=  mag;
    }

    pub fn lerp(&self, rhs: &Vec3, t: f32) -> Vec3
    {
        let clamped = t.min(1.0).max(0.0);

        Vec3 {
            x: self.x + ((rhs.x - self.x) * clamped),
            y: self.x + ((rhs.y - self.y) * clamped),
            z: self.x + ((rhs.z - self.z) * clamped)
        }
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3
    {
        Vec3
        {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x)
        }
    }

    pub fn scale(&self, rhs: f32) -> Vec3
    {
        Vec3
        {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> f32
    {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn reflect(&self, rhs: &Vec3) -> Vec3
    {
        (rhs.scale(2.0 * rhs.dot(self))) - self.clone()
    }

    pub fn refract(&self, rhs: &Vec3, n1: f32, n2: f32, success: &mut bool) -> Vec3
    {
        *success &= false;

        let n = Vec3::new_normalized(rhs);
        let d = Vec3::new_normalized(self);

        let c1 = d.dot(&n);

        let ratio = n1 / n2;

        let end = 1.0 - (ratio.powi(2) * (1.0 - c1.powi(2)));

        if end < 0.0
        {
            return Vec3::zero();
        }

        *success |= true;

        ((d - (n.scale(c1 * ratio)))) - (n.scale(end.sqrt()))
    }

}

impl Mul<f32> for Vec3
{
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3
    {
        Vec3 {
            x: ( self.x * rhs ),
            y: ( self.y * rhs ),
            z: ( self.z * rhs )
        }
    }
}

impl Div<f32> for Vec3
{
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3
    {
        Vec3 {
            x: ( self.x / rhs ),
            y: ( self.y / rhs ),
            z: ( self.z / rhs )
        }
    }
}

impl Add for Vec3
{
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3
    {
        Vec3 {
            x: ( self.x + rhs.x ),
            y: ( self.y + rhs.y ),
            z: ( self.z + rhs.z )
        }
    }
}

impl Sub for Vec3
{
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3
    {
        Vec3 {
            x: ( self.x - rhs.x ),
            y: ( self.y - rhs.y ),
            z: ( self.z - rhs.z )
        }
    }
}

impl Clone for Vec3
{
    fn clone(&self) -> Vec3
    {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}