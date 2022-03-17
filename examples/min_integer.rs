extern crate bit_manipulation as bm;

//toggle the kth bit
//min = y ^ ((x^y) & !(x<y))

fn main() {
    let a = bm::Byte::new(5u8).unwrap();
    let b = bm::Byte::new(7u8).unwrap();
    let c = bit_min(&a, &b).unwrap();
    assert_eq!(Into::<u8>::into(c), 5u8);


    /*
     * it works only u8 and positive numbers
     * let a = bm::Byte::new(5i8).unwrap();
     * let b = bm::Byte::new(-7i8).unwrap();
     * let c = bit_min(&a, &b).unwrap();
     * assert_eq!(Into::<i8>::into(c), -7i8);
     */
}

fn bit_min(x: &bm::Byte, y: &bm::Byte) -> Result<bm::Byte, String> {
    let z: bm::Byte = x^y;
    let w: bm::Byte = From::<bool>::from(x<y);
    let w: bm::Byte = !w;
    let result: bm::Byte = y^&(z & w);

    Ok(result)
}
