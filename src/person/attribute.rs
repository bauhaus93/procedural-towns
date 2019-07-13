use crate::utility::Date;

#[derive(Eq, Clone, Copy)]
pub enum Attribute {
    Male,
    Female,
    Married(u32),
    Pregnant { father_id: u32, birth: Date, count: u32 },
    Fertile
}

impl PartialEq for Attribute {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Attribute::Male, Attribute::Male) => true,
            (Attribute::Female, Attribute::Female) => true,
            (Attribute::Married(_), Attribute::Married(_)) => true,
            (Attribute::Pregnant { .. }, Attribute::Pregnant { .. }) => true,
            (Attribute::Fertile, Attribute::Fertile) => true,
            (_, _) => false
        }
    }
}
