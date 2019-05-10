use lazy_static::lazy_static;
use ndarray::prelude::*;
// use nalgebra;

lazy_static! {
    // put transformation matricies here
    // read from file, store transformation matricies as bytes, create iterator on bits
    // first two bytes give matrix dimension, remaining bytes are bits representing the
    // 1's and 0's in row major order
}

// XXX need tables for heuristics, use similar serialization as transformation matricies
// function to generate them

// implement cube as column vector, turns are matrix transformations
pub struct Cube;

pub struct Move {
    face: Face,
    turn: Turn,
}

pub enum Face {
    U,
    D,
    F,
    B,
    L,
    R,
}

pub enum Turn {
    Clockwise,
    CounterClockwise,
    Double,
}

pub enum Color {
    Blue,
    Green,
    Red,
    Orange,
    White,
    Yellow,
}

impl Cube {
    pub fn new() -> Cube {
        Cube
    }

    pub fn apply(&mut self, moves: Vec<Move>) {}

    pub fn scramble(&mut self) {}

    pub fn solve(&self) -> Vec<Move> {
        // XXX IDA*
        vec![]
    }
}
