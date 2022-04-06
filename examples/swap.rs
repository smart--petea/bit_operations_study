extern crate bit_manipulation as bm;

use bm::byte::Byte;

//toggle the kth bit

fn main() {
    let x = Byte::new("01001100").unwrap();
    let y = Byte::new("00011100").unwrap();

    let x = x ^ y.clone();
    let y = x.clone() ^ y;
    let x = x ^ y.clone();

    assert_eq!(x, Byte::new("00011100").unwrap());
    assert_eq!(y, Byte::new("01001100").unwrap());
}
