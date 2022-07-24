use ambassador::{delegatable_trait, Delegate};
use ndarray::{array, Array1, Array2};
use roots::{find_roots_quadratic, find_roots_quartic, Roots};

use crate::ball::{Ball, BallState};
use crate::math::{angle_between, mut_normalize, norm, normalize};
use crate::pool_table::{Cushion, CushionType, Pocket};

use crate::{SimObjects, G, MU, R};

#[delegatable_trait]
pub trait Event {
    fn get_time_until(&self) -> f64;
    fn calculate_time_until(&mut self, objects: &SimObjects);
    fn apply(&self, objects: &mut SimObjects);
}

#[derive(Delegate)]
#[delegate(Event)]
pub enum EventEnum {
    StopRolling(StopRolling),
    HitBall(HitBall),
    HitCushion(HitCushion),
    HitPocket(HitPocket),
    NullEvent(NullEvent),
}

impl Default for EventEnum {
    fn default() -> Self {
        EventEnum::NullEvent(NullEvent::new())
    }
}

impl EventEnum {
    pub fn mut_compare(&mut self, other: EventEnum) {
        if other.get_time_until() < self.get_time_until() {
            *self = other;
        }
    }
}

pub struct NullEvent {
    time_delta: f64,
}

impl NullEvent {
    pub fn new() -> Self {
        NullEvent {
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for NullEvent {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&mut self, _objects: &SimObjects) {}

    fn apply(&self, objects: &mut SimObjects) {}
}

pub struct StopRolling {
    ball_id: usize,
    time_delta: f64,
}

impl StopRolling {
    pub fn new(ball_id: usize) -> Self {
        Self {
            ball_id,
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for StopRolling {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&mut self, objects: &SimObjects) {
        let ball: &Ball = objects.get_ball(self.ball_id);
        self.time_delta = ball.mag_v / (MU * G);
    }

    fn apply(&self, objects: &mut SimObjects) {
        let b: &mut Ball = objects.get_mut_ball(self.ball_id);
        b.v = array![0.0, 0.0];
        b.mag_v = 0.0;
        b.bstate = BallState::STATIONARY;
        b.set_coeffs();
    }
}

pub struct HitPocket {
    ball_id: usize,
    pocket_id: usize,
    time_delta: f64,
}

impl HitPocket {
    pub fn new(ball_id: usize, pocket_id: usize) -> Self {
        Self {
            ball_id,
            pocket_id,
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for HitPocket {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&mut self, objects: &SimObjects) {
        let b: &Ball = objects.get_ball(self.ball_id);

        if b.is_moving() {
            let p: &Array1<f64> = &objects.get_pocket(self.pocket_id).r;
            let c: &Array2<f64> = &b.r_coeffs;

            let a4: f64 = 0.5 * (c[[0, 2]].powi(2) + c[[1, 2]].powi(2));
            let a3: f64 = c[[0, 2]] * c[[0, 1]] + c[[1, 2]] * c[[1, 1]];
            let a2: f64 = c[[0, 0]] * (c[[0, 0]] - p[0])
                + c[[1, 2]] * (c[[1, 0]] - p[1])
                + 0.5 * (c[[0, 1]] + c[[1, 1]]).powi(2);
            let a1: f64 = c[[0, 1]] * (c[[0, 0]] - p[0]) + c[[1, 1]] * (c[[1, 0]] - p[1]);
            let a0: f64 = 0.5
                * (p[0].powi(2) + p[1].powi(2) + c[[0, 0]].powi(2) + c[[1, 0]].powi(2) - R.powi(2))
                - (c[[0, 0]] * p[0] + c[[1, 0]] * p[1]);
            let roots: Roots<f64> = find_roots_quartic(a4, a3, a2, a1, a0);

            let min_root: Option<f64> = smallest_root(roots);

            if let Some(time_delta) = min_root {
                self.time_delta = time_delta;
            }
        }
    }

    fn apply(&self, objects: &mut SimObjects) {
        let b: &mut Ball = objects.get_mut_ball(self.ball_id);
        b.reset();
        b.bstate = BallState::POCKETED;
    }
}

pub struct HitCushion {
    ball_id: usize,
    cushion_id: usize,
    time_delta: f64,
}

impl HitCushion {
    pub fn new(ball_id: usize, cushion_id: usize) -> Self {
        Self {
            ball_id,
            cushion_id,
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for HitCushion {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&mut self, objects: &SimObjects) {
        let b: &Ball = objects.get_ball(self.ball_id);

        let cushion: &Cushion = objects.get_cushion(self.cushion_id);
        let r1: &Array1<f64> = &cushion.r1;
        let r2: &Array1<f64> = &cushion.r2;
        let c: &Array2<f64> = &b.r_coeffs;

        let lx: f64 = -(r2[1] - r1[1]) / (r2[1] - r1[0]);
        let ly: f64 = 1.0;
        let l0: f64 = -lx * r1[0] - r1[1];

        let a2: f64 = lx * c[[0, 2]] + ly * c[[1, 2]];
        let a1: f64 = lx * c[[0, 1]] + ly * c[[1, 1]];
        let a0_stem: f64 = l0 + lx * c[[0, 0]] + ly * c[[1, 0]];
        let a0_norm: f64 = R * (lx.powi(2) + ly.powi(2)).sqrt();
        let a0_pos: f64 = a0_stem + a0_norm;
        let a0_neg: f64 = a0_stem - a0_norm;
        let roots1: Roots<f64> = find_roots_quadratic(a2, a1, a0_pos);
        let min_root1: Option<f64> = smallest_root(roots1);
        let roots2: Roots<f64> = find_roots_quadratic(a2, a1, a0_neg);
        let min_root2: Option<f64> = smallest_root(roots2);

        if let Some(time_delta) = min_root1 {
            self.time_delta = time_delta;
        }
        if let Some(other_time_delta) = min_root2 {
            if other_time_delta < self.time_delta {
                self.time_delta = other_time_delta;
            }
        }
    }

    fn apply(&self, objects: &mut SimObjects) {
        let cushion: &Cushion = objects.get_cushion(self.cushion_id);
        let mut index: usize = 0;
        if cushion.ctype == CushionType::TOP || cushion.ctype == CushionType::BOTTOM {
            index = 1;
        }
        let b: &mut Ball = objects.get_mut_ball(self.ball_id);
        b.v[index] = -1.0 * b.v[index];
    }
}

pub struct HitBall {
    ball_id: usize,
    other_ball_id: usize,
    time_delta: f64,
}

impl HitBall {
    pub fn new(ball_id: usize, other_ball_id: usize) -> Self {
        Self {
            ball_id,
            other_ball_id,
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for HitBall {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&mut self, objects: &SimObjects) {
        let b1: &Ball = objects.get_ball(self.ball_id);
        let b2: &Ball = objects.get_ball(self.other_ball_id);
        let c: Array2<f64> = &b1.r_coeffs - &b2.r_coeffs;

        let a4: f64 = c[[0, 2]].powi(2) + c[[1, 2]].powi(2);
        let a3: f64 = 2.0 * (c[[0, 2]] * c[[0, 1]] + c[[1, 2]] * c[[1, 1]]);
        let a2: f64 = c[[0, 1]].powi(2)
            + c[[1, 1]].powi(2)
            + 2.0 * (c[[0, 2]] * c[[0, 0]] + c[[1, 2]] * c[[1, 0]]);
        let a1: f64 = 2.0 * (c[[0, 1]] * c[[0, 0]] + c[[1, 1]] * c[[1, 0]]);
        let a0: f64 = c[[0, 0]].powi(2) + c[[1, 0]].powi(2) - 4.0 * R.powi(2);
        let roots: Roots<f64> = find_roots_quartic(a4, a3, a2, a1, a0);

        let min_root: Option<f64> = smallest_root(roots);

        if let Some(time_delta) = min_root {
            self.time_delta = time_delta;
        }
    }

    fn apply(&self, objects: &mut SimObjects) {
        let b1: &Ball = objects.get_ball(self.ball_id);
        let b2: &Ball = objects.get_ball(self.other_ball_id);

        let mut dr: Array1<f64> = &b2.r - &b1.r;
        mut_normalize(&mut dr);
        
        let mut v1_next: Array1<f64> = array![0.0,0.0];
        let mut v2_next: Array1<f64> = array![0.0,0.0];
        match b2.bstate {
            BallState::STATIONARY => {
                let (v1, v2): (Array1<f64>, Array1<f64>) =
                    elastic_collision(&dr, &b1.v, &b2.v);
                v1_next = v1;
                v2_next = v2;
            }
            BallState::MOVING => {
                let v1_tmp: Array1<f64> = &b1.v - &b2.v;
                let v2_tmp: Array1<f64> = &b2.v - &b2.v;
                let (v1, v2): (Array1<f64>, Array1<f64>) =
                    elastic_collision(&dr, &v1_tmp, &v2_tmp);
                v1_next = v1 + &b2.v;
                v2_next = v2 + &b2.v;
            }
            _ => {}
        };

        let b1: &mut Ball = objects.get_mut_ball(self.ball_id);
        b1.v = v1_next;
        let b2: &mut Ball = objects.get_mut_ball(self.other_ball_id);
        b2.v = v2_next;
        b2.bstate = BallState::MOVING;
    }
}

fn smallest_root(roots: Roots<f64>) -> Option<f64> {
    fn smallest_positive_value(roots: &[f64]) -> Option<f64> {
        roots.iter().copied().min_by(|x, y| x.total_cmp(y))
    }
    match roots {
        Roots::One(values) => smallest_positive_value(&values),
        Roots::Two(values) => smallest_positive_value(&values),
        Roots::Three(values) => smallest_positive_value(&values),
        Roots::Four(values) => smallest_positive_value(&values),
        _ => None,
    }
}

fn elastic_collision(
    dr: &Array1<f64>,
    v1: &Array1<f64>,
    v2: &Array1<f64>,
) -> (Array1<f64>, Array1<f64>) {
    let alpha: f64 = angle_between(v1, v2);
    let v_agg: Array1<f64> = v1 + v2;
    let v_mag: f64 = norm(&v_agg);
    (
        v_mag * alpha.sin() * dr,
        v_mag * alpha.cos() * (v_agg / v_mag - dr),
    )
}
