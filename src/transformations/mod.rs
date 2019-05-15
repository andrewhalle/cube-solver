use std::collections::HashMap;

use ndarray::Array2;

pub fn cube2() -> HashMap<String, Array2<u8>> {
    let mut map = HashMap::new();

    let mut u = Array2::<u8>::zeros((24, 24));
    let row_order: Vec<u32> = vec![
        2, 0, 3, 1, 8, 9, 6, 7, 12, 13, 10, 11, 16, 17, 14, 15, 4, 5, 18, 19, 20, 21, 22, 23,
    ];
    for (i, row) in row_order.iter().enumerate() {
        u[[i, *row as usize]] = 1;
    }
    let u2 = u.dot(&u);
    let u_prime = u.dot(&u).dot(&u);

    map.insert(String::from("U"), u);
    map.insert(String::from("U2"), u2);
    map.insert(String::from("U'"), u_prime);

    let mut d = Array2::<u8>::zeros((24, 24));
    let row_order: Vec<u32> = vec![
        0, 1, 2, 3, 4, 5, 18, 19, 8, 9, 6, 7, 12, 13, 10, 11, 16, 17, 14, 15, 22, 20, 23, 21,
    ];
    for (i, row) in row_order.iter().enumerate() {
        d[[i, *row as usize]] = 1;
    }
    let d2 = d.dot(&d);
    let d_prime = d.dot(&d).dot(&d);

    map.insert(String::from("D"), d);
    map.insert(String::from("D2"), d2);
    map.insert(String::from("D'"), d_prime);

    let mut f = Array2::<u8>::zeros((24, 24));
    let row_order: Vec<u32> = vec![
        0, 1, 7, 5, 4, 20, 6, 21, 10, 8, 11, 9, 2, 13, 3, 15, 16, 17, 18, 19, 14, 12, 22, 23,
    ];
    for (i, row) in row_order.iter().enumerate() {
        f[[i, *row as usize]] = 1;
    }
    let f2 = f.dot(&f);
    let f_prime = f.dot(&f).dot(&f);

    map.insert(String::from("F"), f);
    map.insert(String::from("F2"), f2);
    map.insert(String::from("F'"), f_prime);

    let mut b = Array2::<u8>::zeros((24, 24));
    let row_order: Vec<u32> = vec![
        13, 15, 2, 3, 1, 5, 0, 7, 8, 9, 10, 11, 12, 23, 14, 22, 18, 16, 19, 17, 20, 21, 4, 6,
    ];
    for (i, row) in row_order.iter().enumerate() {
        b[[i, *row as usize]] = 1;
    }
    let b2 = b.dot(&b);
    let b_prime = b.dot(&b).dot(&b);

    map.insert(String::from("B"), b);
    map.insert(String::from("B2"), b2);
    map.insert(String::from("B'"), b_prime);

    let mut l = Array2::<u8>::zeros((24, 24));
    let row_order: Vec<u32> = vec![
        19, 1, 17, 3, 6, 4, 7, 5, 0, 9, 2, 11, 12, 13, 14, 15, 16, 22, 18, 20, 8, 21, 10, 23,
    ];
    for (i, row) in row_order.iter().enumerate() {
        l[[i, *row as usize]] = 1;
    }
    let l2 = l.dot(&l);
    let l_prime = l.dot(&l).dot(&l);

    map.insert(String::from("L"), l);
    map.insert(String::from("L2"), l2);
    map.insert(String::from("L'"), l_prime);

    let mut r = Array2::<u8>::zeros((24, 24));
    let row_order: Vec<u32> = vec![
        0, 9, 2, 11, 4, 5, 6, 7, 8, 21, 10, 23, 14, 12, 15, 13, 3, 17, 1, 19, 20, 18, 22, 16,
    ];
    for (i, row) in row_order.iter().enumerate() {
        r[[i, *row as usize]] = 1;
    }
    let r2 = r.dot(&r);
    let r_prime = r.dot(&r).dot(&r);

    map.insert(String::from("R"), r);
    map.insert(String::from("R2"), r2);
    map.insert(String::from("R'"), r_prime);

    map
}
