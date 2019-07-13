use crate::utility::Date;
use super::Attribute;

#[derive(Clone)]
pub struct AttributeList {
    attributes: Vec<Attribute>
}

pub struct AttributeListBuilder {
    list: AttributeList
}

impl AttributeList {

    pub fn builder() -> AttributeListBuilder {
        AttributeListBuilder::default()
    }

    pub fn is_male(&self) -> bool {
        self.has_attribute(&Attribute::Male)
    }
    pub fn is_female(&self) -> bool {
        self.has_attribute(&Attribute::Female)
    }

    pub fn is_married(&self) -> bool {
        self.has_attribute(&Attribute::Married(0))
    }

    pub fn is_fertile(&self) -> bool {
        self.has_attribute(&Attribute::Fertile)
    }

    pub fn get_spouse(&self) -> Option<u32> {
        match self.get_attr(&Attribute::Married(0)) {
            Some(&Attribute::Married(spouse_id)) => Some(spouse_id),
            Some(_) => unreachable!("Attribute should have been Attribute::Married"),
            None => None,
        }
    }

    pub fn set_male(&mut self) {
        if !self.is_male() {
            self.remove_attribute(&Attribute::Female);
            self.attributes.push(Attribute::Male);
        }
    }
    pub fn set_female(&mut self) {
        if !self.is_female() {
            self.remove_attribute(&Attribute::Male);
            self.attributes.push(Attribute::Female);
        }
    }

    pub fn set_married(&mut self, partner_id: u32) {
        self.remove_attribute(&Attribute::Married(0));
        self.attributes.push(Attribute::Married(partner_id));
    }

    pub fn pop_marriage(&mut self) -> Option<Attribute> {
        self.remove_attribute(&Attribute::Married(0))
    }

    pub fn set_pregnant(&mut self, father_id: u32, birth: Date, count: u32) {
        self.attributes.push(Attribute::Pregnant { father_id: father_id, birth: birth, count: count });
    }

    pub fn pop_pregnancy(&mut self) -> Option<Attribute> {
        self.remove_attribute(&Attribute::Pregnant { father_id: 0, birth: Date::default(), count: 0 })
    }

    pub fn set_fertile(&mut self) {
        if !self.is_fertile() {
            self.attributes.push(Attribute::Fertile);
        }
    }

    pub fn clear_fertile(&mut self) {
        self.remove_attribute(&Attribute::Fertile);
    }

    pub fn add(&mut self, attr: Attribute) {
        self.attributes.push(attr);
    }

    pub fn satisfies(&self, wanted_attributes: &AttributeList, unwanted_attributes: &AttributeList) -> bool {
        for want in wanted_attributes.get_list().iter() {
            if !self.has_attribute(want) {
                return false;
            }
        }
        for unwant in unwanted_attributes.get_list().iter() {
            if self.has_attribute(unwant) {
                return false;
            }
        }
        true
    }
    
    fn has_attribute(&self, target_attr: &Attribute) -> bool {
        self.get_attr_index(target_attr).is_some()
    }

    pub fn remove_attribute(&mut self, target_attr: &Attribute) -> Option<Attribute> {
        if let Some(i) = self.get_attr_index(target_attr) {
            Some(self.attributes.swap_remove(i))
        } else {
            None
        }
    }

    fn get_attr_index(&self, target_attr: &Attribute) -> Option<usize> {
        for (i, attr) in self.attributes.iter().enumerate() {
            if attr == target_attr {
                return Some(i);
            }
        }
        None
    }

    fn get_attr(&self, target_attr: &Attribute) -> Option<&Attribute> {
       match self.get_attr_index(target_attr) {
           Some(i) => Some(&self.attributes[i]),
           None => None
       }
    }

    fn get_list(&self) -> &[Attribute] {
        &self.attributes
    }
}


impl Default for AttributeList {
    fn default() -> AttributeList {
        Self {
            attributes: Vec::new()
        }
    }
}

impl AttributeListBuilder {
    pub fn set_male(mut self) -> Self {
        self.list.set_male();
        self
    }
    pub fn set_female(mut self) -> Self {
        self.list.set_female();
        self
    }
    pub fn set_married(mut self) -> Self {
        self.list.set_married(0);
        self
    }
    pub fn set_pregnant(mut self) -> Self {
        self.list.set_pregnant(0, Date::default(), 0);
        self
    }
    pub fn set_fertile(mut self) -> Self {
        self.list.set_fertile();
        self
    }

    pub fn build(self) -> AttributeList {
        self.list
    }
}


impl Default for AttributeListBuilder {
    fn default() -> AttributeListBuilder {
        Self {
            list: AttributeList::default()
        }
    }
}
