use crate::ast;

pub enum ParseErrorType {
    ExpectDocumentPart,
}

pub struct ParseError {
    pub error_type: ParseErrorType,
}

pub fn parse_document(s: &str) -> Vec<ast::DocumentPart> {
    Vec::new()
}

//fn parse_document_part(s: &str) -> Result<ast::DocumentPart, ParseError> {
//}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_basic_functions_test() {
    }
}
