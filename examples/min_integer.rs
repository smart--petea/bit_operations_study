extern crate bit_manipulation as bm;

use bm::byte::Byte;

//toggle the kth bit
//min = y ^ ((x^y) & !(x<y))

fn main() {
    let a = Byte::new(5u8).unwrap();
    let b = Byte::new(7u8).unwrap();
    let c = bit_min(&a, &b).unwrap();
    assert_eq!(Into::<u8>::into(c), 5u8);


    /*
     * it works only u8 and positive numbers
     * let a = Byte::new(5i8).unwrap();
     * let b = Byte::new(-7i8).unwrap();
     * let c = bit_min(&a, &b).unwrap();
     * assert_eq!(Into::<i8>::into(c), -7i8);
     */
}

fn bit_min(x: &Byte, y: &Byte) -> Result<Byte, String> {
    let z: Byte = x^y;
    let w: Byte = From::<bool>::from(x<y);
    let w: Byte = !w;
    let result: Byte = y^&(z & !w);

    Ok(result)
}
