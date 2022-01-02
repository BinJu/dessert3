#[cfg(test)]
use std::collections::HashMap;
#[cfg(not(test))]
use crate::document::css::CssDocument;

use crate::document::Document;

#[cfg(test)]
use crate::mock::{MockCssDocument,InternalNode};

#[derive(Debug,Clone)]
pub struct DocBuilder {
    #[cfg(test)]
    value_map: HashMap<&'static str, Vec<InternalNode>>
}

impl DocBuilder {
    #[cfg(test)]
    pub fn build_doc(&self, _html: &str) -> impl Document {
        MockCssDocument::from_map(self.value_map.clone())
    }

    #[cfg(not(test))]
    pub fn build_doc(&self, html: &str) -> impl Document {
        CssDocument::from(html)
    }

    #[cfg(not(test))]
    pub fn new() -> Self {
        DocBuilder{}
    }

    #[cfg(test)]
    pub fn new() -> Self {
        DocBuilder{value_map: HashMap::new()}
    }

    #[cfg(test)]
    pub fn from(value_map: HashMap<&'static str, Vec<InternalNode>>) -> Self {
        DocBuilder{value_map}
    }
}

