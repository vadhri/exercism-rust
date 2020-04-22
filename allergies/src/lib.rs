pub struct Allergies {
    score: u32
}

#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
    Unknown
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score: score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let a: u8 = allergen.clone() as u8;
        self.score & 2_u32.pow(a.into()) == 2_u32.pow(a.into())
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut bit_vector = Vec::new();
        let mut decimal = self.score;
        let mut bit_counter: i32 = 0;

        while decimal > 0 {
            if decimal % 2 == 1 {
                let allergy = match bit_counter {
                    0 => Allergen::Eggs,
                    1 => Allergen::Peanuts,
                    2 => Allergen::Shellfish,
                    3 => Allergen::Strawberries,
                    4 => Allergen::Tomatoes,
                    5 => Allergen::Chocolate,
                    6 => Allergen::Pollen,
                    7 => Allergen::Cats,
                    _ => Allergen::Unknown
                };
                if allergy != Allergen::Unknown {
                    bit_vector.push(allergy);
                }
            }
            decimal /= 2;
            bit_counter += 1;
        }

        bit_vector
    }
}
