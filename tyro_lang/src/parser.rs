extern crate pest;
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "tyro.pest"]
struct TyroPestParser;

pub fn parse(s: &str) {
    let _pest_parsed_data = TyroPestParser::parse(Rule::file, &s);
    
    //.expect("unsuccessful parse").next().unwrap();
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_basic_functions_test() {
        let path = std::path::Path::new("tyro_lang/src/test_shaders/basic_functions.tyro");
        if let Ok(unparsed_string) = std::fs::read_to_string(path) {
            super::parse(&unparsed_string);
        }
    }
}
