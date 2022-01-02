use scraper::{Selector, Html};
use super::{Document, CommonError, ParseError};

#[derive(Debug,Clone)]
pub struct CssDocument {
    doc: Html
}

impl Document for CssDocument {
    fn select(&self, selector: &str) -> Result<Option<String>, CommonError> {
        let parsed_selector = Selector::parse(selector).map_err(|e| Box::new(ParseError::new_str(format!("[CSS Parse Error]: {:?}", e))) )?;
        match self.doc.select(&parsed_selector).next() {
            Some(node) => {let mut buff = String::new(); node.text().for_each(|x| buff.push_str(x)); Ok(Some(buff))},
            None => Ok(None)
        }
    }

    fn select_prop(&self, selector: &str, prop: &str) -> Result<Option<String>, CommonError> {
        let parsed_selector = Selector::parse(selector).map_err(|e| Box::new(ParseError::new_str(format!("[CSS Parse Error]: {:?}", e))) )?;
        match self.doc.select(&parsed_selector).next() {
            Some(node) => Ok(node.value().attr(prop).map(|x| String::from(x))),
            None => Ok(None)
        }
    }

    fn select_all(&self, selector: &str) -> Result<Option<Vec<String>>, CommonError> {
        let parsed_selector = Selector::parse(selector).map_err(|e| Box::new(ParseError::new_str(format!("[CSS Parse Error]: {:?}", e))) )?;
        let mut result = Vec::new();
        let mut selected_node = self.doc.select(&parsed_selector);
        while let Some(node) = selected_node.next() {
            let mut buff = String::new();
            node.text().for_each(|x| buff.push_str(x));
            result.push(buff);
        }
        if result.len() > 0 {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    fn select_all_prop(&self, selector: &str, prop: &str) -> Result<Option<Vec<String>>, CommonError> {
        let parsed_selector = Selector::parse(selector).map_err(|e| Box::new(ParseError::new_str(format!("[CSS Parse Error]: {:?}", e))) )?;
        let mut result = Vec::new();
        let mut selected_node = self.doc.select(&parsed_selector);
        while let Some(node) = selected_node.next() {
            if let Some(attr_val) = node.value().attr(prop) {
                result.push(String::from(attr_val));
            }
        }
        if result.len() > 0 {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

impl std::convert::From<&str> for CssDocument {
    fn from(item: &str) -> Self {
        CssDocument{doc: Html::parse_document(item)}
    }

}

