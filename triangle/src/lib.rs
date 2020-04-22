pub struct Triangle {
    side_1: u64,
    side_2: u64,
    side_3: u64
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        pub fn are_sides_valid(sides: [u64; 3]) -> bool {
            // all sides > 0
            let positive_sides: bool = sides[0] > 0 && sides[1] > 0 && sides[2] > 0;
            let sum_of_2_sides: bool = (sides[0] + sides[1] >= sides[2]) &&
                (sides[1] + sides[2] >= sides[0]) &&
                (sides[0] + sides[2] >= sides[1]);

            positive_sides && sum_of_2_sides
        }

        match are_sides_valid(sides) {
            true => Some(Triangle {
                side_1: sides[0],
                side_2: sides[1],
                side_3: sides[2]
            }),
            false => None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        (self.side_1 == self.side_2) && (self.side_2 == self.side_3)
    }

    pub fn is_scalene(&self) -> bool {
        (self.side_1 != self.side_2) && (self.side_2 != self.side_3) && (self.side_1 != self.side_3)
    }

    pub fn is_isosceles(&self) -> bool {
        (self.side_1 == self.side_2) || (self.side_1 == self.side_3) || (self.side_2 == self.side_3) 
    }
}
