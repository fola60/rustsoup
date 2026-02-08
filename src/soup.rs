use crate::{element::Element, query_builder::QueryBuilder};
use std::collections::HashMap;

pub struct Soup {
    query_builder: QueryBuilder,
    elements: Vec<Element>
}

impl Soup {
    fn new(raw_html: &str) -> Self {
        Self {
            query_builder: QueryBuilder,
            elements: vec![]
        }
    }
}
