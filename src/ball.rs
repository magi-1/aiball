use crate::{DELTA, G, MU};
use ndarray::{array, Array1, Array2};

#[derive(Debug, PartialEq)]
pub enum BallType {
    SOLID,
    EIGHT,
    STRIPED,
    CUE,
}

#[derive(Debug, PartialEq)]
pub enum BallState {
    STATIONARY,
    MOVING,
    POCKETED,
}

pub struct Ball {
    pub id: usize,
    pub btype: BallType,
    pub bstate: BallState,
    pub r: Array1<f64>,
    pub v: Array1<f64>,
    pub phi: f64,
    pub mag_v: f64,
    pub r_coeffs: Array2<f64>,
    pub v_coeffs: Array2<f64>,
}

impl Ball {
    pub fn new(id: usize, btype: BallType) -> Self {
        Ball {
            id,
            btype,
            bstate: BallState::STATIONARY,
            r: array![0.0, 0.0],
            v: array![0.0, 0.0],
            phi: 0.0,
            mag_v: 0.0,
            r_coeffs: Array2::<f64>::zeros((3, 2)),
            v_coeffs: Array2::<f64>::zeros((2, 2)),
        }
    }

    pub fn update_state(&mut self, time_delta: f64) {
        self.r = self.r_t(time_delta);
        self.v = self.v_t(time_delta);
        self.mag_v = (&self.v.dot(&self.v)).sqrt();
        let cos_phi: f64 = self.phi.cos();
        let sin_phi: f64 = self.phi.sin();
        let grav_fric: f64 = -G * MU;
        let mag_v_cos_phi: f64 = self.mag_v * cos_phi;
        let mag_v_sin_phi: f64 = self.mag_v * sin_phi;
        self.r_coeffs = array![
            [self.r[0], mag_v_cos_phi, grav_fric * cos_phi],
            [self.r[1], mag_v_sin_phi, grav_fric * sin_phi]
        ];
        self.v_coeffs = array![
            [mag_v_cos_phi, grav_fric * cos_phi],
            [mag_v_sin_phi, grav_fric * sin_phi]
        ];

        if self.mag_v < DELTA {
            self.bstate = BallState::STATIONARY;
        } else {
            self.bstate = BallState::MOVING;
        }
    }

    pub fn r_t(&self, t: f64) -> Array1<f64> {
        self.r_coeffs.dot(&array![1.0, t, t.powi(2)])
    }

    pub fn v_t(&self, t: f64) -> Array1<f64> {
        self.v_coeffs.dot(&array![1.0, t, t.powi(2)])
    }

    pub fn reset(&mut self) {
        self.r = array![0.0, 0.0];
        self.v = array![0.0, 0.0];
        self.phi = 0.0;
        self.mag_v = 0.0;
        self.r_coeffs = Array2::<f64>::zeros((3, 2));
        self.v_coeffs = Array2::<f64>::zeros((2, 2));
        self.bstate = BallState::STATIONARY;
    }

    pub fn is_pocketed(&self) -> bool {
        self.bstate == BallState::POCKETED
    }

    pub fn is_moving(&self) -> bool {
        self.bstate == BallState::MOVING
    }
}

impl PartialEq for Ball {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
