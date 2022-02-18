use core::ops::Add;
use core::ops::Not;

//https://www.youtube.com/watch?v=ZusiKXcz_ac
#[derive(Debug, Clone)]
struct Byte {
    inner: [bool; 8] //0bit, 1bit, ..., 8bit. Changed it in order to simplify the computations
}

const U8_0: u8 = '0' as u8;
const U8_1: u8 = '1' as u8;

const ZERO: bool = false;
const ONE: bool = true;

impl Byte {
    pub fn new<T: Into<String>>(s: T) -> Result<Self, String> {
        let s = s.into();
        let s = s.as_bytes();
        match Self::validate_8bits(&s) {
            Some(err) => Err(err),
            None => Ok(Byte{ inner: Byte::string_to_bools(s)})
        }
    }

    fn string_to_bools<'a, T>(slice: T) -> [bool; 8]
    where T: Into<&'a [u8]>
    {
        let slice = slice.into();
        let mut ar = [false; 8];

        for i in 0..=7 {
            ar[i] = match slice[7-i] {
                U8_0 => false,
                _ => true,
            } 
        }

        ar
    }

    fn validate_8bits(s: &[u8]) -> Option<String> {
        if s.len() != 8 {
            return Some("The string's length should be equal to 8".into());
        }

        for &c in s {
            match c {
                U8_0 | U8_1 => { }
                _ => {
                    return Some("String contains symbols other than 0 or 1".into());
                }
            }
        }

        None
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

    fn as_reversed_bytes(s: &str) -> Vec<u8> {
        let mut result = Vec::new();
        for &u in s.as_bytes().iter().rev() {
            result.push(u);
        }

        result
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
    fn test_validate_8bits() {
        let input = "ab".as_bytes();
        assert_eq!(Byte::validate_8bits(&input), Some("The string's length should be equal to 8".into()));

        let input = "123456789".as_bytes();
        assert_eq!(Byte::validate_8bits(&input), Some("The string's length should be equal to 8".into()));

        let input = "000a0000".as_bytes();
        assert_eq!(Byte::validate_8bits(&input), Some("String contains symbols other than 0 or 1".into()));

        let input = "00010000".as_bytes();
        assert_eq!(Byte::validate_8bits(&input), None);
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
}
