
#[derive(Eq, Clone, Copy)]
pub enum Attribute {
    Male,
    Female,
    Single,
    Married(u32)
}

impl PartialEq for Attribute {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Attribute::Male, Attribute::Male) => true,
            (Attribute::Female, Attribute::Female) => true,
            (Attribute::Single, Attribute::Single) => true,
            (Attribute::Married(_), Attribute::Married(_)) => true,
            (_, _) => false
        }
    }
}
