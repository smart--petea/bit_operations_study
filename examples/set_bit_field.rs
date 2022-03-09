extern crate bit_manipulation as bm;

//toggle the kth bit

fn main() {
    let x =    bm::Byte::new("10101001").unwrap();
    let mask = bm::Byte::new("00011100").unwrap();
    let shift = 2;
    let y = bm::Byte::new("00000101").unwrap();

    let result = (x & !mask) | (shift << y);
    let expected = bm::Byte::new("10110101").unwrap();

    assert_eq!(expected, result);
}
