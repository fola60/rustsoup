use crate::element::Element;

pub struct Soup {

}

impl Soup {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find(&self, tag: String) -> Option<Element> {
        Some(Element::new(&"".to_string()))
    }

    pub fn find_all(&self, tag: String) -> Vec<Element> {
        vec![Element::new(&"".to_string())]
    }
}
