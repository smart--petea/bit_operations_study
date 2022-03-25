extern crate bit_manipulation as bm;

// modulat addition
// 0 <= x < n
// 0 <= y < n
// 
//    r =  (x + y) % n
// OR
//    z = x + y
//    r = z - (n & !(z >= n))

fn main() {
    //1. x + y = n
    let n = bm::Byte::new(5u8).unwrap();
    let x = bm::Byte::new(3u8).unwrap();
    let y = bm::Byte::new(2u8).unwrap();

    let z = x + y;
    let w: bm::Byte = From::<bool>::from(z >= n);
    let r = Into::<u8>::into(z.clone()) - Into::<u8>::into(n.clone() & w.clone());
    assert_eq!(r, 0);

    //2. x + y < n
    let n = bm::Byte::new(5u8).unwrap();
    let x = bm::Byte::new(2u8).unwrap();
    let y = bm::Byte::new(2u8).unwrap();

    let z = x + y;
    let w: bm::Byte = From::<bool>::from(z >= n);
    let r = Into::<u8>::into(z.clone()) - Into::<u8>::into(n.clone() & w.clone());
    assert_eq!(r, 4);


    //3. x + y > n
    let n = bm::Byte::new(5u8).unwrap();
    let x = bm::Byte::new(4u8).unwrap();
    let y = bm::Byte::new(2u8).unwrap();

    let z = x + y;
    let w: bm::Byte = From::<bool>::from(z >= n);
    let r = Into::<u8>::into(z.clone()) - Into::<u8>::into(n.clone() & w.clone());
    assert_eq!(r, 1);
}
