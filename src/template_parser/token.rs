use super::{DocBuilder, TemplateNode, StrTemplateNode, CssTemplateNode, LoopTemplateNode, EndTemplateNode, VarTemplateNode};

pub struct TokenParser<'a> {
    template: String,
    cursor: usize,
    doc_builder: &'a DocBuilder
}

impl<'a> Iterator for TokenParser<'a> {
    type Item = Box<dyn TemplateNode>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Some(token_pos) => {
                if token_pos.0 > self.cursor {
                    let result: Box<dyn TemplateNode> = Box::new(StrTemplateNode{text: String::from(&self.template[self.cursor..token_pos.0])});
                    self.cursor = token_pos.0;
                    Some(result)
                } else {
                    let result: Box<dyn TemplateNode> = Self::token_to_template_node(&self.template[token_pos.0+2..token_pos.1], self.doc_builder);
                    self.cursor = token_pos.1 + 2;
                    Some(result)
                }
            },
            None => {
                if self.cursor < self.template.len() {
                    let result: Box<dyn TemplateNode> = Box::new(StrTemplateNode{text: String::from(&self.template[self.cursor..])});
                    self.cursor = self.template.len();
                    Some(result)
                } else {
                    None
                }
            }
        }
    }
}

impl<'a> TokenParser<'a> { pub fn new(template: &str, doc_builder: &'a DocBuilder) -> Self {
        TokenParser{template: String::from(template), cursor: 0, doc_builder}
    }

    fn next_token(&self) -> Option<(usize,usize)> {
        if let Some(start_pos) = self.template[self.cursor..].find("{{") {
            let cur_pos = self.cursor + start_pos + 2;
            if let Some(end_pos) = self.template[cur_pos..].find("}}") {
                let token_pos = (start_pos+self.cursor, end_pos+cur_pos);
                Some(token_pos)
            } else {
                panic!("unable to find matched `}}` token for the token at: {}", start_pos+self.cursor);
            }
        } else {
            None
        }
    }

    fn token_to_template_node(token: &str, doc_builder: &'a DocBuilder) -> Box<dyn TemplateNode> {
        let tokens = Self::tokenize(token);
        if tokens[0] == "css" {
            let selector = Self::tokenized_value_by_key(&tokens, "selector");
            let default = Self::tokenized_value_by_key(&tokens, "default");
            let node_property = Self::tokenized_value_by_key(&tokens, "node-property");
            if selector.is_none() {
                panic!("unable to find `selector` in the css token: {}", token);
            }
            let doc = Self::tokenized_value_by_key(&tokens, "doc");
            let doc_var = Self::tokenized_value_by_key(&tokens, "doc-var");
            let base_doc = Self::tokenized_value_by_key(&tokens, "base-doc");
            Box::new(CssTemplateNode{css_selector: selector.unwrap(), doc, doc_var, base_doc, default_value: default, node_property, doc_builder: doc_builder.clone()})
        } else if tokens[0] == "loop" {
            let loop_var = Self::tokenized_value_by_key(&tokens, "var").expect("missing 'var' for the loop");
            let selector = Self::tokenized_value_by_key(&tokens, "selector");
            if selector.is_none() {
                panic!("unable to find `selector` in the css token: {}", token);
            }
            let selector = selector.unwrap();
            let node_property = Self::tokenized_value_by_key(&tokens, "node-property");
            let doc = Self::tokenized_value_by_key(&tokens, "doc");
            let doc_var = Self::tokenized_value_by_key(&tokens, "doc-var");
            let base_doc = Self::tokenized_value_by_key(&tokens, "base-doc");

            Box::new(LoopTemplateNode{var_name: loop_var, css_selector: selector, node_property, doc, doc_var, base_doc, children: Vec::new(), doc_builder: doc_builder.clone()})
        } else if tokens[0] == "var" {
            let var_name = tokens[1].clone(); 
            Box::new(VarTemplateNode{var_name})
        } else if tokens[0] == "end" {
            Box::new(EndTemplateNode{})
        } else {
            panic!("unknown keyword: {}", tokens[0])
        }
    }

    fn tokenize(token: &str) -> Vec<String> {
        let token = token.trim();
        let mut in_quote = false;
        let mut result = Vec::new();
        let mut buff = String::new();
        let mut is_key = true;
        let mut chars = token.chars();
        while let Some(c) = chars.next() {
            if c == '\'' || c == '"' {
                in_quote = !in_quote;
                if !buff.is_empty() {
                    result.push(buff.clone());
                    buff.clear();
                }
                continue;
            }
            if in_quote {
                buff.push(c);
            } else {
                if c == '=' || c == '>' || c =='<' {
                    if is_key {
                        if !buff.is_empty() {
                            result.push(buff.clone());
                            buff.clear();
                        }
                        is_key = false;
                    }
                } else {
                    if !is_key {
                        if !buff.is_empty() {
                            result.push(buff.clone());
                            buff.clear();
                        }
                        is_key = true;
                    }
                    if c == ' ' {
                        if !buff.is_empty() {
                            result.push(buff.clone());
                            buff.clear();
                        }
                        is_key = true;
                        continue;
                    }
                }
                buff.push(c);
            }
        }
        if !buff.is_empty() {
            result.push(buff.trim().to_owned());
        }
        result
    }

    fn tokenized_value_by_key(tokenized: &Vec<String>, key: &str) -> Option<String> {
        for (idx, ref t) in tokenized.iter().enumerate() {
            if *t == key && tokenized[idx+1] == "=" {
                return Some(tokenized[idx+2].clone());
            } 
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::mock::InternalNode;
    use super::{DocBuilder, TokenParser};
    use std::collections::HashMap;

    #[test]
    fn test_token_parser() {
        let mut mock_data: HashMap<&'static str, Vec<InternalNode>> = HashMap::new();
        let author = InternalNode::new("James Ma");
        mock_data.insert("div.author", vec![author]);
        let doc_builder = DocBuilder::from(mock_data);
        let mut token_parser = TokenParser::new("The author's name is {{css selector='div.author' doc='https://www.google.com'}}!", &doc_builder);
        let token = token_parser.next().unwrap();
        let mut buff = String::new();
        token.evaluate(None, &mut buff).unwrap();
        assert_eq!(buff, "The author's name is ");
        let token = token_parser.next().unwrap();
        token.evaluate(None, &mut buff).unwrap();
        assert_eq!(buff, "The author's name is James Ma");
        let token = token_parser.next().unwrap();
        token.evaluate(None, &mut buff).unwrap();
        assert_eq!(buff, "The author's name is James Ma!");
    }
    #[test]
    fn test_tokenize() {
        let tokens = TokenParser::tokenize("css selector='div.author  > p' default =\"N/A\"");
        assert_eq!(tokens, vec!["css", "selector", "=", "div.author  > p", "default", "=", "N/A"]);
    }

    #[test]
    fn test_tokenized_value_by_key() {
        let tokens = TokenParser::tokenize("css selector='div.author  > p' default =\"N/A\"");
        assert_eq!(TokenParser::tokenized_value_by_key(&tokens, "selector").unwrap(), "div.author  > p");
        assert_eq!(TokenParser::tokenized_value_by_key(&tokens, "default").unwrap(), "N/A");
        assert_eq!(TokenParser::tokenized_value_by_key(&tokens, "the-key-does-not-exist"), None);
    }

}
