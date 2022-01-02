use crate::document::Document;
use crate::template_parser::CommonError;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct MockCssDocument {
    map: HashMap<&'static str,Vec<InternalNode>>
}

impl Document for MockCssDocument {
    fn select(&self, selector: &str) -> Result<Option<String>, CommonError> {
        match self.map.get(selector) {
            Some(v) => { 
                if v.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(v[0].text()))
                }
            },
            None => Ok(None)
        }
    }
    fn select_prop(&self, selector: &str, prop: &str) -> Result<Option<String>, CommonError> {
        match self.map.get(selector) {
            Some(v) => {
                if v.is_empty() {
                    Ok(None)
                } else {
                    Ok(v[0].prop(prop))
                }
            },
            None => Ok(None)
        }
    }
    fn select_all(&self, selector: &str) -> Result<Option<Vec<String>>, CommonError> {
        match self.map.get(selector) {
            Some(v) => {let mut result = Vec::new(); v.iter().for_each(|item| result.push(item.text())); Ok(Some(result))},
            None => Ok(None)
        }
    }
    fn select_all_prop(&self, selector: &str, prop: &str) -> Result<Option<Vec<String>>, CommonError> {
        match self.map.get(selector) {
            Some(v) => {let mut result = Vec::new(); v.iter().for_each(|item| {
                if let Some(prop) = item.prop(prop) {result.push(prop)}
            }); Ok(Some(result))},
            None => Ok(None)
        }
    }
}
impl MockCssDocument {
    /*pub fn set(&mut self, selector: &'static str, value: &'static str) {
        if self.map.get(selector).is_none() {
            self.map.insert(selector, vec![InternalNode::new(value)]);
        } else {
            self.map.get_mut(selector).unwrap().push(InternalNode::new(value));
        }

    }
    pub fn set_with_property(&mut self, selector: &'static str, prop_name: &'static str, prop_value: &'static str) {
        if self.map.get(selector).is_none() {
            self.map.insert(selector, vec![InternalNode::new("")]);
        }
        self.map.get_mut(selector).unwrap()[0].prop_map.insert(prop_name, prop_value);
    }*/
    pub fn new() -> Self {
        MockCssDocument{map: HashMap::new()}
    }

    pub fn from_map(map: HashMap<&'static str, Vec<InternalNode>>) -> Self {
        MockCssDocument{map}
    }
}

#[derive(Debug, Clone)]
pub struct InternalNode {
    text: String,
    prop_map: HashMap<&'static str,&'static str>
}

impl InternalNode {
    pub fn text(&self) -> String {
        self.text.clone()
    }
    pub fn prop(&self, prop_name: &str) -> Option<String> {
        self.prop_map.get(prop_name).map(|x| String::from(*x))
    }
    pub fn set_prop(&mut self, prop_name: &'static str, prop_val: &'static str) {
        self.prop_map.insert(prop_name, prop_val);
    }
    pub fn new(text: &str) -> Self {
        Self{text: String::from(text), prop_map: HashMap::new()}
    }
}
