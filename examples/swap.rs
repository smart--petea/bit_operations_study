extern crate bit_manipulation as bm;

//toggle the kth bit

fn main() {
    let x = bm::Byte::new("01001100").unwrap();
    let y = bm::Byte::new("00011100").unwrap();

    let x = x ^ y.clone();
    let y = x.clone() ^ y;
    let x = x ^ y.clone();

    assert_eq!(x, bm::Byte::new("00011100").unwrap());
    assert_eq!(y, bm::Byte::new("01001100").unwrap());
}
