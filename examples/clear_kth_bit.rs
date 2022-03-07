extern crate bit_manipulation as bm;

//clear kth bit

fn main() {
    let x = bm::Byte::new("01000110").unwrap();
    let y = x & !(2 << bm::Byte::one());

    assert_eq!(y, bm::Byte::new("01000010").unwrap());
}
