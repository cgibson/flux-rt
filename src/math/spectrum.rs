use std::ops::Add;

pub struct Spectrum
{
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32,
}

impl Spectrum
{
    pub fn new(nr: f32, ng: f32, nb: f32, na: f32) -> Spectrum
    { Spectrum { r: nr, g: ng, b: nb, a: na } }

    pub fn black() -> Spectrum
    { Spectrum { r: 0.0, g: 0.0, b: 0.0, a: 1.0 } }
}

impl Add for Spectrum
{
    type Output = Spectrum;
    fn add(self, rhs: Spectrum) -> Spectrum
    {
        Spectrum {
            r: ( self.r + rhs.r ),
            g: ( self.g + rhs.g ),
            b: ( self.b + rhs.b ),
            a: 1.0
        }
    }
}

impl Clone for Spectrum
{
    fn clone(&self) -> Spectrum
    {
        Spectrum {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a
        }
    }
}