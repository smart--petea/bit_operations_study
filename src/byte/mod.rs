use crate::byte_new_facade::ByteNewFacade;

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

use std::cmp::Ordering;

const ZERO: bool = false;
const ONE: bool = true;

//https://www.youtube.com/watch?v=ZusiKXcz_ac
#[derive(Debug, Clone)]
pub struct Byte {
    inner: [bool; 8] //0bit, 1bit, ..., 8bit. Changed it in order to simplify the computations
}

impl From<bool> for Byte {
    fn from(b: bool) -> Byte {
        if b {
            Byte::all_ones()
        } else {
            Byte::empty()
        }
    }
}

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

    pub fn all_ones() -> Self {
        Byte {
            inner: [true; 8]
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
        for i in 0..=7 {
            match (self.inner[7-i], other.inner[7-i]) {
                (false, true) => return Some(Ordering::Less), 
                (true, false) => return Some(Ordering::Greater), 
                _ => continue,
            }
        }

        Some(Ordering::Equal)
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
    fn test_byte_from_bool() {
        let byte: Byte = From::<bool>::from(true);
        assert_eq!(Into::<u8>::into(byte), 255);

        let byte: Byte = From::<bool>::from(false);
        assert_eq!(Into::<u8>::into(byte), 0);
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

    #[test]
    fn test_byte_ordering() {
        let a = Byte::new("10110011").unwrap();
        let b = Byte::new("00110011").unwrap();

        assert_eq!(a == a, true);
        assert_eq!(a > b, true);
        assert_eq!(a < b, false);
    }

    #[test]
    fn test_bytenewfacade_from_bool() {
        let byte = Byte::new(true).unwrap();
        assert_eq!(Into::<u8>::into(byte), 1);

        let byte = Byte::new(false).unwrap();
        assert_eq!(Into::<u8>::into(byte), 0);
    }
}
