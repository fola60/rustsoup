use crate::{element::Element, query_builder::QueryBuilder};

pub struct Soup {
    elements: Vec<Element>
}

impl Soup {
    fn new(raw_html: &str) -> Self {
        let mut elements: Vec<Element> = vec![];
        let mut i = 0;

        for c in raw_html.chars() {
            if c == '<' {
                break;
            } else {
                i += 1
            }
        }


        while i < raw_html.len() {
            let mut j = i + 1;

            for c in raw_html[j..].chars() {
                if c == '<' && raw_html.chars().nth(j + 1) != Some('/') {
                    break;
                } else {
                    j += 1;
                }
            }

            elements.push(Element::new(&raw_html[i..j].to_string()));
            println!("{}", &raw_html[i..j]);
            i = j;

        }

        Self {
            elements
        }
    }

    fn find(&self, tag_name: &str) -> QueryBuilder {
        QueryBuilder::new( 
            Some(1),
            Some(tag_name.to_string()),
            None,
            None,
            &self.elements
        )
    }

    fn find_all(&self, tag_name: &str) -> QueryBuilder {
        QueryBuilder::new( 
            None,
            Some(tag_name.to_string()),
            None,
            None,
            &self.elements
        )
    }



    
}


#[cfg(test)]
mod tests {
    use crate::Soup;

    #[test]
    fn single_elements() {
        
    }

    #[test]
    fn nested_elements() {
        let raw_html = "<body><html>hello</html></body>";
        let soup = Soup::new(raw_html);
        let binding = soup.find("html");
        let finished = binding.finish().unwrap();       
        let html_tag = finished.first().unwrap();       
        assert_eq!(html_tag.text, String::from("hello"));
        assert_eq!(html_tag.name, String::from("html"));

    }

    #[test]
    fn nested_elements_with_inner_values() {
        
    }


}
