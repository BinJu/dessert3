
mod document;
mod template_parser;

use template_parser::builder::DocBuilder;

#[cfg(test)]
mod mock;

/// This is the 3rd edition of the tool `dessert`. It takes the parameter as the template, then
/// render the output. It is simpler and more flexible than the last(second) version.
/// For example you can run:
/// ```
/// dessert3 'The list of provinces of Canada: {{loop selector='div.field-item > ul > li > a' doc="https://www.statcan.gc.ca/en/reference/province" var="province"}}
/// - {{var province}}
/// {{end}}'
/// ```
/// Which is going to output the list of the provinces:
/// ```
/// The list of provinces of Canada:
/// - Alberta
/// - British Columbia
/// - Manitoba
/// - New Brunswick
/// - Newfoundland and Labrador
/// - Northwest Territories
/// - Nova Scotia
/// - Nunavut
/// - Ontario
/// - Prince Edward Island
/// - Quebec
/// - Saskatchewan
/// - Yukon
/// ```
///
fn main() {
    let template = std::env::args().nth(1).expect("Expect template as a parameter");
    let doc_builder = DocBuilder::new();
    match template_parser::parse(&template, &doc_builder) {
        Ok(output) => println!("{}", output),
        Err(error) => println!("Error: {}", error)
 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock::InternalNode;
    use std::collections::HashMap;
    use template_parser::builder::DocBuilder;
    #[test]
    fn test_parse() {
        let mut mock_data: HashMap<&'static str, Vec<InternalNode>> = HashMap::new();
        let name_john= InternalNode::new("John");
        mock_data.insert("div.name", vec![name_john]);
        let doc_builder = DocBuilder::from(mock_data);
        assert_eq!(template_parser::parse("Hello {{css selector='div.name' doc='https://mock'}}!", &doc_builder).unwrap(), "Hello John!");
    }
}
