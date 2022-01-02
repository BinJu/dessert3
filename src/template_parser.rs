mod token;
pub mod builder;

use token::TokenParser;
use crate::document::{Document,ParseError};
use std::collections::HashMap;
use builder::DocBuilder;

pub type Template = Vec<Box<dyn TemplateNode>>;
pub type CommonError = Box<dyn std::error::Error>;

#[allow(dead_code)]
pub fn parse(template: &str, doc_builder: &DocBuilder) -> Result<String, CommonError> {
    let template_nodes = parse_template(template, &doc_builder)?;
    let mut buff = String::new();
    for node in &template_nodes {
        node.evaluate(None, &mut buff)?;
    }
    Ok(buff)
}


fn parse_template(template: &str, doc_builder: &DocBuilder) -> Result<Template, CommonError> {
    let mut temp: Template = Template::new();    
    let mut token_parser = TokenParser::new(template, doc_builder);
    let mut last_containers: Vec<Box<dyn TemplateNode>> = Vec::new();
    while let Some(token) = token_parser.next() {
        if token.is_container() {
            last_containers.push(token);
            continue;
        }
        if token.is_end() {
            let container_token = last_containers.pop().expect("The end token can not match the exist 'if' or 'loop' token");
            if last_containers.is_empty() {
                temp.push(container_token);
            } else {
                last_containers.last_mut().unwrap().add_child(container_token)?;
            }
        } else if last_containers.is_empty() {
            temp.push(token);
        } else {
            last_containers.last_mut().unwrap().add_child(token)?;
        }
    }
    Ok(temp)
}

pub trait TemplateNode: std::fmt::Debug {
    fn evaluate(&self, context: Option<HashMap<String,String>>, buff: &mut String) -> Result<(), CommonError>;
    fn is_container(&self) -> bool {false}
    fn is_end(&self) -> bool {false}
    fn add_child(&mut self, _node: Box<dyn TemplateNode>) -> Result<(), CommonError> {Err(Box::new(ParseError::new("not implemented yet")))}
}

#[derive(Debug)]
struct StrTemplateNode {
    text: String
}

impl TemplateNode for StrTemplateNode {
    fn evaluate(&self, _context: Option<HashMap<String,String>>, buff: &mut String) -> Result<(), CommonError> {
        buff.push_str(&self.text);
        Ok(())
    }
}

fn get_doc(context: &Option<HashMap<String,String>>, doc: &Option<String>, doc_var: &Option<String>, base_doc: &Option<String>) -> String {
    if doc.is_none() && doc_var.is_none() {
        panic!("'doc' or 'doc-var' must be used for the template token");
    }

    let mut doc_url = if let Some(doc_var_url) = doc_var {
        let context = context.clone().expect("empty context for the 'doc-var'");
        if let Some(context_doc_url) = context.get(doc_var_url){
            context_doc_url.clone()
        } else {
            doc.clone().expect("'doc-var' does not exist in the context")
        }
    } else {
        doc.clone().expect("'doc' does not exist")
    };

    if !doc_url.starts_with("http") && base_doc.is_some() {
        doc_url = base_doc.clone().unwrap() + &doc_url;
    }

    #[cfg(not(test))]
    {
        reqwest::blocking::get(&doc_url).expect(&format!("failed to access the document at: {}", doc_url)).text().expect(&format!("failed to get the text of the document: {}", doc_url))
    }
    #[cfg(test)] {
        String::new()
    }
}

#[derive(Debug)]
struct CssTemplateNode {
    css_selector: String,
    default_value: Option<String>,
    node_property: Option<String>,
    doc: Option<String>,
    doc_var: Option<String>,
    base_doc: Option<String>,
    doc_builder: DocBuilder
}

impl TemplateNode for CssTemplateNode {
    fn evaluate(&self, context: Option<HashMap<String,String>>, buff: &mut String) -> Result<(), CommonError> {
        let html_text = get_doc(&context, &self.doc, &self.doc_var, &self.base_doc);
        let css_doc = self.doc_builder.build_doc(&*html_text);

        let selected_value = match &self.node_property {
            Some(property) => css_doc.select_prop(&self.css_selector, &property),
            None => css_doc.select(&self.css_selector)
        }?;

        match selected_value {
            Some(value) => buff.push_str(&value),
            None => match &self.default_value {
                Some(default) => buff.push_str(&default),
                None => return Err(Box::new(ParseError::new("Not able to render the variable"))),
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct LoopTemplateNode {
    var_name: String,
    css_selector: String,
    node_property: Option<String>,
    doc: Option<String>,
    doc_var: Option<String>,
    base_doc: Option<String>,
    children: Vec<Box<dyn TemplateNode>>,
    doc_builder: DocBuilder
}

impl TemplateNode for LoopTemplateNode {
    fn evaluate(&self, context: Option<HashMap<String,String>>, buff: &mut String) -> Result<(), CommonError> {
        let html_text = get_doc(&context, &self.doc, &self.doc_var, &self.base_doc);
        let css_doc = self.doc_builder.build_doc(&*html_text);

        let selected_values = match &self.node_property {
            Some(property) => css_doc.select_all_prop(&self.css_selector, &property),
            None => css_doc.select_all(&self.css_selector)
        }.expect(&format!("failed to render css selector: {}", self.css_selector));

        if let Some(values) = selected_values {
            for value in values {
                let mut context = HashMap::new();
                context.insert(self.var_name.clone(), value.clone());
                for node in &self.children {
                    node.evaluate(Some(context.clone()), buff)?  
                }
            }
        }
        Ok(()) 
    }

    fn is_container(&self) -> bool {
        true
    }

    fn add_child(&mut self, node: Box<dyn TemplateNode>) -> Result<(), CommonError> {
        self.children.push(node);
        Ok(())
    }
}

#[derive(Debug)]
struct EndTemplateNode;

impl TemplateNode for EndTemplateNode {
    fn evaluate(&self, _context: Option<HashMap<String,String>>, _buff: &mut String) -> Result<(), CommonError> {
        Ok(())
    }

    fn is_end(&self) -> bool {
        true
    }
}

#[derive(Debug)]
struct VarTemplateNode {
    var_name: String
}

impl TemplateNode for VarTemplateNode {
    fn evaluate(&self, context: Option<HashMap<String,String>>, buff: &mut String) -> Result<(), CommonError> {
        match context {
            Some(var_context) => {
                match var_context.get(&self.var_name) {
                    Some(var_val) => buff.push_str(var_val),
                    None => {panic!("Unable to find the context for the 'var' node")}
                }
            },
            None => panic!("Unable to find the context for the 'var' node")
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::mock::InternalNode;
    use crate::template_parser::DocBuilder;
    use std::collections::HashMap;

    #[test]
    fn test_css_property() {
        let mut mock_data: HashMap<&'static str, Vec<InternalNode>> = HashMap::new();
        let mut url = InternalNode::new("");
        url.set_prop("href", "https://www.google.com");
        mock_data.insert("div.url", vec![url]);
        let doc_builder = DocBuilder::from(mock_data);
        let result = parse("The author's home page is {{css selector='div.url' node-property='href' doc='https://mock'}}!", &doc_builder).unwrap();
        assert_eq!(result, "The author's home page is https://www.google.com!");
    }

    #[test]
    fn test_css_default_value() {
        let mock_data: HashMap<&'static str, Vec<InternalNode>> = HashMap::new();
        let doc_builder = DocBuilder::from(mock_data);
        let result = parse("The author's home page is {{css selector='div.url' node-property='href' default='https://youtube.com' doc='https://mock'}}!", &doc_builder).unwrap();
        assert_eq!(result, "The author's home page is https://youtube.com!");

    }

    #[test]
    fn test_loop() {
        let mut mock_data: HashMap<&'static str, Vec<InternalNode>> = HashMap::new();
        let name_james = InternalNode::new("James Ma");
        let name_alex = InternalNode::new("Alex Wang");
        mock_data.insert("div.name", vec![name_james, name_alex]);
        let doc_builder = DocBuilder::from(mock_data);
        let result = parse("The authors are: {{loop selector='div.name' var ='name' doc='https://mock'}}Mr. {{var name}} {{end}}", &doc_builder).unwrap();
        assert_eq!(result, "The authors are: Mr. James Ma Mr. Alex Wang ");
    }

    #[test]
    fn test_doc_var() {
        let mut mock_data: HashMap<&'static str, Vec<InternalNode>> = HashMap::new();
        let address_1 = InternalNode::new("https://mock1");
        let address_2 = InternalNode::new("https://mock2");
        mock_data.insert("div.url", vec![address_1, address_2]);
        let info1 = InternalNode::new("This is My Page");
        mock_data.insert("div.info", vec![info1]);
        let doc_builder = DocBuilder::from(mock_data);
        let result = parse("The authors are: {{loop selector='div.url' var ='name' doc='https//mock'}}Info: {{css selector='div.info' doc-var='name'}} {{end}}", &doc_builder).unwrap();
        assert_eq!(result, "The authors are: Info: This is My Page Info: This is My Page ");
    }
}
