use crate::{G, MU};
use ndarray::{array, Array1};

#[derive(Debug)]
pub enum BallType {
    SOLID,
    EIGHT,
    STRIPED,
    CUE,
    UNASSIGNED,
}

pub struct Ball {
    pub btype: BallType,
    pub r: Array1<f64>,
    pub v: Array1<f64>,
}

impl Ball {
    pub fn new(btype: BallType) -> Ball {
        Ball {
            btype,
            r: array![0.0, 0.0],
            v: array![0.0, 0.0],
        }
    }

    pub fn mag_v(&self) -> f64 {
        (&self.v.dot(&self.v)).sqrt()
    }

    pub fn p_t(&self, t: f64) -> Array1<f64> {
        &self.r + (self.mag_v() * t - 0.5 * MU * G * t) * &self.v
    }

    pub fn v_t(&self, t: f64) -> Array1<f64> {
        (self.mag_v() - MU * G * t) * &self.v
    }
}
