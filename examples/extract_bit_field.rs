extern crate bit_manipulation as bm;

//toggle the kth bit

fn main() {
    let x =    bm::Byte::new("01001100").unwrap();
    let mask = bm::Byte::new("00011100").unwrap();

    let fields = (x & mask) >> 2;
    assert_eq!(fields, bm::Byte::new("00000011").unwrap());
}
