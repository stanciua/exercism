#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs = 0x1,
    Peanuts = 0x2,
    Shellfish = 0x4,
    Strawberries = 0x8,
    Tomatoes = 0x10,
    Chocolate = 0x20,
    Pollen = 0x40,
    Cats = 0x80,
}

pub struct Allergies {
    allergies: i32,
}

impl Allergies {
    pub fn new(allergies: i32) -> Allergies {
        Allergies { allergies: allergies }
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        let mut rest = self.allergies;
        let mut v: Vec<Allergen> = Vec::new();

        for allergen in &[0x80, 0x40, 0x20, 0x10, 0x8, 0x4, 0x2, 0x1] {
            if Allergies::allergic_to(rest, *allergen) {
                v.push(Allergies::i32_to_allergen(*allergen));
                rest -= *allergen;
            }
        }
        v.into_iter().rev().collect()
    }
    fn allergic_to(allergies: i32, allergen: i32) -> bool {
        allergies - allergen >= 0
    }

    fn i32_to_allergen(val: i32) -> Allergen {
        match val {
            0x1 => Allergen::Eggs,
            0x2 => Allergen::Peanuts,
            0x4 => Allergen::Shellfish,
            0x8 => Allergen::Strawberries,
            0x10 => Allergen::Tomatoes,
            0x20 => Allergen::Chocolate,
            0x40 => Allergen::Pollen,
            0x80 => Allergen::Cats,
            _ => panic!("Unsupported allergen!"),
        }
    }
    pub fn is_allergic_to(&self, allergy: &Allergen) -> bool {
        self.allergies().contains(allergy)
    }
}
