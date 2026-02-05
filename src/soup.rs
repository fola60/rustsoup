use crate::element::Element;
use std::collections::HashMap;

pub struct Soup {
    element_map: HashMap<String, Vec<Element>>,

}

impl Soup {
    pub fn new(raw_html: &String) -> Self {
        Self {}
    }

    pub fn find(&self, tag: &str) -> Option<&Element> {
        self.element_map
            .get(tag)
            .and_then(|v| v.first())
    }

    pub fn find_all(&self, tag: &str) -> &[Element] {
        self.element_map
            .get(tag)
            .map(|v| v.as_slice())
            .unwrap_or(&[])  
    }
}
