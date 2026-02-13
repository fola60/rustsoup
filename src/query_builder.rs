use crate::element::Element;


pub struct QueryBuilder<'a> {
    limit: Option<u16>,
    tag: Option<String>,
    attr: Option<String>,
    attr_value: Option<String>,
    elements: &'a Vec<Element>
}

impl<'a> QueryBuilder<'a> {

    pub fn new(
        limit: Option<u16>,
        tag: Option<String>,
        attr: Option<String>,
        attr_value: Option<String>,
        elements: &'a Vec<Element>
    ) -> Self { 
        QueryBuilder { limit: limit, tag: tag, attr: attr, attr_value: attr_value, elements: elements }
    }
    

    pub fn find(mut self, tag_name: &str) -> QueryBuilder<'a> {
        self.limit = Some(1);
        self.tag = Some(tag_name.to_string());
        self

    }

    pub fn find_all(mut self, tag_name: &str) -> QueryBuilder<'a> {

        self.tag = Some(tag_name.to_string());
        self
    }

    pub fn limit(mut self, limit: u16) -> QueryBuilder<'a> {
        self.limit = Some(limit);
        self
    }

    pub fn attr(mut self, attr: &str) -> QueryBuilder<'a> {
        self.attr = Some(attr.to_string());
        self
    }

    pub fn attr_value(mut self, attr_value: &str) -> QueryBuilder<'a> {
        self.attr_value = Some(attr_value.to_string());
        self
    }

    pub fn finish(&self) -> Option<Vec<&Element>> {
        let mut results: Vec<&Element> = self.elements.iter().collect();

        if let Some(tag) = &self.tag { 
            results = results
                .into_iter()
                .filter(|element| element.name == *tag)
                .collect();
        }

        if let (Some(attr), Some(attr_value)) = (&self.attr, &self.attr_value) {
            results = results
                .into_iter()
                .filter(|element| element.get_tag_value(attr) == Some(attr_value))
                .collect();
        }

        if let Some(limit) = self.limit {
            results.truncate(limit as usize);
        }

        Some(results)
    }



}