use super::Attribute;

#[derive(Clone)]
pub struct AttributeList {
    attributes: Vec<Attribute>
}

impl AttributeList {

    pub fn is_male(&self) -> bool {
        self.has_attribute(&Attribute::Male)
    }
    pub fn is_female(&self) -> bool {
        self.has_attribute(&Attribute::Female)
    }

    pub fn is_married(&self) -> bool {
        self.has_attribute(&Attribute::Married(0))
    }
    pub fn is_single(&self) -> bool {
        self.has_attribute(&Attribute::Single)
    }

    pub fn set_male(&mut self) -> &mut Self {
        if !self.is_male() {
            self.remove_attribute(&Attribute::Female);
            self.attributes.push(Attribute::Male);
        }
        self
    }
    pub fn set_female(&mut self) -> &mut Self {
        if !self.is_female() {
            self.remove_attribute(&Attribute::Male);
            self.attributes.push(Attribute::Female);
        }
        self
    }

    pub fn set_married(&mut self, partner_id: u32) -> &mut Self {
        self.remove_attribute(&Attribute::Single);
        self.remove_attribute(&Attribute::Married(0));
        self.attributes.push(Attribute::Married(partner_id));
        self
    }

    pub fn set_single(&mut self) -> &mut Self {
        while self.is_married() {
            self.remove_attribute(&Attribute::Married(0));
        }
        if !self.is_single() {
            self.attributes.push(Attribute::Single);
        }
        self
    } 
    pub fn satisfies(&self, wanted_attributes: &AttributeList) -> bool {
        for want in wanted_attributes.get_list().iter() {
            if !self.has_attribute(want) {
                return false;
            }
        }
        true
    }
    
    fn has_attribute(&self, target_attr: &Attribute) -> bool {
        self.get_attr_index(target_attr).is_some()
    }

    fn remove_attribute(&mut self, target_attr: &Attribute) {
        if let Some(i) = self.get_attr_index(target_attr) {
            self.attributes.swap_remove(i);
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
