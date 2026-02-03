use std::collections::HashMap;

pub struct Element {
    pub text: String,
    tags: HashMap<String, String>

}

impl Element {
    pub fn new(raw_element: &String) -> Self {
        let mut tags: HashMap<String, String> = HashMap::new();   
        let mut i = 0;

        // moving i to the correct starting index
        for c in raw_element.chars() {
            if c == '<' || c == ' ' {
                i += 1;
            } else {
                break;
            }
        }

        // moving i past the name of the element
        for c in raw_element[i..].chars() {
            if c == ' ' {
                break;
            } else {
                i += 1;
            }
        }

        for c in raw_element[i..].chars() {
            if c == ' ' {
                i += 1;
            } else {
                break;
            }
        }
        // Adding key, value pairs to tags
        let mut end = true;
        while i < raw_element.len() && end {
            let tag_name: String;
            let tag_value: String;

            let mut tag_name_start_idx = i;
            let mut tag_name_end_idx = tag_name_start_idx;
            for c in raw_element[tag_name_start_idx..].chars() {
                if c == '=' {
                    break;
                } else {
                    tag_name_end_idx += 1;
                }
            }

            tag_name = raw_element[tag_name_start_idx..tag_name_end_idx].to_string();

            let mut tag_value_start_idx = tag_name_end_idx;
            for c in raw_element[tag_value_start_idx..].chars() {
                if c == '"' || c == '\'' || c == '=' {
                    tag_value_start_idx += 1;
                } else {
                    break;
                }
            }
            
            let mut tag_value_end_idx = tag_value_start_idx;
            for c in raw_element[tag_value_start_idx..].chars() {
                if c == '"' || c == '\'' || c == ' ' || c == '>' {
                    break;
                } else {
                    tag_value_end_idx += 1;
                }
            }

            tag_value = raw_element[tag_value_start_idx..tag_value_end_idx].to_string();
            println!("Tag name:{tag_name}, tag_value:{tag_value}");
            tags.insert(tag_name.to_lowercase(), tag_value);

            i = tag_value_end_idx;       
            for c in raw_element[i..].chars() {
                if c == '>' {
                    end = false;
                } else if c == '"' || c == '\'' || c == ' ' {
                    i += 1;
                } else {
                    break;
                }
            }
            


        }

        


        Self { text: raw_element.clone() , tags}
    }

    pub fn get_tag_value(&self, tag: &String) -> Option<&String> {
        self.tags.get(&tag.to_lowercase())
    }    




}


#[cfg(test)]
mod tests {
    use crate::element::Element;

    #[test]
    fn test_element_creation() {
        let raw_element = "<div class=\"fola\" id=\"alien\">".to_string();
        let element = Element::new(&raw_element);
        assert_eq!(element.get_tag_value(&"cLass".to_string()), Some(&"fola".to_string()));
        assert_eq!(element.get_tag_value(&"id".to_string()), Some(&"alien".to_string()));
    }
}