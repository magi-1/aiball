use crate::{G, MU, DELTA};
use ndarray::{array, Array1};

#[derive(Debug)]
pub enum BallType {
    SOLID,
    EIGHT,
    STRIPED,
    CUE,
}

#[derive(Debug)]
pub enum BallState {
    STATIONARY,
    MOVING,
    POCKETED,
}

pub struct Ball {
    pub btype: BallType,
    pub r: Array1<f64>,
    pub v: Array1<f64>,
    pub mag_v: f64
}

impl Ball {

    pub fn new(btype: BallType) -> Ball {
        Ball {
            btype,
            r: array![0.0, 0.0],
            v: array![0.0, 0.0],
            mag_v: 0.0
        }
    }

    pub fn update_velocity(&mut self, v: Array1<f64>) {
        self.v = v;
        self.mag_v = (&self.v.dot(&self.v)).sqrt();
     }

    pub fn p_t(&self, t: f64) -> Array1<f64> {
        &self.r + (self.mag_v * t - 0.5 * MU * G * t) * &self.v
    }

    pub fn v_t(&self, t: f64) -> f64 {
        self.mag_v - MU * G * t
    }

    pub fn is_moving(&self, t: f64) -> bool {
        self.v_t(t) < DELTA
    }
}
