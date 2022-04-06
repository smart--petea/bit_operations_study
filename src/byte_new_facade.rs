const U8_0: u8 = '0' as u8;
const U8_1: u8 = '1' as u8;
const U8_2: u8 = '2' as u8;
const U8_3: u8 = '3' as u8;
const U8_4: u8 = '4' as u8;
const U8_5: u8 = '5' as u8;
const U8_6: u8 = '6' as u8;
const U8_7: u8 = '7' as u8;
const U8_8: u8 = '8' as u8;
const U8_9: u8 = '9' as u8;
const U8_A: u8 = 'A' as u8;
const U8_B: u8 = 'B' as u8;
const U8_C: u8 = 'C' as u8;
const U8_D: u8 = 'D' as u8;
const U8_E: u8 = 'E' as u8;
const U8_F: u8 = 'F' as u8;

#[derive(Debug)]
pub struct ByteNewFacade {
    bytes: Result<[u8; 8], String>
}

impl ByteNewFacade {
    pub fn as_bytes(self) -> Result<[u8; 8], String> {
        self.bytes
    }

    fn validate_8bits(s: &[u8]) -> Option<String> {
        if s.len() != 8 {
            return Some("The string's length should be equal to 8".into());
        }

        for &c in s {
            match c {
                U8_0 | U8_1 | 0 | 1=> { }
                _ => {
                    return Some("String contains symbols other than 0 or 1".into());
                }
            }
        }

        None
    }

    fn validate_2hex(input: &[u8]) -> Option<String> {
        if input.len() != 2 {
            return Some("The u8 slice's length in hex representation should be equal to 2".into());
        }

        for &c in input {
            match c {
                U8_0..=U8_9 => { }
                U8_A..=U8_F => { }
                _ => {
                    return Some("u8 slice contains symbols other than 0..9 and A-F".into());
                }
            }
        }

        None
    }

    pub fn transform_2hex_to_8u8(s: &[u8]) -> [u8; 8] {
        let mut result = [0u8; 8];
        let mut i = 0;

        for &c in s {
            for bit in Self::transform_u8_to_4bits(c) {
                result[i] = bit;
                i = i + 1;
            }
        }

        result
    }

    fn transform_u8_to_4bits(c: u8) ->  [u8; 4] {
        match c {
            U8_0 => [0u8, 0u8, 0u8, 0u8],
            U8_1 => [0u8, 0u8, 0u8, 1u8],
            U8_2 => [0u8, 0u8, 1u8, 0u8],
            U8_3 => [0u8, 0u8, 1u8, 1u8],
            U8_4 => [0u8, 1u8, 0u8, 0u8],
            U8_5 => [0u8, 1u8, 0u8, 1u8],
            U8_6 => [0u8, 1u8, 1u8, 0u8],
            U8_7 => [0u8, 1u8, 1u8, 1u8],
            U8_8 => [1u8, 0u8, 0u8, 0u8],
            U8_9 => [1u8, 0u8, 0u8, 1u8],
            U8_A => [1u8, 0u8, 1u8, 0u8],
            U8_B => [1u8, 0u8, 1u8, 1u8],
            U8_C => [1u8, 1u8, 0u8, 0u8],
            U8_D => [1u8, 1u8, 0u8, 1u8],
            U8_E => [1u8, 1u8, 1u8, 0u8],
            U8_F => [1u8, 1u8, 1u8, 1u8],
            _ => panic!("wrong char to be trasnformed in hex {}", c)
        }
    }
}

impl From<bool> for ByteNewFacade {
    fn from(b: bool) -> ByteNewFacade {
        if b {
            let mut bytes = [0u8; 8]; 
            bytes[7] = 1;

            ByteNewFacade {
                bytes: Ok(bytes)
            }
        } else {
            ByteNewFacade {
                bytes: Ok([0u8; 8])
            }
        }
    }
}

impl From<i8> for ByteNewFacade {
    fn from(i: i8) -> ByteNewFacade {
        if i >= 0 {
            return From::<u8>::from(i as u8);
        }

        if i == -128 {
            let mut bits_ar = [0u8; 8];
            bits_ar[0] = 1;

            return ByteNewFacade {
                bytes: Ok(bits_ar)
            };
        }

        let mut u = (-i) as u8;
        let mut bits_vec = std::vec::Vec::new();

        while u > 0 {
            bits_vec.push(u%2);
            u = u / 2;
        }

        let mut bits_ar = [0u8; 8];
        for ii in 0..bits_vec.len() {
            bits_ar[7-ii] = bits_vec[ii];
        }

        let mut add = 1;
        for ii in 0..8 {
            let not = bits_ar[7-ii]^1;
            match not + add {
                2 => {
                    add = 1;
                    bits_ar[7-ii] = 0;
                }
                1 => {
                    add = 0;
                    bits_ar[7-ii] = 1;
                }
                _ => {
                    add = 0;
                    bits_ar[7-ii] = 0;
                }
            }
        }

        ByteNewFacade {
            bytes: Ok(bits_ar)
        }
    }
}

impl From<u8> for ByteNewFacade {
    fn from(mut u: u8) -> ByteNewFacade {
        let mut bits_vec = std::vec::Vec::new();

        while u > 0 {
            bits_vec.push(u%2);
            u = u / 2;
        }

        let mut bits_ar = [0u8; 8];

        for i in 0..bits_vec.len() {
            bits_ar[7-i] = bits_vec[i];
        }


        ByteNewFacade {
            bytes: Ok(bits_ar)
        }
    }
}

impl From<[bool; 0]> for ByteNewFacade {
    fn from(_: [bool; 0]) -> ByteNewFacade {
        ByteNewFacade {
            bytes: Ok([0u8; 8])
        }
    }
}

impl From<[bool; 8]> for ByteNewFacade {
    fn from(bools: [bool; 8]) -> ByteNewFacade {
        From::<&[bool]>::from(&bools)
    }
}

impl From<[u8; 8]> for ByteNewFacade {
    fn from(input: [u8; 8]) -> ByteNewFacade {
        From::<&[u8]>::from(&input as &[u8])
    }
} 

impl From<[u8; 0]> for ByteNewFacade {
    fn from(_: [u8; 0]) -> ByteNewFacade {
        ByteNewFacade {
            bytes: Ok([0u8; 8])
        }
    }
} 

impl From<&[u8]> for ByteNewFacade {
    fn from(input: &[u8]) -> ByteNewFacade {
        if input.len() == 0 {
            return ByteNewFacade {
                bytes: Ok([0u8; 8])
            };
        }

        if Self::validate_8bits(input).is_none() {
            let mut bytes = [0u8; 8];
            let mut i = 0;
            for &c in input {
                bytes[i] = match c {
                    U8_0 => 0,
                    U8_1 => 1,
                    0 => 0,
                    1 => 1,
                    _ => panic!("unreacheable")
                };

                i = i + 1;
            }

            return ByteNewFacade {
                bytes: Ok(bytes)
            };
        }

        if Self::validate_2hex(input).is_none() {
            return ByteNewFacade {
                bytes: Ok(Self::transform_2hex_to_8u8(input))
            }
        }

        ByteNewFacade {
            bytes: Err("Can't deduce ByteNewFacade".into())
        }
    }
}

impl From<&[bool]> for ByteNewFacade {
    fn from(bools: &[bool]) -> ByteNewFacade {
        if bools.len() == 0 {
            return ByteNewFacade {
                bytes: Ok([0u8; 8])
            };
        }

        if bools.len() == 8 {
            let mut bytes = [0u8; 8];
            let mut i = 0;

            for &bl in bools {
                bytes[i] = if bl { 1 } else { 0 };
                i = i + 1;
            }

            return ByteNewFacade {
                bytes: Ok(bytes)
            };
        }

        ByteNewFacade {
            bytes: Err("length of the slice should be either zero or 8".into())
        }
    }
}

impl From<String> for ByteNewFacade {
    fn from(s: String) -> ByteNewFacade {
        From::<&str>::from(&s)
    }
}

impl From<&str> for ByteNewFacade {
    fn from(l: &str) -> ByteNewFacade {
        From::<&[u8]>::from(l.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_8bits() {
        assert_eq!(ByteNewFacade::validate_8bits("ab".as_bytes()), Some("The string's length should be equal to 8".into()));
        assert_eq!(ByteNewFacade::validate_8bits("123456789".as_bytes()), Some("The string's length should be equal to 8".into()));
        assert_eq!(ByteNewFacade::validate_8bits("000a0000".as_bytes()), Some("String contains symbols other than 0 or 1".into()));
        assert_eq!(ByteNewFacade::validate_8bits("00010000".as_bytes()), None);
    }

    #[test]
    fn test_validate_2hex() {
        assert_eq!(ByteNewFacade::validate_2hex("ABC".as_bytes()), Some("The u8 slice's length in hex representation should be equal to 2".into()));
        assert_eq!(ByteNewFacade::validate_2hex("01".as_bytes()), None);
        assert_eq!(ByteNewFacade::validate_2hex("23".as_bytes()), None);
        assert_eq!(ByteNewFacade::validate_2hex("45".as_bytes()), None);
        assert_eq!(ByteNewFacade::validate_2hex("67".as_bytes()), None);
        assert_eq!(ByteNewFacade::validate_2hex("89".as_bytes()), None);
        assert_eq!(ByteNewFacade::validate_2hex("AB".as_bytes()), None);
        assert_eq!(ByteNewFacade::validate_2hex("CD".as_bytes()), None);
        assert_eq!(ByteNewFacade::validate_2hex("EF".as_bytes()), None);

        assert_eq!(ByteNewFacade::validate_2hex("GH".as_bytes()), Some("u8 slice contains symbols other than 0..9 and A-F".into()));
    }
}
