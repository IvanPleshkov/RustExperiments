use memory_safe_std::locale;

pub type BigDigit = u32;
pub type SignedBigDigit = i32;
pub type DoubleBigDigit = u64;
pub const BIG_DIGIT_SIZE : i8 = 32;
pub const BIG_DIGIT_MAX_VALUE : BigDigit = std::u32::MAX;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Clone)]
pub struct BigInteger {
    digits: Vec<BigDigit>,
    sign: Sign,
}

impl BigInteger {
    pub fn from_str_radix(s: &str, radix: u8) -> Option<BigInteger> {
        let digits = Self::get_digits_from_str(s, radix)?;
        let binary_digits = Self::to_binary_digits(digits, radix);

        let mut data = {
            let mut data_size = binary_digits.len() / BIG_DIGIT_SIZE as usize;
            if binary_digits.len() % (BIG_DIGIT_SIZE as usize) != 0 {
                data_size = data_size + 1;
            };
            let mut data = Vec::with_capacity(data_size);
            data.resize(data_size, 0);
            data
        };

        for i in 0..(binary_digits.len() - 1) {
            if binary_digits[i] == 0 {
                continue;
            }
            let byte_pos = i / BIG_DIGIT_SIZE as usize;
            let bit_pos = i % BIG_DIGIT_SIZE as usize;
            let d : BigDigit = data[byte_pos];
            let d = d + (1 << bit_pos);
            data[byte_pos] = d;
        };

        Some(BigInteger {
            digits: data,
            sign: Self::get_sign_from_str(s)?,
        })
    }

    pub fn from_str(s: &str) -> Option<BigInteger> {
        let radix = Self::get_radix_from_str(s)?;
        Self::from_str_radix(s, radix)
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        if self.is_negative() {
            s.push('-');
        }
        for digit in self.to_digits().iter() {
            if let Some(c) = std::char::from_digit(*digit as u32, 10) {
                s.push(c);
            } else {
                panic!("");
            }
        };
        s
    }

    pub fn to_digits_with_radix(&self, radix: u8) -> Vec<u8> {
        let mut result = Vec::new();
        let mut n = self.clone();
        while !n.is_zero() {
            let (m, mod_result) = n.div_and_mod_to_u64(radix as u64);
            n = m;
            result.push(mod_result as u8);
        };
        result
    }

    pub fn to_digits(&self) -> Vec<u8> {
        self.to_digits_with_radix(10)
    }

    pub fn div_and_mod_to_u64(&self, val: u64) -> (BigInteger, u64) {
        if val == 0 {
            panic!("div 0");
        };

        let mut result = BigInteger{
            digits: Vec::new(),
            sign: self.sign,
        };
        let mut mod_result : DoubleBigDigit = 0;
        for big_digit in self.digits.iter() {
            let value = *big_digit as DoubleBigDigit + mod_result * BIG_DIGIT_MAX_VALUE as DoubleBigDigit;
            result.digits.push((value / val as DoubleBigDigit) as BigDigit);
            mod_result = value % val as DoubleBigDigit;
        };
        result.digits.reverse();
        result.remove_last_zeros();
        (result, mod_result)
    }

    pub fn is_zero(&self) -> bool {
        if self.digits.is_empty() {
            return true;
        }
        assert!(*self.digits.last().unwrap() != 0);
        false
    }

    pub fn is_negative(&self) -> bool {
        self.sign == Sign::Negative
    }

    pub fn sign(&self) -> i8 {
        match self.sign {
            Sign::Positive => if self.is_zero() { 0 } else { 1 },
            Sign::Negative => -1,
        }
    }

    fn remove_last_zeros(&mut self) {
        while let Some(e) = self.digits.last() {
            if *e == 0 {
                self.digits.pop();
            } else {
                break;
            }
        };
    }

    fn get_radix_from_str(s: &str) -> Option<u8> {
        if s.len() == 0 {
            return None;
        } else {
            return Some(10);
        };
    }

    fn get_sign_from_str(s: &str) -> Option<Sign> {
        if s.len() == 0 {
            None
        } else {
            let first_char = s.chars().next().unwrap();
            Some( if locale::is_minus_sign_symbol(first_char) {
                Sign::Negative
            } else {
                Sign::Positive
            })
        }
    }

    fn get_digits_from_str(s: &str, radix: u8) -> Option<(Vec<u8>)> {
        let mut digits = Vec::with_capacity(s.len());
        for c in s.chars() {
            if let Some(digit) = c.to_digit(radix as u32) {
                let digit = digit as u8;
                assert!(digit < radix);
                digits.push(digit);
            } else {
                return None;
            }
        };

        if digits.len() > 0 {
            Some(digits)
        } else {
            None
        }
    }

    fn to_binary_digits(mut digits: Vec<u8>, radix: u8) -> Vec<u8> {
        if radix == 2 {
            digits
        } else {
            let mut binary_digits = Vec::new();
            digits.reverse();
            Self::remove_zeros_from_reversed_digits(&mut digits);
            while digits.len() > 0 {
                let mod_result = Self::div_reversed_digits_by_2(&mut digits, radix);
                binary_digits.push(mod_result);
            };
            binary_digits.reverse();
            binary_digits
        }
    }

    fn div_reversed_digits_by_2(digits: &mut Vec<u8>, radix: u8) -> u8 {
        let mut prev_mod = 0;
        for digit in digits.iter_mut().rev() {
            let val = *digit + radix * prev_mod;
            let new_val = val / 2;
            assert!(new_val < radix);
            prev_mod = val % 2;
            *digit = new_val;
        };
        Self::remove_zeros_from_reversed_digits(digits);
        prev_mod
    }

    fn remove_zeros_from_reversed_digits(digits: &mut Vec<u8>) {
        while digits.len() > 0 && digits[digits.len() - 1] == 0 {
            digits.pop();
        };
    }
}

#[cfg(test)]
mod tests {
    use super::BigInteger;

    #[test]
    fn test_to_binary_digits_1() {
        let test_vec : Vec<u8> = vec![0, 9, 1];
        let radix : u8 = 10;
        let binary_digits = BigInteger::to_binary_digits(test_vec, radix);
        assert_eq!(binary_digits, vec![1, 0, 1, 1, 0, 1, 1]);
    }

    #[test]
    fn test_to_binary_digits_2() {
        let test_vec : Vec<u8> = vec![1, 5, 2, 2, 7, 3, 0, 6, 2, 0, 1, 2, 3];
        let radix : u8 = 10;
        let binary_digits = BigInteger::to_binary_digits(test_vec, radix);
        assert_eq!(binary_digits, vec![
            1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0,
            0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0,
            0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0,
            1, 1, 0, 1, 1, 0, 1, 1]);
    }
        
    #[test]
    fn test_to_binary_digits_3() {
        let test_vec : Vec<u8> = vec![
            5, 7, 5, 2, 2, 1, 0, 8, 6, 6, 0,
            6, 6, 3, 9, 8, 9, 0, 1, 9, 5, 9,
            4, 7, 1, 1, 7, 2, 9, 5, 2, 5, 9, 7, 9];
        let radix : u8 = 10;
        let binary_digits = BigInteger::to_binary_digits(test_vec, radix);
        assert_eq!(binary_digits, vec![
            1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 
            0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0,
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
            1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1,
            0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0,
            0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1,
            0, 1, 1, 0, 1, 1]);
    }

    #[test]
    fn test_to_binary_digits_4() {
        let test_vec : Vec<u8> = vec![1, 5, 2, 2, 7, 3, 0, 6, 2, 0, 1, 2, 3];
        let radix : u8 = 10;
        let binary_digits = BigInteger::to_binary_digits(test_vec, radix);
        assert_eq!(binary_digits, vec![
            1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0,
            0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0,
            0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0,
            1, 1, 0, 1, 1, 0, 1, 1]);
    }

    // #[test]
    // fn test_to_binary_digits_5() {
    //     let test_vec : Vec<u16> = vec![2, 293, 322, 285, 307, 303, 195, 116, 260, 321, 182, 235, 111, 22, 163, 329, 224, 117];
    //     let radix : u16 = 353;
    //     let binary_digits = BigInteger::to_binary_digits(test_vec, radix);
    //     assert_eq!(binary_digits, vec![
    //         1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0,
    //         0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0,
    //         0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,
    //         0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0,
    //         1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1,
    //         0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1,
    //         0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1,
    //         1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1,
    //         0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0,
    //         1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0,
    //         0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0,
    //         0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0,
    //         0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1,
    //         0, 1, 1]);
    // }

    #[test]
    fn test_div_reversed_digits_by_1() {
        // 156
        let mut test_vec : Vec<u8> = vec![0, 0, 1, 5, 6];
        let radix = 10;
        test_vec.reverse();

        // 156 / 2 = 78
        let mod_result = BigInteger::div_reversed_digits_by_2(&mut test_vec, radix);
        assert_eq!(test_vec, vec![8, 7]);
        assert_eq!(mod_result, 0);

        // 78 / 2 = 39
        let mod_result = BigInteger::div_reversed_digits_by_2(&mut test_vec, radix);
        assert_eq!(test_vec, vec![9, 3]);
        assert_eq!(mod_result, 0);

        // 39 / 2 = 19
        let mod_result = BigInteger::div_reversed_digits_by_2(&mut test_vec, radix);
        assert_eq!(test_vec, vec![9, 1]);
        assert_eq!(mod_result, 1);

        // 19 / 2 = 9
        let mod_result = BigInteger::div_reversed_digits_by_2(&mut test_vec, radix);
        assert_eq!(test_vec, vec![9]);
        assert_eq!(mod_result, 1);

        // 9 / 2 == 4
        let mod_result = BigInteger::div_reversed_digits_by_2(&mut test_vec, radix);
        assert_eq!(test_vec, vec![4]);
        assert_eq!(mod_result, 1);

        // 4 / 2 = 2
        let mod_result = BigInteger::div_reversed_digits_by_2(&mut test_vec, radix);
        assert_eq!(test_vec, vec![2]);
        assert_eq!(mod_result, 0);

        // 2 / 2 = 1
        let mod_result = BigInteger::div_reversed_digits_by_2(&mut test_vec, radix);
        assert_eq!(test_vec, vec![1]);
        assert_eq!(mod_result, 0);

        // 1 / 2 = 0
        let mod_result = BigInteger::div_reversed_digits_by_2(&mut test_vec, radix);
        assert!(test_vec.is_empty());
        assert_eq!(mod_result, 1);
    }

    #[test]
    fn to_digits_with_radix() {
        let b = BigInteger::from_str("123").unwrap();
        let digits = b.to_digits_with_radix(2);
        assert_eq!(digits, vec![1, 2, 3]);
    }

    #[test]
    fn big_integer_from_str_1() {
        let b = BigInteger::from_str("123").unwrap();
        assert_eq!(b.to_string(), "123");
    }
}
