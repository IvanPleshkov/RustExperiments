use memory_safe_std::runtime_error::RuntimeError;
use memory_safe_std::safe_vec::SafeVec;
use memory_safe_std::safe_string::SafeString;
use memory_safe_std::locale;
use crate::machine_integer::MachineHalfUinteger;
use crate::machine_integer::MachineHalfIntegerSize;

pub struct BigInteger {
    digits: SafeVec<MachineHalfUinteger>,
    negative: bool,
}

#[derive(Debug)]
pub enum ParseError {
    RuntimeError(RuntimeError),
    SyntaxError,
}

impl BigInteger {
    pub fn from_str(s: &str) -> Result<BigInteger, ParseError> {
        let (digits, radix) = Self::to_digits(s)?;
        let binary_digits = match Self::to_binary_digits(digits, radix) {
            Ok(o) => o,
            Err(e) => return Err(ParseError::RuntimeError(e)),
        };

        let mut data = {
            let mut data = SafeVec::new();
            let mut dataSize = binary_digits.len() / MachineHalfIntegerSize as usize;
            if binary_digits.len() % (MachineHalfIntegerSize as usize) != 0 {
                dataSize = dataSize + 1;
            }
            if let Err(e) = data.resize(dataSize, 0) {
                return Err(ParseError::RuntimeError(e));
            }
            data
        };

        for i in 0..(binary_digits.len() - 1) {
            if binary_digits[i] == 0 {
                continue;
            }
            let bytePos = i / MachineHalfIntegerSize as usize;
            let bitPos = i % MachineHalfIntegerSize as usize;
            let d : MachineHalfUinteger = data[bytePos];
            let d = d + (1 << bitPos);
            data[bytePos] = d;
        };

        Ok(BigInteger {
            digits: data,
            negative: Self::get_sign(s)?,
        })
    }

    pub fn get_radix(s: &str) -> Result<u8, ParseError> {
        if s.len() == 0 {
            return Err(ParseError::SyntaxError);
        } else {
            return Ok(10);
        };
    }

    pub fn get_sign(s: &str) -> Result<bool, ParseError> {
        if s.len() == 0 {
            return Err(ParseError::SyntaxError);
        } else {
            let first_char = s.chars().next().unwrap();
            return Ok(locale::is_minus_sign_symbol(first_char));
        };
    }

    pub fn to_digits(s: &str) -> Result<(SafeVec<u8>, u8), ParseError> {
        let radix = Self::get_radix(s)? as u32;

        let mut digits = SafeVec::new();
        if let Err(e) = digits.reserve(s.len()) {
            return Err(ParseError::RuntimeError(e));
        }

        for c in s.chars() {
            if let Some(digit) = c.to_digit(radix) {
                if digit < radix {
                    return Err(ParseError::SyntaxError);    
                }

                if let Err(e) = digits.push(digit as u8) {
                    return Err(ParseError::RuntimeError(e))
                }
            } else {
                return Err(ParseError::SyntaxError);
            }
        };

        Ok((digits, radix as u8))
    }

    pub fn to_string(&self) -> Result<SafeString, RuntimeError> {
        Err(RuntimeError::Exception)
    }

    fn to_binary_digits(digits: SafeVec<u8>, radix: u8) -> Result<SafeVec<u8>, RuntimeError> {
        let binary_digits = if radix == 2 {
            return Ok(digits)
        } else {
            let digits = Self::reverse(digits);
            let binary_digits = SafeVec::new();
            while digits.len() > 0 {
                let mod_result = Self::div_reversed_digits_by_2(&mut digits, radix);
                binary_digits.push(mod_result)?;
            };
            return Ok(binary_digits)
        };
    }

    fn div_reversed_digits_by_2(digits: &mut SafeVec<u8>, radix: u8) -> u8 {
        let prevMod = 0;
        for digit in digits.reverse_iter() {
            let val = digit + radix * prevMod;
            let newVal = val / 2;
            let prevMod = val % 2;
            digit = val;
        };
        while digits.len() > 0 && digits[digits.len() - 1] == 0 {
            digits.pop();
        };
        prevMod
    }

    fn reverse<T>(v: SafeVec<T>) -> SafeVec<T> {
        let maxIndex = v.len() - 1;
        for i in 0..maxIndex / 2 {
            let b = v[i];
            v[i] = v[maxIndex - i];
            v[maxIndex - i] = b;
        };
        v
    }
}

#[cfg(test)]
mod tests {
    use super::BigInteger;

    #[test]
    fn big_integer_from_str_1() {
        let b = BigInteger::from_str("123").unwrap();
        assert_eq!(b.to_string().unwrap().s, "123");
    }
}
