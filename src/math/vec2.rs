
pub struct Vec2
{
    pub x : f32,
    pub y : f32
}

impl Vec2
{
    pub fn new(nx: f32, ny: f32) -> Vec2
    { Vec2 { x: nx, y:ny } }
}