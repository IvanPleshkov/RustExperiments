
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Pow,
    Or,
    And,
    Xor,
}

pub enum Number {
    Integer,
    Float,
    Double,
}

pub enum BracketType {
    Round,
    Curly,
    Square,
    Angle,
}

pub enum SpecialCharacter {
    Comma,
    Colon,
    DoubleColon,
    Semicolon,
    Dot,
}

pub enum TokenType {
    Unknown,
    Identifier,
    Number(Number),
    OpenBracket(BracketType),
    CloseBracket(BracketType),
    Operator(Operator),
    SpecialCharacter(SpecialCharacter),
    Comment,
}

pub struct Token<'a> {
    pub position: usize,
    pub line: usize,
    pub column: usize,
    pub data: &'a str,
    pub token_type: TokenType,
}

pub fn tokenize(s: &str) -> Vec<Token> {
    vec![]
}

fn compare_token<'a, 'b>(t1: &Token<'a>, t2: &Token<'b>) -> bool {
    false
}

fn compare_tokens<'a, 'b>(t1: &Vec<Token<'a>>, t2: &Vec<Token<'b>>) -> bool {
    false
}

#[cfg(test)]
mod tests {

    #[test]
    fn empty_string() {
        use super::*;
        let s = "";
        let result = tokenize(s);
        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn only_spaces_and_enters() {
        use super::*;
        let s = "  \n\n  \n";
        let result = tokenize(s);
        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn only_identifier() {
        use super::*;
        let s = "hello";
        let result = tokenize(s);
        let tokens = vec![
            Token {
                position: 0,
                line: 0,
                column: 0,
                data: "hello",
                token_type: TokenType::Identifier,
            },
        ];
        assert_eq!(compare_tokens(&result, &tokens), true);
    }

    #[test]
    fn trim_identifier() {
        use super::*;
        let s = "  q  ";
        let result = tokenize(s);
        let tokens = vec![
            Token {
                position: 2,
                line: 0,
                column: 2,
                data: "q",
                token_type: TokenType::Identifier,
            },
        ];
        assert_eq!(compare_tokens(&result, &tokens), true);
    }
}
