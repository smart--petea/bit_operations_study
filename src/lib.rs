//https://www.youtube.com/watch?v=ZusiKXcz_ac
fn str_to_unsigned(s: &str) -> Result<u8, &str> {
    if let Some(err) = validate_8bits(s)  {
        return Err(err);
    }
    
    Ok(str_to_unsigned_flat(s))
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

fn str_to_unsigned_flat(s: &str) -> u8 {
    let s = s.chars().collect::<Vec<char>>();

    let mut result = 0u8;
    let mut multiply = 1u8;
    for i in 0..=7 {
        let position = 7 - i;

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
    }

    #[test]
    fn test_validate_8bits() {
        let input = "ab";
        assert_eq!(validate_8bits(input), Some("The string's length should be equal to 8"));

        let input = "123456789";
        assert_eq!(validate_8bits(input), Some("The string's length should be equal to 8"));

        let input = "000a0000";
        assert_eq!(validate_8bits(input), Some("String contains symbols other than 0 or 1"));

        let input = "00010000";
        assert_eq!(validate_8bits(input), None);
    }
}
