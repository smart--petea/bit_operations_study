use core::ops::Add;

//https://www.youtube.com/watch?v=ZusiKXcz_ac
struct Byte {
    inner: String
}

impl Byte {
    pub fn new(s: String) -> Result<Self, String> {
        match Self::validate_8bits(&s) {
            Some(err) => Err(err),
            None => Ok(Byte{
                inner: s
            })
        }
    }

    fn validate_8bits(s: &String) -> Option<String> {
        if s.len() != 8 {
            return Some("The string's length should be equal to 8".into());
        }

        for c in s.chars() {
            match c {
                '0' | '1' => { }
                _ => {
                    return Some("String contains symbols other than 0 or 1".into());
                }
            }
        }

        None
    }

    pub fn to_signed(s: &String) -> i8 {
        let unsigned_part = Self::to_unsigned(&s[1..].into());
        let signed_part = match s.chars().next().unwrap() {
            '0' => {0i8},
            _ => {-128i8}
        };

        signed_part + (unsigned_part as i8)
    }

    fn to_unsigned(s: &String) -> u8 {
        let s = s.chars().collect::<Vec<char>>();

        let mut result = 0u8;
        let mut multiply = 1u8;
        let len = s.len() - 1;
        for i in 0..=len {
            let position = len - i;

            match s[position] {
                '1' => {
                    result = result + multiply;
                }
                _ => {}
            }

            multiply = multiply << 1;
        }

        result
    }

    fn sum_str<'b>(left: &String, right: &String) -> String {
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

impl Add for Byte {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let sum = Self::sum_str(&self.inner, &other.inner);

        Self::new(sum).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_u8() {
        let byte = Byte::new("10000011".into()).unwrap();
        assert_eq!(Into::<u8>::into(byte), 131u8);
    }

    #[test]
    fn test_to_unsigned() {
        assert_eq!(Byte::to_unsigned(&("00000001").into()), 1);
        assert_eq!(Byte::to_unsigned(&("00000011").into()), 3);
        assert_eq!(Byte::to_unsigned(&("10000011").into()), 131);

        let input = "00010000";
        assert_eq!(Byte::to_unsigned(&(&input[3..].into())), 16);
    }

    #[test]
    fn test_to_signed() {
        assert_eq!(Byte::to_signed(&"10010110".into()), -106);
        assert_eq!(Byte::to_signed(&"10000000".into()), -128);
        assert_eq!(Byte::to_signed(&"00000100".into()), 4);
    }

    #[test]
    fn test_validate_8bits() {
        let input = "ab".into();
        assert_eq!(Byte::validate_8bits(&input), Some("The string's length should be equal to 8".into()));

        let input = "123456789".into();
        assert_eq!(Byte::validate_8bits(&input), Some("The string's length should be equal to 8".into()));

        let input = "000a0000".into();
        assert_eq!(Byte::validate_8bits(&input), Some("String contains symbols other than 0 or 1".into()));

        let input = "00010000".into();
        assert_eq!(Byte::validate_8bits(&input), None);
    }
}
