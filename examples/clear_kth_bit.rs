extern crate bit_manipulation as bm;

use bm::byte::Byte;

//clear kth bit

fn main() {
    let x = Byte::new("01000110").unwrap();
    let y = x & !(2 << Byte::all_ones());

    assert_eq!(y, Byte::new("01000010").unwrap());
}
