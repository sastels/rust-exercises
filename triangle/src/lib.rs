pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let sum: u64 = sides.iter().sum();
        if sides.iter().any(|&x| x >= sum - x) {
            return None;
        }
        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        let min: u64 = *self.sides.iter().min().unwrap();
        self.sides.iter().all(|&x| x == min)
    }

    pub fn is_scalene(&self) -> bool {
        let min: u64 = *self.sides.iter().min().unwrap();
        let max: u64 = *self.sides.iter().max().unwrap();
        min != max && self.sides.iter().any(|&x| x != min && x != max)
    }

    pub fn is_isosceles(&self) -> bool {
        let min: u64 = *self.sides.iter().min().unwrap();
        let max: u64 = *self.sides.iter().max().unwrap();
        min != max && self.sides.iter().all(|&x| x == min || x == max)
    }
}
