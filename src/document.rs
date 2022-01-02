pub mod css;

pub type CommonError = Box<dyn std::error::Error>;

pub trait Document: Clone {
    fn select(&self, selector: &str) -> Result<Option<String>, CommonError>;
    fn select_prop(&self, selector: &str, prop: &str) -> Result<Option<String>, CommonError>;
    fn select_all(&self, selector: &str) -> Result<Option<Vec<String>>, CommonError>; 
    fn select_all_prop(&self, selector: &str, prop: &str) -> Result<Option<Vec<String>>, CommonError>;
}

#[derive(Debug,Clone)]
pub struct ParseError {
    error: String 
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ERROR] {}", self.error)
    }
}

impl ParseError {
    pub fn new(error: &str) -> Self{
        ParseError{error: error.to_owned()}
    }
    pub fn new_str(error: String) -> Self {
        ParseError{error}
    }
}
