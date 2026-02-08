use crate::element::{self, Element};


pub struct QueryBuilder {
    limit: Option<u64>,
    tag: Option<String>,
    attr: Option<String>,
    attr_value: Option<String>,
    elements: Vec<Element>
}

impl QueryBuilder {

    

    fn find(self, tag_name: &str) -> QueryBuilder {
        QueryBuilder {
            limit: Some(1),
            tag: Some(tag_name.to_string()), 
            attr: self.attr, 
            attr_value: self.attr_value,
            elements: self.elements
        }
    }

    fn find_all(self, tag_name: &str) -> QueryBuilder {
        QueryBuilder {
            limit: self.limit, 
            tag: Some(tag_name.to_string()), 
            attr: self.attr, 
            attr_value: self.attr_value,
            elements: self.elements
        }
    }

    fn limit(self, limit: u64) -> QueryBuilder {
        QueryBuilder { 
            limit: Some(limit), 
            tag: self.tag, 
            attr: self.attr, 
            attr_value: self.attr_value,
            elements: self.elements
        }
    }

    fn attr(self, attr: &str) -> QueryBuilder {
        QueryBuilder { 
            limit: self.limit, 
            tag: self.tag, 
            attr: Some(attr.to_string()), 
            attr_value: self.attr_value,
            elements: self.elements
        }
    }

    fn attr_value(self, attr_value: &str) -> QueryBuilder {
        QueryBuilder { 
            limit: self.limit, 
            tag: self.tag, 
            attr: self.attr, 
            attr_value: Some(attr_value.to_string()),
            elements: self.elements
        }
    }

    fn finish(self) -> Option<Vec<Element>> {
        let results = self.elements;
        match self.tag {
            Some(tag) => {
                results = results
                    .iter()
                    .filter(|tag_name| tag_name == tag)
            },
            None
        }
    }



}