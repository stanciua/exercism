#![feature(slice_patterns)]

pub struct Triangle {
    x: i32,
    y: i32,
    z: i32,
}

impl Triangle {
    pub fn build(sides: [i32; 3]) -> Result<Triangle, &'static str> {
        let [x, y, z] = sides;
        if x + y < z || x + z < y || y + z < x || x == 0 || y == 0 || z == 0 {
            return Err("Not a Triangle!");
        }

        Ok(Triangle { x: x, y: y, z: z })
    }

    pub fn is_equilateral(&self) -> bool {
        self.x == self.y && self.y == self.z && self.x == self.z
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_equilateral() && (self.x == self.y || self.y == self.z || self.x == self.z)
    }

    pub fn is_scalene(&self) -> bool {
        // self.x != self.y && self.x != self.z && self.y != self.z
        !self.is_isosceles() && !self.is_equilateral()
    }
}
