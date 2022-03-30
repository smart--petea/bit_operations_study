extern crate bit_manipulation as bm;

//round up to 2 power
//input: n
//output: the smallest 2^x >= n
//
//
//for example
//input: 5
//output: 8

fn main() {
}

fn round_up(mut n:  u8) -> u8 {
    if n == 0 {
        return 0;
    }

    let n_bit = bm::Byte::new(n).unwrap();
    let bit_one :bm::Byte = bm::Byte::new(1u8).unwrap();

    Into::<u8>::into(n_bit + bit_one)
}
