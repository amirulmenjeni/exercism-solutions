pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[repr(u32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs            = 0x01,
    Peanuts         = 0x02,
    Shellfish       = 0x04,
    Strawberries    = 0x08,
    Tomatoes        = 0x10,
    Chocolate       = 0x20,
    Pollen          = 0x40,
    Cats            = 0x80,
}

impl Allergies {

    fn try_push(score: u32, allergen: Allergen, allergens: &mut Vec<Allergen>) {
        if score & (allergen as u32) != 0 {
            allergens.push(allergen);
        }
    }

    pub fn new(score: u32) -> Self {
        let mut allergens: Vec<Allergen> = Vec::new();
        
        Allergies::try_push(score, Allergen::Eggs, &mut allergens);
        Allergies::try_push(score, Allergen::Peanuts, &mut allergens);
        Allergies::try_push(score, Allergen::Shellfish, &mut allergens);
        Allergies::try_push(score, Allergen::Strawberries, &mut allergens);
        Allergies::try_push(score, Allergen::Tomatoes, &mut allergens);
        Allergies::try_push(score, Allergen::Chocolate, &mut allergens);
        Allergies::try_push(score, Allergen::Pollen, &mut allergens);
        Allergies::try_push(score, Allergen::Cats, &mut allergens);

        Allergies {
            allergens
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.iter().any(|x| x == allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
