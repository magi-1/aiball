use crate::ball::*;
use crate::events::*;
use crate::pool_balls::*;
use crate::pool_table::*;

use std::boxed::Box;

// https://dhghomon.github.io/easy_rust/Chapter_54.html

enum GameState {
    BALLINHAND,
}

struct Game {
    balls: PoolBalls,
    table: PoolTable,
}

impl Game {
    fn new() -> Game {
        Game {
            balls: PoolBalls::new(),
            table: PoolTable::new(),
        }
    }

    fn make_move(&mut self) {}

    fn get_next_event(&mut self) -> Box<dyn Event> {
        let mut next_event: Box<dyn Event> = Box::new(NullEvent::new());

        for ball in self.balls.balls.iter_mut() {
            if !ball.is_pocketed() {
                if ball.is_moving() {
                    let mut b_event = StopRolling::new(ball);
                    b_event.calculate_time_until();
                    if b_event.get_time_until() < (*next_event).get_time_until() {
                        next_event = Box::new(b_event);
                    }
                    for pocket in &self.table.pockets {
                        let mut p_event = HitPocket::new(ball, pocket);
                    }

                    for cushion in &self.table.cushions {
                        let mut c_event = HitCushion::new(ball, cushion);
                    }

                    // add HitBall Event
                }
            }
        }
        next_event
    }

    fn step_sim(&mut self) {
        loop {
            let mut event = self.get_next_event();
            let time_delta = event.get_time_until();
            (*event).apply();
            for ball in self.balls.balls.iter_mut() {
                ball.update_state(time_delta);
            }
        }
    }

    fn set_game_state(&mut self) {
        /*
        This doesnt update things about the balls, but it reads the balls and sets game level
        descriptors so that users know who is up, what balls are viable, etc.
            - current_player
            - game_state
        */
    }

    fn play_turn(&mut self) {
        self.make_move();
        self.step_sim();
        self.set_game_state();
    }
}
