use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::Path;

use lazy_static::lazy_static;
use ndarray::{Array1, Array2};

use crate::search::IDAStarNode;

lazy_static! {
    static ref CORNER_PERMUTATIONS: HashMap<String, usize> = all_corner_permutations();
    static ref EDGES1_PERMUTATIONS: HashMap<String, usize> = all_edges1_permutations();
    static ref EDGES2_PERMUTATIONS: HashMap<String, usize> = all_edges2_permutations();
    static ref EDGES1_MASK: HashSet<Edge> = edges1_mask();
    static ref EDGES2_MASK: HashSet<Edge> = edges2_mask();
}

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
    // fix this with unreachable!
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

    fn index(&self) -> usize {
        match self {
            Corner::BlueOrangeWhite => 0,
            Corner::BlueOrangeYellow => 1,
            Corner::BlueRedWhite => 2,
            Corner::BlueRedYellow => 3,
            Corner::GreenOrangeWhite => 4,
            Corner::GreenOrangeYellow => 5,
            Corner::GreenRedWhite => 6,
            Corner::GreenRedYellow => 7,
        }
    }
}

fn all_corner_permutations() -> HashMap<String, usize> {
    let path = Path::new("tables/corner_permutations.json");
    let file = File::open(&path).unwrap();

    let perms: HashMap<String, usize> = serde_json::from_reader(&file).unwrap();

    perms
}

fn all_edges1_permutations() -> HashMap<String, usize> {
    let path = Path::new("tables/edges1_permutations.json");
    let file = File::open(&path).unwrap();

    let perms: HashMap<String, usize> = serde_json::from_reader(&file).unwrap();

    perms
}

fn all_edges2_permutations() -> HashMap<String, usize> {
    let path = Path::new("tables/edges2_permutations.json");
    let file = File::open(&path).unwrap();

    let perms: HashMap<String, usize> = serde_json::from_reader(&file).unwrap();

    perms
}

fn edges1_mask() -> HashSet<Edge> {
    let mut mask = HashSet::new();

    mask.insert(Edge::RedWhite);
    mask.insert(Edge::RedYellow);
    mask.insert(Edge::GreenOrange);
    mask.insert(Edge::GreenWhite);
    mask.insert(Edge::GreenYellow);
    mask.insert(Edge::GreenRed);

    mask
}

fn edges2_mask() -> HashSet<Edge> {
    let mut mask = HashSet::new();

    mask.insert(Edge::BlueOrange);
    mask.insert(Edge::BlueWhite);
    mask.insert(Edge::BlueYellow);
    mask.insert(Edge::BlueRed);
    mask.insert(Edge::OrangeWhite);
    mask.insert(Edge::OrangeYellow);

    mask
}

// which slice contains blue/green, up/down, right/left, or front/back
#[derive(Debug)]
enum CornerOrientation {
    Up,
    Right,
    Front,
}

impl CornerOrientation {
    // fix this with unreachable!
    fn from(v: [u8; 3]) -> Option<CornerOrientation> {
        match v {
            [0, _, _] | [5, _, _] => Some(CornerOrientation::Up),
            [_, 0, _] | [_, 5, _] => Some(CornerOrientation::Right),
            [_, _, 0] | [_, _, 5] => Some(CornerOrientation::Front),
            _ => None,
        }
    }

    fn index(&self) -> usize {
        match self {
            CornerOrientation::Up => 0,
            CornerOrientation::Right => 1,
            CornerOrientation::Front => 2,
        }
    }
}

enum EdgeSet {
    Edges1,
    Edges2,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Edge {
    BlueOrange,
    BlueWhite,
    BlueYellow,
    BlueRed,
    OrangeWhite,
    OrangeYellow,
    RedWhite,
    RedYellow,
    GreenOrange,
    GreenWhite,
    GreenYellow,
    GreenRed,
    Ignored, // special type for ignoring pieces for generating tables
}

impl Edge {
    // clean this up with a macro
    fn from(v: [u8; 2], mask: &HashSet<Edge>) -> Edge {
        match v {
            [0, 4] => {
                if mask.contains(&Edge::BlueOrange) {
                    Edge::Ignored
                } else {
                    Edge::BlueOrange
                }
            }
            [0, 1] => {
                if mask.contains(&Edge::BlueWhite) {
                    Edge::Ignored
                } else {
                    Edge::BlueWhite
                }
            }
            [0, 3] => {
                if mask.contains(&Edge::BlueYellow) {
                    Edge::Ignored
                } else {
                    Edge::BlueYellow
                }
            }
            [0, 2] => {
                if mask.contains(&Edge::BlueRed) {
                    Edge::Ignored
                } else {
                    Edge::BlueRed
                }
            }
            [1, 4] => {
                if mask.contains(&Edge::OrangeWhite) {
                    Edge::Ignored
                } else {
                    Edge::OrangeWhite
                }
            }
            [3, 4] => {
                if mask.contains(&Edge::OrangeYellow) {
                    Edge::Ignored
                } else {
                    Edge::OrangeYellow
                }
            }
            [1, 2] => {
                if mask.contains(&Edge::RedWhite) {
                    Edge::Ignored
                } else {
                    Edge::RedWhite
                }
            }
            [2, 3] => {
                if mask.contains(&Edge::RedYellow) {
                    Edge::Ignored
                } else {
                    Edge::RedYellow
                }
            }
            [4, 5] => {
                if mask.contains(&Edge::GreenOrange) {
                    Edge::Ignored
                } else {
                    Edge::GreenOrange
                }
            }
            [1, 5] => {
                if mask.contains(&Edge::GreenWhite) {
                    Edge::Ignored
                } else {
                    Edge::GreenWhite
                }
            }
            [3, 5] => {
                if mask.contains(&Edge::GreenYellow) {
                    Edge::Ignored
                } else {
                    Edge::GreenYellow
                }
            }
            [2, 5] => {
                if mask.contains(&Edge::GreenRed) {
                    Edge::Ignored
                } else {
                    Edge::GreenRed
                }
            }
            _ => unreachable!(),
        }
    }

    fn index(&self) -> usize {
        match self {
            Edge::BlueOrange => 0,
            Edge::BlueWhite => 1,
            Edge::BlueYellow => 2,
            Edge::BlueRed => 3,
            Edge::OrangeWhite => 4,
            Edge::OrangeYellow => 5,
            Edge::RedWhite => 6,
            Edge::RedYellow => 7,
            Edge::GreenOrange => 8,
            Edge::GreenWhite => 9,
            Edge::GreenYellow => 10,
            Edge::GreenRed => 11,
            Edge::Ignored => 12,
        }
    }
}

#[derive(Debug)]
enum EdgeOrientation {
    Good,
    Bad,
}

impl EdgeOrientation {
    fn from(v: [u8; 2]) -> EdgeOrientation {
        match v {
            [0, 4]
            | [0, 1]
            | [0, 3]
            | [0, 2]
            | [4, 1]
            | [4, 3]
            | [2, 1]
            | [2, 3]
            | [5, 4]
            | [5, 1]
            | [5, 3]
            | [5, 2] => EdgeOrientation::Good,
            [4, 0]
            | [1, 0]
            | [3, 0]
            | [2, 0]
            | [1, 4]
            | [3, 4]
            | [1, 2]
            | [3, 2]
            | [4, 5]
            | [1, 5]
            | [3, 5]
            | [2, 5] => EdgeOrientation::Bad,
            _ => unreachable!(),
        }
    }

    fn index(&self) -> usize {
        match self {
            EdgeOrientation::Good => 0,
            EdgeOrientation::Bad => 1,
        }
    }
}

#[derive(Clone, Debug)]
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
        for face in self.data.to_vec().chunks(9) {
            let color = face[0];
            for c in face.iter() {
                if c != &color {
                    return false;
                }
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

    // XXX clean this up with a macro
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

    // XXX clean this up with a macro
    fn edges_data(&self, mask: &HashSet<Edge>) -> (Vec<Edge>, Vec<EdgeOrientation>) {
        let mut perm = Vec::new();
        let mut orient = Vec::new();

        let mut c = vec![self.data[1], self.data[37]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[3], self.data[10]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[5], self.data[28]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[7], self.data[19]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[41], self.data[12]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[39], self.data[32]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[21], self.data[14]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[23], self.data[30]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[52], self.data[43]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[48], self.data[16]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[50], self.data[34]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        let mut c = vec![self.data[46], self.data[25]];
        orient.push(EdgeOrientation::from([c[0], c[1]]));
        c.sort();
        perm.push(Edge::from([c[0], c[1]], &mask));

        (perm, orient)
    }

    pub fn successors(&self) -> Vec<IDAStarNode<'a>> {
        let mut result = Vec::new();
        let all_moves = vec![
            "U", "U'", "U2", "F", "F'", "F2", "R", "R'", "R2", "D", "D'", "D2", "B", "B'", "B2",
            "L", "L'", "L2",
        ];

        for m in all_moves.into_iter() {
            let mut c = self.clone();
            c.twist(m);

            result.push(IDAStarNode {
                state: c,
                mv_to_get_here: m.to_string(),
            });
        }

        result
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

fn corner_perm_as_string(perm: &[Corner]) -> String {
    perm.iter()
        .map(|x| x.index().to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn edges_perm_as_string(perm: &[Edge]) -> String {
    perm.iter()
        .map(|x| x.index().to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn corners_index(c: &Cube) -> usize {
    let (perm, orient) = c.corners_data();

    let perm_index = CORNER_PERMUTATIONS
        .get(&corner_perm_as_string(&perm))
        .unwrap();

    let orient_index = {
        let mut result = 0 as usize;

        // intentionally skip one, last orientation is governed by other 7
        for i in 0..7 {
            result += orient[i].index() * (3 as usize).pow(i as u32);
        }

        result
    };

    (perm_index * 2187) + orient_index
}

fn edges_index(c: &Cube, set: EdgeSet) -> usize {
    let (perm, orient) = match &set {
        EdgeSet::Edges1 => c.edges_data(&EDGES1_MASK),
        EdgeSet::Edges2 => c.edges_data(&EDGES2_MASK),
    };

    let perm_index = match &set {
        EdgeSet::Edges1 => EDGES1_PERMUTATIONS
            .get(&edges_perm_as_string(&perm))
            .unwrap(),
        EdgeSet::Edges2 => EDGES2_PERMUTATIONS
            .get(&edges_perm_as_string(&perm))
            .unwrap(),
    };

    let orient_index = {
        let mut result = 0 as usize;

        let mut power = 0;
        // only consider orientations of non-ignored
        for (i, o) in orient.iter().enumerate() {
            if perm[i] != Edge::Ignored {
                result += o.index() * (2 as usize).pow(power);
                power += 1;
            }
        }

        result
    };

    (perm_index * 64) + orient_index
}

pub fn edges1_index(c: &Cube) -> usize {
    edges_index(c, EdgeSet::Edges1)
}

pub fn edges2_index(c: &Cube) -> usize {
    edges_index(c, EdgeSet::Edges2)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::cube::{Cube, Edge};
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
    fn test_corners_data() {
        let t = transformations::cube3();
        let mut c = Cube::new(3, &t);
        //c.twist("U F' R2 U2 R B' R2 B R U L2 R2 F' L R2 F L' R F' B2 R B L' R' B");
        //c.twist("U L");
        let data = c.corners_data();
        println!("{:?}, {:?}", data.0, data.1);
        println!("{}", crate::cube::corners_index(&c));
    }

    #[test]
    fn test_edges_data() {
        let t = transformations::cube3();
        let mut c = Cube::new(3, &t);
        c.twist("B' R D2 U2 R' L U D' F2 D' F2 L F2 L2 B U2 R' F' L' B2 D' F L2");

        println!("{}", crate::cube::edges2_index(&c));
    }

    #[test]
    fn test_is_solved() {
        let t = transformations::cube3();
        let mut c = Cube::new(3, &t);
        c.twist("U U'");
        println!("cube is solved: {}", c.is_solved());
    }
}
