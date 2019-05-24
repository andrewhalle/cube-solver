use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use ndarray::{Array1, Array2};

#[derive(Debug)]
enum Corner {
    BlueOrangeWhite,
    BlueOrangeYellow,
    BlueRedWhite,
    BlueRedYellow,
    GreenOrangeWhite,
    GreenOrangeYellow,
    GreenRedWhite,
    GreenRedYellow,
}

impl Corner {
    fn from(v: [u8; 3]) -> Option<Corner> {
        match v {
            [0, 1, 4] => Some(Corner::BlueOrangeWhite),
            [0, 1, 2] => Some(Corner::BlueRedWhite),
            [0, 2, 3] => Some(Corner::BlueRedYellow),
            [0, 3, 4] => Some(Corner::BlueOrangeYellow),
            [1, 4, 5] => Some(Corner::GreenOrangeWhite),
            [1, 2, 5] => Some(Corner::GreenRedWhite),
            [2, 3, 5] => Some(Corner::GreenRedYellow),
            [3, 4, 5] => Some(Corner::GreenOrangeYellow),
            _ => None,
        }
    }
}

// which slice contains blue/green, up/down, right/left, or front/back
#[derive(Debug)]
enum CornerOrientation {
    Up,
    Right,
    Front,
}

impl CornerOrientation {
    fn from(v: [u8; 3]) -> Option<CornerOrientation> {
        match v {
            [0, _, _] | [5, _, _] => Some(CornerOrientation::Up),
            [_, 0, _] | [_, 5, _] => Some(CornerOrientation::Right),
            [_, _, 0] | [_, _, 5] => Some(CornerOrientation::Front),
            _ => None,
        }
    }
}

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

    pub fn corners(transformations: &'a HashMap<String, Array2<u8>>) -> Self {
        let mut c = Cube::new(3, transformations);
        let indices = vec![
            1, 3, 4, 5, 7, 10, 12, 13, 14, 16, 19, 21, 22, 23, 25, 28, 30, 31, 32, 34, 37, 39, 40,
            41, 43, 46, 48, 49, 50, 52,
        ];
        for i in indices.into_iter() {
            c.data[[i]] = 255;
        }

        c
    }

    pub fn edges1(transformations: &'a HashMap<String, Array2<u8>>) -> Self {
        let mut c = Cube::new(3, transformations);
        let indices = vec![
            0, 2, 4, 6, 8, 9, 11, 12, 13, 15, 16, 17, 18, 20, 22, 24, 25, 26, 27, 29, 31, 32, 33,
            34, 35, 36, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
        ];
        for i in indices.into_iter() {
            c.data[[i]] = 255;
        }

        c
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

    fn corners_data(&self) -> (Vec<Corner>, Vec<CornerOrientation>) {
        let mut perm = Vec::new();
        let mut orient = Vec::new();

        let mut c = vec![self.data[0], self.data[9], self.data[38]];
        orient.push(CornerOrientation::from([c[0], c[1], c[2]]).unwrap());
        c.sort();
        perm.push(Corner::from([c[0], c[1], c[2]]).unwrap());

        c = vec![self.data[2], self.data[29], self.data[36]];
        orient.push(CornerOrientation::from([c[0], c[1], c[2]]).unwrap());
        c.sort();
        perm.push(Corner::from([c[0], c[1], c[2]]).unwrap());

        c = vec![self.data[6], self.data[11], self.data[18]];
        orient.push(CornerOrientation::from([c[0], c[1], c[2]]).unwrap());
        c.sort();
        perm.push(Corner::from([c[0], c[1], c[2]]).unwrap());

        c = vec![self.data[8], self.data[27], self.data[20]];
        orient.push(CornerOrientation::from([c[0], c[1], c[2]]).unwrap());
        c.sort();
        perm.push(Corner::from([c[0], c[1], c[2]]).unwrap());

        c = vec![self.data[51], self.data[15], self.data[44]];
        orient.push(CornerOrientation::from([c[0], c[1], c[2]]).unwrap());
        c.sort();
        perm.push(Corner::from([c[0], c[1], c[2]]).unwrap());

        c = vec![self.data[53], self.data[35], self.data[42]];
        orient.push(CornerOrientation::from([c[0], c[1], c[2]]).unwrap());
        c.sort();
        perm.push(Corner::from([c[0], c[1], c[2]]).unwrap());

        c = vec![self.data[45], self.data[17], self.data[24]];
        orient.push(CornerOrientation::from([c[0], c[1], c[2]]).unwrap());
        c.sort();
        perm.push(Corner::from([c[0], c[1], c[2]]).unwrap());

        c = vec![self.data[47], self.data[33], self.data[26]];
        orient.push(CornerOrientation::from([c[0], c[1], c[2]]).unwrap());
        c.sort();
        perm.push(Corner::from([c[0], c[1], c[2]]).unwrap());

        (perm, orient)
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

pub fn corners_state(c: &Cube) -> String {
    let mut color_table: HashMap<u8, u8> = HashMap::new();
    let mut counter = 0;

    let mut data_to_encode: Vec<u8> = Vec::new();
    for val in c.data.to_vec().into_iter() {
        if val != 255 {
            data_to_encode.push(val);
        }
    }

    base64::encode(
        &data_to_encode
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

pub fn edges_state(c: &Cube) -> String {
    // XXX fix this to read off all edges, cycle so first edge isn't ignored, then encode
    let mut color_table: HashMap<u8, u8> = HashMap::new();
    let mut counter = 0;

    let mut data_to_encode: Vec<u8> = Vec::new();
    for val in c.data.to_vec().into_iter() {
        if val != 255 {
            data_to_encode.push(val);
        }
    }

    base64::encode(
        &data_to_encode
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

pub fn corners_index(c: &Cube) -> usize {
    corners_perm_index(c) * corners_orient_index(c)
}

fn corners_perm_index(c: &Cube) -> usize {
    let v = c.corners_data();

    0 // XXX
}

fn corners_orient_index(c: &Cube) -> usize {
    let v = c.corners_data();

    0 // XXX
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

    #[test]
    fn test_cube3() {
        let t = transformations::cube3();
        let mut c = Cube::new(3, &t);
        c.twist("U F' R2 U2 R B' R2 B R U L2 R2 F' L R2 F L' R F' B2 R B L' R' B");
        println!("{:?}", c.data);
    }

    #[test]
    fn test_corner_data() {
        let t = transformations::cube3();
        let mut c = Cube::new(3, &t);
        c.twist("U F' R2 U2 R B' R2 B R U L2 R2 F' L R2 F L' R F' B2 R B L' R' B");
        let data = c.corners_data();
        println!("{:?}, {:?}", data.0, data.1);
    }
}
