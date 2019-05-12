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

    map
}
