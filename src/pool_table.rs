use ndarray::{array, Array1};

pub const WIDTH: f64 = 150.0; // cm
pub const LENGTH: f64 = 980.665; // cm

pub struct PoolTable {
    pub pockets: Vec<Pocket>,
    pub cushions: Vec<Cushion>,
}

impl PoolTable {
    pub fn new() -> Self {
        PoolTable {
            pockets: init_pockets(),
            cushions: init_cushions(),
        }
    }

    pub fn get_pocket(&self, pocket_id: usize) -> &Pocket {
        &self.pockets[pocket_id]
    }

    pub fn num_pockets(&self) -> usize {
        self.pockets.len()
    }

    pub fn num_cushions(&self) -> usize {
        self.cushions.len()
    }
}

pub struct Cushion {
    pub r1: Array1<f64>,
    pub r2: Array1<f64>,
}

pub struct Pocket {
    pub r: Array1<f64>,
}

fn init_pockets() -> Vec<Pocket> {
    vec![
        Pocket {
            r: array![0.0, 0.0],
        },
        Pocket {
            r: array![0.0, LENGTH / 2.0],
        },
        Pocket {
            r: array![0.0, LENGTH],
        },
        Pocket {
            r: array![WIDTH, 0.0],
        },
        Pocket {
            r: array![WIDTH, LENGTH / 2.0],
        },
        Pocket {
            r: array![WIDTH, LENGTH],
        },
    ]
}

fn init_cushions() -> Vec<Cushion> {
    vec![
        Cushion {
            r1: array![0.0, 0.0],
            r2: array![WIDTH, 0.0],
        },
        Cushion {
            r1: array![0.0, 0.0],
            r2: array![0.0, LENGTH],
        },
        Cushion {
            r1: array![0.0, LENGTH],
            r2: array![WIDTH, LENGTH],
        },
        Cushion {
            r1: array![WIDTH, 0.0],
            r2: array![WIDTH, LENGTH],
        },
    ]
}
