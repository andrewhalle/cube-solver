use std::collections::HashMap;

use ndarray::{Array1, Array2};

pub struct Cube<'a> {
    transformations: &'a HashMap<String, Array2<u8>>,
    data: Array1<u8>,
}

impl<'a> Cube<'a> {
    pub fn new(dim: u32, transformations: &'a HashMap<String, Array2<u8>>) -> Self {
        if dim != 2 {
            unimplemented!()
        }

        let mut v = Vec::new();
        for color in 0..6 {
            for _i in 0..dim.pow(2) {
                v.push(color);
            }
        }

        Cube {
            transformations,
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
    use crate::cube::Cube;
    use crate::transformations;

    #[test]
    fn it_works() {
        let t = transformations::cube2();
        let mut c = Cube::new(2, &t);
        c.twist("B U2 B R2 D F2 B' U' L2");
        println!("{:?}", c.data);
    }
}
