use events::{Event, EventEnum};

pub mod ball;
pub mod events;
pub mod game;
pub mod pool_balls;
pub mod pool_table;

pub const DELTA: f64 = 0.001; // Friction: kg*cm*s^-2
pub const MU: f64 = 1.0; // Friction: kg*cm*s^-2
pub const R: f64 = 2.8575; // Ball Radius: cm
pub const G: f64 = 980.665; // Gravity: cm/s^2
pub const PI: f64 = std::f64::consts::PI;

pub struct SimObjects {
    pub balls: Vec<ball::Ball>,
    pub pockets: Vec<pool_table::Pocket>,
    pub cushions: Vec<pool_table::Cushion>,
}

impl SimObjects {
    pub fn new() -> Self {
        let balls: Vec<ball::Ball> = pool_balls::rack();
        let pockets: Vec<pool_table::Pocket> = pool_table::init_pockets();
        let cushions: Vec<pool_table::Cushion> = pool_table::init_cushions();
        Self {
            balls,
            pockets,
            cushions,
        }
    }

    pub fn num_balls(&self) -> usize {
        self.balls.len()
    }

    pub fn get_ball(&self, ball_id: usize) -> &ball::Ball {
        &self.balls[ball_id]
    }

    pub fn get_mut_ball(&mut self, ball_id: usize) -> &mut ball::Ball {
        &mut self.balls[ball_id]
    }

    pub fn get_mut_cue(&mut self) -> &mut ball::Ball {
        &mut self.balls[15]
    }

    pub fn get_pocket(&self, pocket_id: usize) -> &pool_table::Pocket {
        &self.pockets[pocket_id]
    }

    pub fn get_cushion(&self, cushion_id: usize) -> &pool_table::Cushion {
        &self.cushions[cushion_id]
    }

    pub fn num_pockets(&self) -> usize {
        self.pockets.len()
    }

    pub fn num_cushions(&self) -> usize {
        self.cushions.len()
    }

    pub fn apply_event(&mut self, event: EventEnum) {
        let time_delta: f64 = event.get_time_until();
        for ball in &mut self.balls {
            ball.update_state(time_delta);
        }
        event.apply(&mut self.balls);
    }
}
