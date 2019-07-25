pub struct BigInteger {
    digits: Vec<u32>,
    negative: bool,
}

pub struct ParseError;

impl BigInteger {
    pub fn from_str() -> Result<BigInteger, ParseError> {
        Ok(BigInteger {
            digits: Vec::new(),
            negative: false,
        })
    }
}
