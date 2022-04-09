extern crate bit_manipulation as bm;

use bm::byte::Byte;

//Get the mask for the least significant bit in the arbitrary given input
//
//x - input
//mask = x & (-x)

fn main() {
    let x = Byte::new("01000110").unwrap();
    let mask = x.clone() & (-x);
    let expected_mask = Byte::new("00000010").unwrap();

    assert_eq!(mask, expected_mask);
}
