use core::ops::Add;

//https://www.youtube.com/watch?v=ZusiKXcz_ac
struct Byte {
    inner: [u8; 8]
}

const U8_0: u8 = '0' as u8;
const U8_1: u8 = '1' as u8;

impl Byte {
    pub fn new(s: String) -> Result<Self, String> {
        let s = s.as_bytes();
        match Self::validate_8bits(&s) {
            Some(err) => Err(err),
            None => Ok(
                Byte{
                inner: Byte::slice_to_array(s)
            })
        }
    }

    fn slice_to_array<'a, T>(slice: T) -> [u8; 8]
    where T: Into<&'a [u8]>
    {
        let slice = slice.into();
        let mut ar = [0u8; 8];

        for i in 0..=7 {
            ar[i] = slice[7-i];
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

    pub fn to_signed(s: &[u8]) -> i8 {
        let unsigned_part = Self::to_unsigned(&s[..7]);
        let signed_part = match *s.last().unwrap() {
            U8_0 => {0i8},
            _ => {-128i8}
        };

        signed_part + (unsigned_part as i8)
    }

    fn to_unsigned(s: &[u8]) -> u8 {
        let mut result = 0u8;
        let mut multiply = 1u8;
        let len = s.len() - 1;
        for &u in s {
            match u {
                U8_1 => {
                    result = result + multiply;
                }
                _ => {}
            }

            multiply = multiply << 1;
        }

        result
    }

    //todo
    fn sum_str(left: &String, right: &String) -> String {
        let left = left.as_bytes();
        assert_eq!(left.len(), 8);

        let right = right.as_bytes();
        assert_eq!(right.len(), 8);


        for i in 0..=7 {
        }

        String::new()
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

/*todo
impl Add for Byte {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let sum = Self::sum_str(&self.inner, &other.inner);

        Self::new(sum).unwrap()
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_u8() {
        let byte = Byte::new("10000011".into()).unwrap();
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
        let input =  as_reversed_bytes("00000001");
        assert_eq!(Byte::to_unsigned(&input[..]), 1);

        let input = as_reversed_bytes("00000011");
        assert_eq!(Byte::to_unsigned(&input[..]), 3);

        let input = as_reversed_bytes("10000011");
        assert_eq!(Byte::to_unsigned(&input[..]), 131);
    }

    #[test]
    fn test_to_signed() {
        let input = as_reversed_bytes("10010110");
        assert_eq!(Byte::to_signed(&input[..]), -106);

        let input = as_reversed_bytes("10000000");
        assert_eq!(Byte::to_signed(&input[..]), -128);

        let input = as_reversed_bytes("00000100");
        assert_eq!(Byte::to_signed(&input[..]), 4);
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
}
