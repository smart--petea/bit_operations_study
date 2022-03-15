extern crate bit_manipulation as bm;

//toggle the kth bit

fn main() {
    let x = bm::Byte::new("01000110").unwrap();

    let toggle_1bit = x.clone() ^ (2 << bm::Byte::one());
    assert_eq!(toggle_1bit, bm::Byte::new("01000010").unwrap());

    let toggle_0bit = x ^ (4 << bm::Byte::one());
    assert_eq!(toggle_0bit, bm::Byte::new("01010110").unwrap());
}
