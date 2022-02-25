use core::ops::Add;
use core::ops::Not;
use std::result::Result;

#[derive(Debug)]
struct ByteNewFacade {
    bytes: Result<[u8; 8], String>
}

impl ByteNewFacade {
    fn as_bytes(self) -> Result<[u8; 8], String> {
        self.bytes
    }

    fn validate_8bits(s: &str) -> Option<String> {
        if s.len() != 8 {
            return Some("The string's length should be equal to 8".into());
        }

        for &c in s.as_bytes() {
            match c {
                U8_0 | U8_1 => { }
                _ => {
                    return Some("String contains symbols other than 0 or 1".into());
                }
            }
        }

        None
    }

    fn validate_2hex(s: &str) -> Option<String> {
        if s.len() != 2 {
            return Some("The string's length in hex representation should be equal to 2".into());
        }

        for &c in s.as_bytes() {
            match c {
                U8_0..=U8_9 => { }
                U8_A..=U8_F => { }
                _ => {
                    return Some("String contains symbols other than 0..9 and A-F".into());
                }
            }
        }

        None
    }

    fn transform_2hex_to_8u8(s: &str) -> [u8; 8] {
        let mut result = [0u8; 8];
        let mut i = 0;

        for c in s.chars() {
            for bit in Self::transform_char_to_4bits(c) {
                result[i] = bit;
                i = i + 1;
            }
        }

        result
    }

    fn transform_char_to_4bits(c: char) ->  [u8; 4] {
        match c {
            '0' => [0u8, 0u8, 0u8, 0u8],
            '1' => [0u8, 0u8, 0u8, 1u8],
            '2' => [0u8, 0u8, 1u8, 0u8],
            '3' => [0u8, 0u8, 1u8, 1u8],
            '4' => [0u8, 1u8, 0u8, 0u8],
            '5' => [0u8, 1u8, 0u8, 1u8],
            '6' => [0u8, 1u8, 1u8, 0u8],
            '7' => [0u8, 1u8, 1u8, 1u8],
            '8' => [1u8, 0u8, 0u8, 0u8],
            '9' => [1u8, 0u8, 0u8, 1u8],
            'A' => [1u8, 0u8, 1u8, 0u8],
            'B' => [1u8, 0u8, 1u8, 1u8],
            'C' => [1u8, 1u8, 0u8, 0u8],
            'D' => [1u8, 1u8, 0u8, 1u8],
            'E' => [1u8, 1u8, 1u8, 0u8],
            'F' => [1u8, 1u8, 1u8, 1u8],
            _ => panic!("wrong char to be trasnformed in hex {}", c)
        }
    }
}

impl<'a> From<&str> for ByteNewFacade {
    fn from(l: &str) -> ByteNewFacade {
        if Self::validate_8bits(l).is_none() {
            let mut bytes = [0u8; 8];
            let mut i = 0;
            for c in l.chars() {
                bytes[i] = match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("unreacheable")
                };

                i = i + 1;
            }

            return ByteNewFacade {
                bytes: Ok(bytes)
            }
        }

        if Self::validate_2hex(l).is_none() {
            return ByteNewFacade {
                bytes: Ok(Self::transform_2hex_to_8u8(l))
            }
        }

        ByteNewFacade {
            bytes: Err("Can't deduce ByteNewFacade".into())
        }
    }
}

//https://www.youtube.com/watch?v=ZusiKXcz_ac
#[derive(Debug, Clone)]
struct Byte {
    inner: [bool; 8] //0bit, 1bit, ..., 8bit. Changed it in order to simplify the computations
}

const U8_0: u8 = '0' as u8;
const U8_1: u8 = '1' as u8;
const U8_9: u8 = '9' as u8;
const U8_A: u8 = 'A' as u8;
const U8_F: u8 = 'F' as u8;

const ZERO: bool = false;
const ONE: bool = true;

impl Byte {
    pub fn new<T: Into<ByteNewFacade>>(b: T) -> Result<Self, String> {
        match b.into().as_bytes() {
            Ok(bytes) => Ok(Byte{ inner: Byte::string_to_bools(&bytes as &[u8])}),
            Err(err) => Err(err) 
        }
    }

    pub fn to_hex(&self) -> String {
        let mut result = String::new();

        result.push(Self::transform_bits_to_char(&self.inner[4..]));
        result.push(Self::transform_bits_to_char(&self.inner[0..4]));

        result
    }

    fn transform_bits_to_char(bits: &[bool]) -> char {
        let u = Self::to_unsigned(bits); 
        match  u {
            0..=9 => std::char::from_digit(u as u32, 10).unwrap(),
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            _  => 'F',
        }

    }

    fn string_to_bools<'a, T>(slice: T) -> [bool; 8]
    where T: Into<&'a [u8]>
    {
        let slice = slice.into();
        let mut ar = [false; 8];

        for i in 0..=7 {
            ar[i] = match slice[7-i] {
                0u8 => false,
                _ => true,
            } 
        }

        ar
    }
    pub fn to_signed(s: &[bool]) -> i8 {
        let unsigned_part = Self::to_unsigned(&s[..7]);
        let signed_part = match *s.last().unwrap() {
            ZERO => {0i8},
            _ => {-128i8}
        };

        signed_part + (unsigned_part as i8)
    }

    fn to_unsigned(s: &[bool]) -> u8 {
        let mut result = 0u8;
        let mut multiply = 1u8;
        for &u in s {
            match u {
                ONE => {
                    result = result + multiply;
                }
                _ => {}
            }

            multiply = multiply << 1;
        }

        result
    }
}

impl Into<i8> for Byte {
    fn into(self) -> i8 {
        Byte::to_signed(&self.inner)
    }
}

impl Into<u8> for Byte {
    fn into(self) -> u8 {
        Byte::to_unsigned(&self.inner)
    }
}

impl Add for Byte {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sum = [false; 8];

        let mut overflow = ZERO;
        for i in 0..=7 {
            match (self.inner[i], other.inner[i]) {
                (ONE, ONE) => {
                    sum[i] = overflow;
                    overflow = ONE;
                }
                (ONE, ZERO) | (ZERO, ONE) => {
                    sum[i] = !overflow;
                    overflow = !sum[i];
                }
                _ => {
                    sum[i] = overflow;
                    overflow = ZERO;
                }
            }
        }

        Self {
            inner: sum
        }
    }
}

impl Not for Byte {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        for i in 0..=7 {
            self.inner[i] = !self.inner[i];
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_u8() {
        let byte = Byte::new("10000011").unwrap();
        assert_eq!(Into::<u8>::into(byte), 131u8);
    }

    #[test]
    fn test_to_unsigned() {
        assert_eq!(Byte::to_unsigned(&[true, false, false, false, false, false, false, false]), 1);
        assert_eq!(Byte::to_unsigned(&[true,  true, false, false, false, false, false, false]), 3);
        assert_eq!(Byte::to_unsigned(&[true,  true, false, false, false, false, false, true]), 131);
    }

    #[test]
    fn test_to_signed() {
        assert_eq!(Byte::to_signed(&[false, true, true, false, true, false, false, true]), -106);
        assert_eq!(Byte::to_signed(&[false, false, false, false, false, false, false, true]), -128);
        assert_eq!(Byte::to_signed(&[false, false, true, false, false, false, false, false]), 4);
    }

    #[test]
    fn test_validate_2hex() {
        assert_eq!(ByteNewFacade::validate_2hex("ABC"), Some("The string's length in hex representation should be equal to 2".into()));
        assert_eq!(ByteNewFacade::validate_2hex("01"), None);
        assert_eq!(ByteNewFacade::validate_2hex("23"), None);
        assert_eq!(ByteNewFacade::validate_2hex("45"), None);
        assert_eq!(ByteNewFacade::validate_2hex("67"), None);
        assert_eq!(ByteNewFacade::validate_2hex("89"), None);
        assert_eq!(ByteNewFacade::validate_2hex("AB"), None);
        assert_eq!(ByteNewFacade::validate_2hex("CD"), None);
        assert_eq!(ByteNewFacade::validate_2hex("EF"), None);

        assert_eq!(ByteNewFacade::validate_2hex("GH"), Some("String contains symbols other than 0..9 and A-F".into()));
    }

    #[test]
    fn test_validate_8bits() {
        assert_eq!(ByteNewFacade::validate_8bits("ab"), Some("The string's length should be equal to 8".into()));
        assert_eq!(ByteNewFacade::validate_8bits("123456789"), Some("The string's length should be equal to 8".into()));
        assert_eq!(ByteNewFacade::validate_8bits("000a0000"), Some("String contains symbols other than 0 or 1".into()));
        assert_eq!(ByteNewFacade::validate_8bits("00010000"), None);
    }

    #[test]
    fn test_add() {
        let left = Byte::new("00000000").unwrap();
        let right = Byte::new("00000000").unwrap();
        let sum = left + right;
        assert_eq!(Into::<u8>::into(sum), 0u8);

        let left = Byte::new( "00000001").unwrap();
        let right = Byte::new("00000000").unwrap();
        let sum = left + right;
        assert_eq!(Into::<u8>::into(sum), 1u8);

        let left = Byte::new( "00000001").unwrap();
        let right = Byte::new("00000001").unwrap();
        let sum = left + right;
        assert_eq!(Into::<u8>::into(sum), 2u8);

        let left = Byte::new( "00000011").unwrap();
        let right = Byte::new("00000001").unwrap();
        let sum = left + right;
        assert_eq!(Into::<u8>::into(sum), 4u8);

        let left = Byte::new( "00000011").unwrap();
        let right = Byte::new("00000011").unwrap();
        let sum = left + right;
        assert_eq!(Into::<u8>::into(sum), 6u8);

        let left = Byte::new( "00000111").unwrap();
        let right = Byte::new("00000011").unwrap();
        let sum = left + right;
        assert_eq!(Into::<u8>::into(sum), 10u8);

        let left = Byte::new( "00000101").unwrap();
        let right = Byte::new("00000011").unwrap();
        let sum = left + right;
        assert_eq!(Into::<u8>::into(sum), 8u8);
    }

    #[test]
    fn test_not() {
        let left = !Byte::new( "00000101").unwrap();
        assert_eq!(Into::<i8>::into(left), -6);
    }

    #[test]
    fn test_minus_one() {
        let left = Byte::new( "00000101").unwrap();
        let sum = !left.clone() + left;

        assert_eq!(Into::<i8>::into(sum), -1);

    }

    #[test]
    fn test_byte_to_hex() {
        let byte = Byte::new("00000000").unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new("00010010").unwrap();
        assert_eq!(byte.to_hex(), "12");

        let byte = Byte::new("00110100").unwrap();
        assert_eq!(byte.to_hex(), "34");

        let byte = Byte::new("01010110").unwrap();
        assert_eq!(byte.to_hex(), "56");

        let byte = Byte::new("01111000").unwrap();
        assert_eq!(byte.to_hex(), "78");

        let byte = Byte::new("10011010").unwrap();
        assert_eq!(byte.to_hex(), "9A");

        let byte = Byte::new("10111100").unwrap();
        assert_eq!(byte.to_hex(), "BC");

        let byte = Byte::new("11011110").unwrap();
        assert_eq!(byte.to_hex(), "DE");

        let byte = Byte::new("11111111").unwrap();
        assert_eq!(byte.to_hex(), "FF");



        let byte = Byte::new("00").unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new("12").unwrap();
        assert_eq!(byte.to_hex(), "12");

        let byte = Byte::new("34").unwrap();
        assert_eq!(byte.to_hex(), "34");

        let byte = Byte::new("56").unwrap();
        assert_eq!(byte.to_hex(), "56");

        let byte = Byte::new("78").unwrap();
        assert_eq!(byte.to_hex(), "78");

        let byte = Byte::new("9A").unwrap();
        assert_eq!(byte.to_hex(), "9A");

        let byte = Byte::new("BC").unwrap();
        assert_eq!(byte.to_hex(), "BC");

        let byte = Byte::new("DE").unwrap();
        assert_eq!(byte.to_hex(), "DE");

        let byte = Byte::new("FF").unwrap();
        assert_eq!(byte.to_hex(), "FF");
    }

    #[test]
    fn test_transform_2hex_to_8u8() {
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("01"), [0, 0, 0, 0,   0, 0, 0, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("23"), [0, 0, 1, 0,   0, 0, 1, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("45"), [0, 1, 0, 0,   0, 1, 0, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("67"), [0, 1, 1, 0,   0, 1, 1, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("89"), [1, 0, 0, 0,   1, 0, 0, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("AB"), [1, 0, 1, 0,   1, 0, 1, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("CD"), [1, 1, 0, 0,   1, 1, 0, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("EF"), [1, 1, 1, 0,   1, 1, 1, 1]);
    }

    #[test]
    fn test_test() {
        let bnf = ByteNewFacade::from("00");
        let byte = Byte::new(bnf).unwrap();
        println!("{:?}", byte);

            /*
        let byte = Byte::new("00").unwrap();
        assert_eq!(byte.to_hex(), "00");
            */
    }
}
