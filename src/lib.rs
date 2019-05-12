use std::collections::HashMap;

use ndarray::{Array1, Array2};

mod search;
mod transformations;

pub struct Cube {
    transformations: HashMap<String, Array2<u8>>,
    data: Array1<u8>,
}

impl Cube {
    pub fn new(dim: u32) -> Self {
        if dim != 2 {
            unimplemented!()
        }

        let mut v = Vec::new();
        for color in 0..6 {
            for _i in 0..4 {
                v.push(color);
            }
        }

        Cube {
            transformations: transformations::cube2(),
            data: Array1::from(v),
        }
    }

    pub fn twist(&mut self, moves: &str) {
        let moves = String::from(moves);

        for mv in moves.split_whitespace() {
            self.data = self.transformations[mv].dot(&self.data);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Cube;

    #[test]
    fn it_works() {
        let mut c = Cube::new(2);
        c.twist("D");
        println!("{:?}", c.data);
    }
}
