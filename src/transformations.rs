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

pub fn cube3() -> HashMap<String, Array2<u8>> {
    // fundamental moves
    let mut u = Array2::<u8>::zeros((54, 54));
    let row_order: Vec<u32> = vec![
        6, 3, 0, 7, 4, 1, 8, 5, 2, 18, 19, 20, 12, 13, 14, 15, 16, 17, 27, 28, 29, 21, 22, 23, 24,
        25, 26, 36, 37, 38, 30, 31, 32, 33, 34, 35, 9, 10, 11, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53,
    ];
    for (i, row) in row_order.iter().enumerate() {
        u[[i, *row as usize]] = 1;
    }
    let u2 = u.dot(&u);
    let u_prime = u2.dot(&u);

    let mut x = Array2::<u8>::zeros((54, 54));
    let row_order: Vec<u32> = vec![
        18, 19, 20, 21, 22, 23, 24, 25, 26, 11, 14, 17, 10, 13, 16, 9, 12, 15, 45, 46, 47, 48, 49,
        50, 51, 52, 53, 33, 30, 27, 34, 31, 28, 35, 32, 29, 8, 7, 6, 5, 4, 3, 2, 1, 0, 44, 43, 42,
        41, 40, 39, 38, 37, 36,
    ];
    for (i, row) in row_order.iter().enumerate() {
        x[[i, *row as usize]] = 1;
    }
    let x2 = x.dot(&x);
    let x_prime = x2.dot(&x);

    let mut z = Array2::<u8>::zeros((54, 54));
    let row_order: Vec<u32> = vec![
        15, 12, 9, 16, 13, 10, 17, 14, 11, 51, 48, 45, 52, 49, 46, 53, 50, 47, 24, 21, 18, 25, 22,
        19, 26, 23, 20, 6, 3, 0, 7, 4, 1, 8, 5, 2, 38, 41, 44, 37, 40, 43, 36, 39, 42, 33, 30, 27,
        34, 31, 28, 35, 32, 29,
    ];
    for (i, row) in row_order.iter().enumerate() {
        z[[i, *row as usize]] = 1;
    }
    let z2 = z.dot(&z);
    let z_prime = z2.dot(&z);

    let mut map = HashMap::new();
    map.insert(String::from("R"), z_prime.dot(&u).dot(&z));
    map.insert(String::from("R2"), z_prime.dot(&u2).dot(&z));
    map.insert(String::from("R'"), z_prime.dot(&u_prime).dot(&z));

    map.insert(String::from("L"), z.dot(&u).dot(&z_prime));
    map.insert(String::from("L2"), z.dot(&u2).dot(&z_prime));
    map.insert(String::from("L'"), z.dot(&u_prime).dot(&z_prime));

    map.insert(String::from("F"), x.dot(&u).dot(&x_prime));
    map.insert(String::from("F2"), x.dot(&u2).dot(&x_prime));
    map.insert(String::from("F'"), x.dot(&u_prime).dot(&x_prime));

    map.insert(String::from("B"), x_prime.dot(&u).dot(&x));
    map.insert(String::from("B2"), x_prime.dot(&u2).dot(&x));
    map.insert(String::from("B'"), x_prime.dot(&u_prime).dot(&x));

    map.insert(String::from("D"), x2.dot(&u).dot(&x2));
    map.insert(String::from("D2"), x2.dot(&u2).dot(&x2));
    map.insert(String::from("D'"), x2.dot(&u_prime).dot(&x2));

    map.insert(String::from("U"), u);
    map.insert(String::from("U2"), u2);
    map.insert(String::from("U'"), u_prime);

    map
}
