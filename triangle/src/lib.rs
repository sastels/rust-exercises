use std::ops::{Add, Sub};

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Copy + PartialOrd,
    T: Add<Output = T>,
    <T as Add>::Output: Sub<Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let sum: T = sides[0] + sides[1] + sides[2];
        if sides.iter().any(|&x| x >= sum - x) {
            return None;
        }
        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
    }
}
