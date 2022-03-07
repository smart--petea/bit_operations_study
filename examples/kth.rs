extern crate bit_manipulation as bm;

fn main() {
    let x = bm::Byte::new("01000110").unwrap();
    let y = x | (3 << bm::Byte::one());

    assert_eq!(y, bm::Byte::new("01001110").unwrap());
}
