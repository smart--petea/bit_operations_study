use core::ops::Add;
use core::ops::AddAssign;

use core::ops::Not;

use std::ops::BitAnd;
use std::ops::BitAndAssign;

use std::ops::BitOr;
use std::ops::BitOrAssign;

use std::ops::BitXor;
use std::ops::BitXorAssign;

use std::ops::Shr;
use std::ops::ShrAssign;

use std::ops::Shl;
use std::ops::ShlAssign;

use std::result::Result;
use std::cmp::PartialEq;
use std::cmp::Eq;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct ByteNewFacade {
    bytes: Result<[u8; 8], String>
}

impl ByteNewFacade {
    fn as_bytes(self) -> Result<[u8; 8], String> {
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

    fn transform_2hex_to_8u8(s: &[u8]) -> [u8; 8] {
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

impl From<i8> for ByteNewFacade {
    fn from(mut i: i8) -> ByteNewFacade {
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

//https://www.youtube.com/watch?v=ZusiKXcz_ac
#[derive(Debug, Clone)]
pub struct Byte {
    inner: [bool; 8] //0bit, 1bit, ..., 8bit. Changed it in order to simplify the computations
}

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

    pub fn empty() -> Self {
        Byte {
            inner: [false; 8]
        }
    }

    pub fn one() -> Self {
        let mut inner = [false; 8];
        inner[0] = true;

        Byte {
            inner: inner
        }
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

impl AddAssign for Byte {
    fn add_assign(&mut self, other: Self) {
        let mut overflow = ZERO;
        for i in 0..=7 {
            match (self.inner[i], other.inner[i]) {
                (ONE, ONE) => {
                    self.inner[i] = overflow;
                    overflow = ONE;
                }
                (ONE, ZERO) | (ZERO, ONE) => {
                    self.inner[i] = !overflow;
                    overflow = !self.inner[i];
                }
                _ => {
                    self.inner[i] = overflow;
                    overflow = ZERO;
                }
            }
        }
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

impl BitAnd for Byte {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut byte =  Byte::new([false; 0]).unwrap();

        for i in 0..=7 {
            byte.inner[i] = self.inner[i] && rhs.inner[i];
        }

        byte
    }
}

impl BitAndAssign for Byte {
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..=7 {
            self.inner[i] = self.inner[i] && rhs.inner[i];
        }
    }
}

impl BitOr for Byte {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut byte = Byte::new([false;0]).unwrap();
        for i in 0..=7 {
            byte.inner[i] = self.inner[i] || rhs.inner[i];
        }

        byte
    }
}

impl BitOrAssign for Byte {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..=7 {
            self.inner[i] = self.inner[i] || rhs.inner[i];
        }
    }
}

impl BitXor for &Byte {
    type Output = Byte;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut byte = Byte::new([false;0]).unwrap();
        for i in 0..=7 {
            byte.inner[i] = !(self.inner[i] == rhs.inner[i]);
        }

        byte
    }
}

impl BitXor for Byte {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut byte = Byte::new([false;0]).unwrap();
        for i in 0..=7 {
            byte.inner[i] = !(self.inner[i] == rhs.inner[i]);
        }

        byte
    }
}

impl BitXorAssign for Byte {
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..=7 {
            self.inner[i] = !(self.inner[i] == rhs.inner[i]);
        }
    }
}

impl ShrAssign<usize> for Byte {
    fn shr_assign(&mut self, shift: usize) {
         if shift == 0 {
            return;
        }

        let shift = std::cmp::min(8, shift);

        if shift == 8 {
            self.inner = [false; 8];
            return;
        }

        for i in 0..(8-shift) {
            self.inner[i] = self.inner[i + shift];
        }

        for i in (8-shift)..8 {
            self.inner[i] = false;
        }
    }
}

impl Shr<usize> for Byte {
    type Output = Self;

    fn shr(self, shift: usize) -> Self {
        if shift == 0 {
            return self;
        }

        let mut slf = self.clone();
        let shift = std::cmp::min(8, shift);

        if shift == 8 {
            slf.inner = [false; 8];
            return slf;
        }

        for i in 0..(8-shift) {
            slf.inner[i] = slf.inner[i + shift];
        }

        for i in (8-shift)..8 {
            slf.inner[i] = false;
        }

        slf
    }
}

impl Shl<Byte> for usize {
    type Output = Byte;

    fn shl(self, byte: Byte) -> Self::Output {
        let mut output = Byte::empty();

        if self < 8 {
            for i in 0..(8-self) {
                output.inner[i + self] = byte.inner[i];
            }
        }

        output
    }
}

impl Shl<usize> for Byte {
    type Output = Self;

    fn shl(self, shift: usize) -> Self {
        if shift == 0 {
            return self;
        }

        let mut slf = self.clone();
        let shift = std::cmp::min(8, shift);

        if shift == 8 {
            slf.inner = [false; 8];
            return slf;
        }

        let mut i = (7 - shift) as usize;

        loop {
            slf.inner[i + shift] = slf.inner[i];

            if i == 0 {
                break;
            }

            i = i - 1;
        }

        for j in 0..shift {
            slf.inner[j] = false;
        }

        slf
    }
}

impl ShlAssign<usize> for Byte {
    fn shl_assign(&mut self, shift: usize) {
        let shift = std::cmp::min(8, shift);

        if shift == 8 {
            self.inner = [false; 8];
            return;
        }

        let mut i = (7 - shift) as usize;

        loop {
            self.inner[i + shift] = self.inner[i];

            if i == 0 {
                break;
            }

            i = i - 1;
        }

        for j in 0..shift {
            self.inner[j] = false;
        }
    }
}

impl PartialEq for Byte {
    fn eq(&self, other:&Self) -> bool {

        for i in 0..=7 {
            if self.inner[i] != other.inner[i] {
                return false;
            }
        }

        true
    }
}

impl Eq for Byte {}

impl PartialOrd for Byte {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut i = 0;
        loop {
            if self.inner[i] != other.inner[i] {
                break;
            }

            i = i + 1;
            if i == 8 {
                return Some(Ordering::Equal);
            }
        }

        for i in 0..8 {
            if self.inner[i] < other.inner[i] {
                return Some(Ordering::Greater);
            }
        }

        Some(Ordering::Less)
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

    #[test]
    fn test_validate_8bits() {
        assert_eq!(ByteNewFacade::validate_8bits("ab".as_bytes()), Some("The string's length should be equal to 8".into()));
        assert_eq!(ByteNewFacade::validate_8bits("123456789".as_bytes()), Some("The string's length should be equal to 8".into()));
        assert_eq!(ByteNewFacade::validate_8bits("000a0000".as_bytes()), Some("String contains symbols other than 0 or 1".into()));
        assert_eq!(ByteNewFacade::validate_8bits("00010000".as_bytes()), None);
    }

    #[test]
    fn test_add_assign() {
        let mut left = Byte::new("00000000").unwrap();
        let right = Byte::new("00000000").unwrap();
        left += right;
        assert_eq!(Into::<u8>::into(left), 0u8);

        let mut left = Byte::new( "00000001").unwrap();
        let right = Byte::new("00000000").unwrap();
        left += right;
        assert_eq!(Into::<u8>::into(left), 1u8);

        let mut left = Byte::new( "00000001").unwrap();
        let right = Byte::new("00000001").unwrap();
        left += right;
        assert_eq!(Into::<u8>::into(left), 2u8);

        let mut left = Byte::new( "00000011").unwrap();
        let right = Byte::new("00000001").unwrap();
        left += right;
        assert_eq!(Into::<u8>::into(left), 4u8);

        let mut left = Byte::new( "00000011").unwrap();
        let right = Byte::new("00000011").unwrap();
        left += right;
        assert_eq!(Into::<u8>::into(left), 6u8);

        let mut left = Byte::new( "00000111").unwrap();
        let right = Byte::new("00000011").unwrap();
        left += right;
        assert_eq!(Into::<u8>::into(left), 10u8);

        let left = Byte::new( "00000101").unwrap();
        let right = Byte::new("00000011").unwrap();
        let sum = left + right;
        assert_eq!(Into::<u8>::into(sum), 8u8);
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
    fn test_byte_from_slice_bools() {
        let err = Byte::new(&[true; 1] as &[bool]);
        assert_eq!(err.err(), Some("length of the slice should be either zero or 8".into()));

        let byte = Byte::new(&[true; 0] as &[bool]).unwrap();
        assert_eq!(byte.to_hex(), "00");


        let byte = Byte::new(&[false, false, false, false, false, false, false, false] as &[bool]).unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new(&[false, false, false, true, false, false, true, false] as &[bool]).unwrap();
        assert_eq!(byte.to_hex(), "12");

        let byte = Byte::new(&[false, false, true, true, false, true, false, false] as &[bool]).unwrap();
        assert_eq!(byte.to_hex(), "34");

        let byte = Byte::new(&[false, true, false, true, false, true, true, false] as &[bool]).unwrap();
        assert_eq!(byte.to_hex(), "56");

        let byte = Byte::new(&[false, true, true, true, true, false, false, false] as &[bool]).unwrap();
        assert_eq!(byte.to_hex(), "78");

        let byte = Byte::new(&[true, false, false, true, true, false, true, false] as &[bool]).unwrap();
        assert_eq!(byte.to_hex(), "9A");

        let byte = Byte::new(&[true, false, true, true, true, true, false, false] as &[bool]).unwrap();
        assert_eq!(byte.to_hex(), "BC");

        let byte = Byte::new(&[true, true, false, true, true, true, true, false] as &[bool]).unwrap();
        assert_eq!(byte.to_hex(), "DE");

        let byte = Byte::new(&[true; 8] as &[bool]).unwrap();
        assert_eq!(byte.to_hex(), "FF");
    }

    #[test]
    fn test_byte_from_array_u8_len8() {
        let byte = Byte::new([0u8; 0]).unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new([0u8; 8]).unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new([0u8, 0, 0, 1, 0, 0, 1, 0]).unwrap();
        assert_eq!(byte.to_hex(), "12");

        let byte = Byte::new([0u8, 0u8, 1u8, 1u8, 0u8, 1u8, 0u8, 0u8]).unwrap();
        assert_eq!(byte.to_hex(), "34");

        let byte = Byte::new([0u8, 1, 0, 1, 0, 1, 1, 0]).unwrap();
        assert_eq!(byte.to_hex(), "56");

        let byte = Byte::new([0u8, 1, 1, 1, 1, 0, 0, 0]).unwrap();
        assert_eq!(byte.to_hex(), "78");

        let byte = Byte::new([1u8, 0, 0, 1, 1, 0, 1, 0]).unwrap();
        assert_eq!(byte.to_hex(), "9A");

        let byte = Byte::new([1u8, 0, 1, 1, 1, 1, 0, 0]).unwrap();
        assert_eq!(byte.to_hex(), "BC");

        let byte = Byte::new([1u8, 1, 0, 1, 1, 1, 1, 0]).unwrap();
        assert_eq!(byte.to_hex(), "DE");

        let byte = Byte::new([1u8; 8]).unwrap();
        assert_eq!(byte.to_hex(), "FF");
    }

    #[test]
    fn test_byte_from_slice_u8_len8() {
        let byte = Byte::new(&[0u8; 0] as &[u8]).unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new(&[0u8; 8] as &[u8]).unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new(&[0u8, 0, 0, 1, 0, 0, 1, 0] as &[u8]).unwrap();
        assert_eq!(byte.to_hex(), "12");

        let byte = Byte::new(&[0u8, 0u8, 1u8, 1u8, 0u8, 1u8, 0u8, 0u8] as &[u8]).unwrap();
        assert_eq!(byte.to_hex(), "34");

        let byte = Byte::new(&[0u8, 1, 0, 1, 0, 1, 1, 0] as &[u8]).unwrap();
        assert_eq!(byte.to_hex(), "56");

        let byte = Byte::new(&[0u8, 1, 1, 1, 1, 0, 0, 0] as &[u8]).unwrap();
        assert_eq!(byte.to_hex(), "78");

        let byte = Byte::new(&[1u8, 0, 0, 1, 1, 0, 1, 0] as &[u8]).unwrap();
        assert_eq!(byte.to_hex(), "9A");

        let byte = Byte::new(&[1u8, 0, 1, 1, 1, 1, 0, 0] as &[u8]).unwrap();
        assert_eq!(byte.to_hex(), "BC");

        let byte = Byte::new(&[1u8, 1, 0, 1, 1, 1, 1, 0] as &[u8]).unwrap();
        assert_eq!(byte.to_hex(), "DE");

        let byte = Byte::new(&[1u8; 8] as &[u8]).unwrap();
        assert_eq!(byte.to_hex(), "FF");
    }

    #[test]
    fn test_byte_from_bools_len8() {
        let byte = Byte::new([false; 0]).unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new([false, false, false, false, false, false, false, false]).unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new([false, false, false, true, false, false, true, false]).unwrap();
        assert_eq!(byte.to_hex(), "12");

        let byte = Byte::new([false, false, true, true, false, true, false, false]).unwrap();
        assert_eq!(byte.to_hex(), "34");

        let byte = Byte::new([false, true, false, true, false, true, true, false]).unwrap();
        assert_eq!(byte.to_hex(), "56");

        let byte = Byte::new([false, true, true, true, true, false, false, false]).unwrap();
        assert_eq!(byte.to_hex(), "78");

        let byte = Byte::new([true, false, false, true, true, false, true, false]).unwrap();
        assert_eq!(byte.to_hex(), "9A");

        let byte = Byte::new([true, false, true, true, true, true, false, false]).unwrap();
        assert_eq!(byte.to_hex(), "BC");

        let byte = Byte::new([true, true, false, true, true, true, true, false]).unwrap();
        assert_eq!(byte.to_hex(), "DE");

        let byte = Byte::new([true; 8]).unwrap();
        assert_eq!(byte.to_hex(), "FF");
    }

    #[test]
    fn test_byte_from_i8() {
        /*
        let byte = Byte::new(0 as i8).unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new(0x12 as i8).unwrap();
        assert_eq!(byte.to_hex(), "12");

        let byte = Byte::new(0x34 as i8).unwrap();
        assert_eq!(byte.to_hex(), "34");

        let byte = Byte::new(0x56 as i8).unwrap();
        assert_eq!(byte.to_hex(), "56");

        let byte = Byte::new(0x78 as i8).unwrap();
        assert_eq!(byte.to_hex(), "78");

        let byte = Byte::new(0x7F as i8).unwrap(); //127
        assert_eq!(byte.to_hex(), "7F");
        */

        let byte = Byte::new(-1i8).unwrap();
        assert_eq!(byte.to_hex(), "FF");

        let byte = Byte::new(-2i8).unwrap();
        assert_eq!(byte.to_hex(), "FE");

        let byte = Byte::new(-3i8).unwrap();
        assert_eq!(byte.to_hex(), "FD");

        let byte = Byte::new(-127i8).unwrap();
        assert_eq!(byte.to_hex(), "81");

        let byte = Byte::new(-128i8).unwrap();
        assert_eq!(byte.to_hex(), "80");
    }

    #[test]
    fn test_byte_from_u8() {
        let byte = Byte::new(0x00 as u8).unwrap();
        assert_eq!(byte.to_hex(), "00");

        let byte = Byte::new(0x12 as u8).unwrap();
        assert_eq!(byte.to_hex(), "12");

        let byte = Byte::new(0x34 as u8).unwrap();
        assert_eq!(byte.to_hex(), "34");

        let byte = Byte::new(0x56 as u8).unwrap();
        assert_eq!(byte.to_hex(), "56");

        let byte = Byte::new(0x78 as u8).unwrap();
        assert_eq!(byte.to_hex(), "78");

        let byte = Byte::new(0x9A as u8).unwrap();
        assert_eq!(byte.to_hex(), "9A");

        let byte = Byte::new(0xBC as u8).unwrap();
        assert_eq!(byte.to_hex(), "BC");

        let byte = Byte::new(0xDE as u8).unwrap();
        assert_eq!(byte.to_hex(), "DE");

        let byte = Byte::new(0xFF as u8).unwrap();
        assert_eq!(byte.to_hex(), "FF");
    }

    #[test]
    fn test_byte_from_slice_str_len8() {
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
    }

    #[test]
    fn test_byte_from_slice_str_len2() {
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
    fn test_byte_from_string() {
        let byte = Byte::new( String::from("00")).unwrap();
        assert_eq!(byte.to_hex(), "00");
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
    }

    #[test]
    fn test_transform_2hex_to_8u8() {
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("01".as_bytes()), [0, 0, 0, 0,   0, 0, 0, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("23".as_bytes()), [0, 0, 1, 0,   0, 0, 1, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("45".as_bytes()), [0, 1, 0, 0,   0, 1, 0, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("67".as_bytes()), [0, 1, 1, 0,   0, 1, 1, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("89".as_bytes()), [1, 0, 0, 0,   1, 0, 0, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("AB".as_bytes()), [1, 0, 1, 0,   1, 0, 1, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("CD".as_bytes()), [1, 1, 0, 0,   1, 1, 0, 1]);
        assert_eq!(ByteNewFacade::transform_2hex_to_8u8("EF".as_bytes()), [1, 1, 1, 0,   1, 1, 1, 1]);
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

    #[test]
    fn test_byte_bitand() {
        let a = Byte::new("10110011").unwrap();
        let b = Byte::new("01101001").unwrap();

        let a_and_b = Byte::new("00100001").unwrap();

        assert_eq!(a & b, a_and_b);
    }

    #[test]
    fn test_byte_bitor() {
        let a = Byte::new("10110011").unwrap();
        let b = Byte::new("01101001").unwrap();

        let a_or_b = Byte::new("11111011").unwrap();

        assert_eq!(a | b, a_or_b);
    }

    #[test]
    fn test_byte_bitor_assign() {
        let mut a = Byte::new("10110011").unwrap();
        let b = Byte::new("01101001").unwrap();

        let a_or_b = Byte::new("11111011").unwrap();

        a |= b;
        assert_eq!(a, a_or_b);
    }

    #[test]
    fn test_byte_bitxor_assign() {
        let mut a = Byte::new("10110011").unwrap();
        let b = Byte::new("01101001").unwrap();

        let a_xor_b = Byte::new("11011010").unwrap();

        a ^= b;
        assert_eq!(a, a_xor_b);
    }

    #[test]
    fn test_refbyte_bitxor() {
        let a = Byte::new("10110011").unwrap();
        let b = Byte::new("01101001").unwrap();

        let a_xor_b = Byte::new("11011010").unwrap();

        let result = &a ^ &b;
        assert_eq!(result, a_xor_b);
    }

    #[test]
    fn test_byte_shl() {
        let a = Byte::new("10110011").unwrap();
        let a_shl0 = Byte::new("10110011").unwrap();
        let a = a << 0;
        assert_eq!(a, a_shl0);

        let a = Byte::new("10110011").unwrap();
        let a_shl1 = Byte::new("01100110").unwrap();
        let a = a << 1;
        assert_eq!(a, a_shl1);

        let a = Byte::new("10110011").unwrap();
        let a_shl2 = Byte::new("11001100").unwrap();
        let a = a << 2;
        assert_eq!(a, a_shl2);

        let a = Byte::new("10110011").unwrap();
        let a_shl3 = Byte::new("10011000").unwrap();
        let a = a << 3;
        assert_eq!(a, a_shl3);

        let a = Byte::new("10110011").unwrap();
        let a_shl4 = Byte::new("00110000").unwrap();
        let a = a << 4;
        assert_eq!(a, a_shl4);

        let a = Byte::new("10110011").unwrap();
        let a_shl5 = Byte::new("01100000").unwrap();
        let a = a << 5;
        assert_eq!(a, a_shl5);

        let a = Byte::new("10110011").unwrap();
        let a_shl6 = Byte::new("11000000").unwrap();
        let a = a << 6;
        assert_eq!(a, a_shl6);

        let a = Byte::new("10110011").unwrap();
        let a_shl7 = Byte::new("10000000").unwrap();
        let a = a << 7;
        assert_eq!(a, a_shl7);

        let a = Byte::new("10110011").unwrap();
        let a_shl8 = Byte::new("00000000").unwrap();
        let a = a << 8;
        assert_eq!(a, a_shl8);

        let a = Byte::new("10110011").unwrap();
        let a_shl108 = Byte::new("00000000").unwrap();
        let a = a << 108;
        assert_eq!(a, a_shl108);
    }

    #[test]
    fn test_byte_shl_assign() {
        let mut a = Byte::new("10110011").unwrap();
        let a_shl0 = Byte::new("10110011").unwrap();
        a <<= 0;
        assert_eq!(a, a_shl0);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl1 = Byte::new("01100110").unwrap();
        a <<= 1;
        assert_eq!(a, a_shl1);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl2 = Byte::new("11001100").unwrap();
        a <<= 2;
        assert_eq!(a, a_shl2);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl3 = Byte::new("10011000").unwrap();
        a <<= 3;
        assert_eq!(a, a_shl3);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl4 = Byte::new("00110000").unwrap();
        a <<= 4;
        assert_eq!(a, a_shl4);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl5 = Byte::new("01100000").unwrap();
        a <<= 5;
        assert_eq!(a, a_shl5);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl6 = Byte::new("11000000").unwrap();
        a <<= 6;
        assert_eq!(a, a_shl6);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl7 = Byte::new("10000000").unwrap();
        a <<= 7;
        assert_eq!(a, a_shl7);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl8 = Byte::new("00000000").unwrap();
        a <<= 8;
        assert_eq!(a, a_shl8);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl108 = Byte::new("00000000").unwrap();
        a <<= 108;
        assert_eq!(a, a_shl108);
    }

    #[test]
    fn test_byte_shr() {
        let a = Byte::new("10110011").unwrap();
        let a_shl0 = Byte::new("10110011").unwrap();
        let a = a >> 0;
        assert_eq!(a, a_shl0);

        let a = Byte::new("10110011").unwrap();
        let a_shl1 = Byte::new("01011001").unwrap();
        let a = a >> 1;
        assert_eq!(a, a_shl1);

        let a = Byte::new("10110011").unwrap();
        let a_shl2 = Byte::new("00101100").unwrap();
        let a = a >> 2;
        assert_eq!(a, a_shl2);

        let a = Byte::new("10110011").unwrap();
        let a_shl3 = Byte::new("00010110").unwrap();
        let a = a >> 3;
        assert_eq!(a, a_shl3);

        let a = Byte::new("10110011").unwrap();
        let a_shl4 = Byte::new("00001011").unwrap();
        let a = a >> 4;
        assert_eq!(a, a_shl4);

        let a = Byte::new("10110011").unwrap();
        let a_shl5 = Byte::new("00000101").unwrap();
        let a = a >> 5;
        assert_eq!(a, a_shl5);

        let a = Byte::new("10110011").unwrap();
        let a_shl6 = Byte::new("00000010").unwrap();
        let a = a >> 6;
        assert_eq!(a, a_shl6);

        let a = Byte::new("10110011").unwrap();
        let a_shl7 = Byte::new("00000001").unwrap();
        let a = a >> 7;
        assert_eq!(a, a_shl7);

        let a = Byte::new("10110011").unwrap();
        let a_shl8 = Byte::new("00000000").unwrap();
        let a = a >> 8;
        assert_eq!(a, a_shl8);

        let a = Byte::new("10110011").unwrap();
        let a_shl108 = Byte::new("00000000").unwrap();
        let a = a >> 108;
        assert_eq!(a, a_shl108);
    }

    #[test]
    fn test_byte_shr_assign() {
        let mut a = Byte::new("10110011").unwrap();
        let a_shl0 = Byte::new("10110011").unwrap();
        a >>= 0;
        assert_eq!(a, a_shl0);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl1 = Byte::new("01011001").unwrap();
        a >>= 1;
        assert_eq!(a, a_shl1);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl2 = Byte::new("00101100").unwrap();
        a >>= 2;
        assert_eq!(a, a_shl2);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl3 = Byte::new("00010110").unwrap();
        a >>= 3;
        assert_eq!(a, a_shl3);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl4 = Byte::new("00001011").unwrap();
        a >>= 4;
        assert_eq!(a, a_shl4);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl5 = Byte::new("00000101").unwrap();
        a >>= 5;
        assert_eq!(a, a_shl5);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl6 = Byte::new("00000010").unwrap();
        a >>= 6;
        assert_eq!(a, a_shl6);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl7 = Byte::new("00000001").unwrap();
        a >>= 7;
        assert_eq!(a, a_shl7);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl8 = Byte::new("00000000").unwrap();
        a >>= 8;
        assert_eq!(a, a_shl8);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl108 = Byte::new("00000000").unwrap();
        a >>= 108;
        assert_eq!(a, a_shl108);
    }

    #[test]
    fn test_byte_shl_usize() {
        let mut a = Byte::new("10110011").unwrap();
        let a_shl0 = Byte::new("10110011").unwrap();
        a = 0 << a;
        assert_eq!(a, a_shl0);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl1 = Byte::new("01100110").unwrap();
        a = 1 << a;
        assert_eq!(a, a_shl1);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl2 = Byte::new("11001100").unwrap();
        a = 2 << a;
        assert_eq!(a, a_shl2);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl3 = Byte::new("10011000").unwrap();
        a = 3 << a;
        assert_eq!(a, a_shl3);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl4 = Byte::new("00110000").unwrap();
        a = 4 << a;
        assert_eq!(a, a_shl4);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl5 = Byte::new("01100000").unwrap();
        a = 5 << a;
        assert_eq!(a, a_shl5);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl6 = Byte::new("11000000").unwrap();
        a = 6 << a;
        assert_eq!(a, a_shl6);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl7 = Byte::new("10000000").unwrap();
        a = 7 << a;
        assert_eq!(a, a_shl7);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl8 = Byte::new("00000000").unwrap();
        a = 8 << a;
        assert_eq!(a, a_shl8);

        let mut a = Byte::new("10110011").unwrap();
        let a_shl108 = Byte::new("00000000").unwrap();
        a = 108 << a;
        assert_eq!(a, a_shl108);
    }
}
