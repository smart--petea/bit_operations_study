//https://www.youtube.com/watch?v=ZusiKXcz_ac
fn str_to_unsigned(s: &str) -> Result<u8, &str> {
    if s.len() != 8 {
        return Err("The string's length should be equal to 8");
    }

    let s = s.chars().collect::<Vec<char>>();

    let mut result = 0u8;
    let mut multiply = 1u8;
    for i in 0..=7 {
        let position = 7 - i;

        match s[position] {
            '1' => {
                result = result + multiply;
            }
            '0' => {}
            _ => {
                return Err("String contains symbols other than 0 or 1");
            }
        }

        multiply = multiply << 1;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_unsigned() {
        let input = "ab";
        assert_eq!(str_to_unsigned(input), Err("The string's length should be equal to 8"));

        let input = "123456789";
        assert_eq!(str_to_unsigned(input), Err("The string's length should be equal to 8"));

        let input = "000a0000";
        assert_eq!(str_to_unsigned(input), Err("String contains symbols other than 0 or 1"));

        let input = "00000001";
        assert_eq!(str_to_unsigned(input), Ok(1));

        let input = "00000011";
        assert_eq!(str_to_unsigned(input), Ok(3));

        let input = "10000011";
        assert_eq!(str_to_unsigned(input), Ok(131));
    }
}
