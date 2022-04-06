extern crate bit_manipulation as bm;

use bm::byte::Byte;

//toggle the kth bit

fn main() {
    let x =    Byte::new("01001100").unwrap();
    let mask = Byte::new("00011100").unwrap();

    let fields = (x & mask) >> 2;
    assert_eq!(fields, Byte::new("00000011").unwrap());
}
