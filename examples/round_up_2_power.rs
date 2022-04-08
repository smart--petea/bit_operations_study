extern crate bit_manipulation as bm;

use bm::byte::Byte;

//round up to 2 power
//input: n
//output: the smallest 2^x >= n
//
//
//for example
//input: 5
//output: 8

fn main() {
    assert_eq!(round_up(0), 0);

    assert_eq!(round_up(1), 1);

    assert_eq!(round_up(2), 2);

    assert_eq!(round_up(3), 4);
    assert_eq!(round_up(4), 4);

    assert_eq!(round_up(5), 8);
    assert_eq!(round_up(8), 8);

    assert_eq!(round_up(9), 16);
    assert_eq!(round_up(16), 16);
}

fn round_up(n: u8) -> u8 {
    if n == 0 {
        return 0;
    }

    let n_bit: Byte = Byte::new(n).unwrap();
    let bit_one: Byte = Byte::new(1u8).unwrap();

    let mut temp: Byte = n_bit - bit_one.clone();
    temp |= temp >> 1;
    temp |= temp >> 2;
    temp |= temp >> 4;
    temp |= temp >> 8;

    temp += bit_one;

    Into::<u8>::into(temp)
}
