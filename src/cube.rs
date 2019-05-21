use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use ndarray::{Array1, Array2};

#[derive(Clone)]
pub struct Cube<'a> {
    transformations: &'a HashMap<String, Array2<u8>>,
    data: Array1<u8>,
}

impl<'a> Cube<'a> {
    pub fn new(dim: u32, transformations: &'a HashMap<String, Array2<u8>>) -> Self {
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

    pub fn is_solved(&self) -> bool {
        for face in self.data.to_vec().chunks(4) {
            if face[0] != face[1] || face[0] != face[2] || face[0] != face[3] {
                return false;
            }
        }

        true
    }

    pub fn state_string(&self) -> String {
        let mut color_table: HashMap<u8, u8> = HashMap::new();
        let mut counter = 0;
        base64::encode(
            &self
                .data
                .to_vec()
                .into_iter()
                .map(|x| {
                    if color_table.contains_key(&x) {
                        *color_table.get(&x).unwrap()
                    } else {
                        color_table.insert(x, counter);
                        counter += 1;
                        *color_table.get(&x).unwrap()
                    }
                })
                .collect::<Vec<u8>>()[..],
        )
    }
}

impl<'a> Hash for Cube<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<'a> PartialEq for Cube<'a> {
    fn eq(&self, other: &Cube) -> bool {
        self.data == other.data
    }
}

impl<'a> Eq for Cube<'a> {}

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

    #[test]
    fn test_cube3() {
        let t = transformations::cube3();
        let mut c = Cube::new(3, &t);
        //c.twist("U F' R2 U2 R B' R2 B R U L2 R2 F' L R2 F L' R F' B2 R B L' R' B");
        c.twist("B");
        println!("{:?}", c.data);
    }
}
