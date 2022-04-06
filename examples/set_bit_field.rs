extern crate bit_manipulation as bm;

use bm::byte::Byte;

//toggle the kth bit

fn main() {
    let x =    Byte::new("10101001").unwrap();
    let mask = Byte::new("00011100").unwrap();
    let shift = 2;
    let y = Byte::new("00000101").unwrap();

    let result = (x & !mask) | (shift << y);
    let expected = Byte::new("10110101").unwrap();

    assert_eq!(expected, result);
}
