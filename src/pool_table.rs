use ndarray::{array, Array1};

pub const WIDTH: f64 = 150.0; // cm
pub const LENGTH: f64 = 980.665; // cm

#[derive(Debug, PartialEq)]
pub enum CushionType {
    TOP,
    LEFT,
    RIGHT,
    BOTTOM,
}

pub struct Cushion {
    pub r1: Array1<f64>,
    pub r2: Array1<f64>,
    pub ctype: CushionType,
}

pub struct Pocket {
    pub r: Array1<f64>,
}

pub fn init_pockets() -> Vec<Pocket> {
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

pub fn init_cushions() -> Vec<Cushion> {
    vec![
        Cushion {
            r1: array![0.0, 0.0],
            r2: array![WIDTH, 0.0],
            ctype: CushionType::BOTTOM,
        },
        Cushion {
            r1: array![0.0, 0.0],
            r2: array![0.0, LENGTH],
            ctype: CushionType::LEFT,
        },
        Cushion {
            r1: array![0.0, LENGTH],
            r2: array![WIDTH, LENGTH],
            ctype: CushionType::TOP,
        },
        Cushion {
            r1: array![WIDTH, 0.0],
            r2: array![WIDTH, LENGTH],
            ctype: CushionType::RIGHT,
        },
    ]
}
