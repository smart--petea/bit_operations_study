//https://www.youtube.com/watch?v=ZusiKXcz_ac
struct Byte<'a> {
    inner: &'a str
}

impl<'a> Byte<'a> {
    pub fn new(s: &'a str) -> Result<Self, &'a str> {
        match Self::validate_8bits(s) {
            Some(err) => Err(err),
            None => Ok(Byte{
                inner: s
            })
        }
    }

    fn validate_8bits(s: &str) -> Option<&str> {
        if s.len() != 8 {
            return Some("The string's length should be equal to 8");
        }

        for c in s.chars() {
            match c {
                '0' | '1' => { }
                _ => {
                    return Some("String contains symbols other than 0 or 1");
                }
            }
        }

        None
    }
}

fn str_to_unsigned_flat(s: &str) -> u8 {
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

fn str_to_signed_flat(s: &str) -> i8 {
    let unsigned_part = str_to_unsigned_flat(&s[1..]);
    let signed_part = match s.chars().next().unwrap() {
        '0' => {0i8},
        _ => {-128i8}
    };

    signed_part + (unsigned_part as i8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_unsigned_flat() {
        let input = "00000001";
        assert_eq!(str_to_unsigned_flat(input), 1);

        let input = "00000011";
        assert_eq!(str_to_unsigned_flat(input), 3);

        let input = "10000011";
        assert_eq!(str_to_unsigned_flat(input), 131);

        let input = "00010000";
        assert_eq!(str_to_unsigned_flat(&input[3..]), 16);

    }

    #[test]
    fn test_str_to_signed_flat() {
        assert_eq!(str_to_signed_flat("10010110"), -106);
        assert_eq!(str_to_signed_flat("10000000"), -128);
        assert_eq!(str_to_signed_flat("00000100"), 4);
    }

    #[test]
    fn test_validate_8bits() {
        let input = "ab";
        assert_eq!(Byte::validate_8bits(input), Some("The string's length should be equal to 8"));

        let input = "123456789";
        assert_eq!(Byte::validate_8bits(input), Some("The string's length should be equal to 8"));

        let input = "000a0000";
        assert_eq!(Byte::validate_8bits(input), Some("String contains symbols other than 0 or 1"));

        let input = "00010000";
        assert_eq!(Byte::validate_8bits(input), None);
    }
}
