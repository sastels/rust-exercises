pub struct Allergies {
    allergens: Vec<Allergen>,
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
    FavaBeans,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        use Allergen::*;
        let allergens = (0..)
            .zip(
                [
                    Eggs,
                    Peanuts,
                    Shellfish,
                    Strawberries,
                    Tomatoes,
                    Chocolate,
                    Pollen,
                    Cats,
                ]
                .iter(),
            )
            .filter(|(n, _)| (score >> n) & 1 == 1)
            .map(|(_, allergen)| allergen)
            .cloned()
            .collect();
        Allergies { allergens }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
