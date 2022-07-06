pub mod ball;
pub mod events;
pub mod game;
pub mod pool_balls;
pub mod pool_table;

pub const DELTA: f64 = 0.001; // Friction: kg*cm*s^-2
pub const MU: f64 = 1.0; // Friction: kg*cm*s^-2
pub const R: f64 = 2.8575; // Ball Radius: cm
pub const G: f64 = 980.665; // Gravity: cm/s^2

pub struct SimObjects {
    pub pool_balls: pool_balls::PoolBalls,
    pub pool_table: pool_table::PoolTable,
}

impl SimObjects {
    pub fn new() -> Self {
        Self {
            pool_balls: pool_balls::PoolBalls::new(),
            pool_table: pool_table::PoolTable::new(),
        }
    }
}
