use ndarray::{array, Array1};


pub struct Cushion {
    pub id: usize,
    pub r1: Array1<f64>,
    pub r2: Array1<f64>,
}

pub struct Pocket {
    pub id: usize,
    pub r: Array1<f64>,
}

pub fn init_pockets() -> Vec<Pocket> {
    Vec::new()
}

pub fn init_cushions() -> Vec<Cushion> {
    Vec::new()
}