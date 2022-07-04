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

    fn get_next_event<'a, 'b: 'a>(&'b mut self) -> EventEnum<'a> {
        let mut ij_time = (0, 0, f64::INFINITY);
        for i in 0..(self.balls.num_balls() - 1) {
            for j in (i + 1)..self.balls.num_balls() {
                if i != j {
                    let (first, second) = self.balls.balls.split_at_mut(j);
                    let b1 = &mut first[i];
                    let b2 = &mut second[0];

                    let mut bb_event = HitBall::new(b1, b2);

                    if ij_time.2 > bb_event.get_time_until() {
                        ij_time = (i, j, bb_event.get_time_until())
                    }
                }
            }
        }

        let mut next_event: EventEnum = EventEnum::NullEvent(NullEvent::new());
        for ball in self.balls.balls.iter_mut() {
            if !ball.is_pocketed() {
                if ball.is_moving() {
                    let mut b_event = EventEnum::StopRolling(StopRolling::new(ball));

                    for pocket in &self.table.pockets {
                        let mut p_event = EventEnum::HitPocket(HitPocket::new(ball, pocket));
                    }

                    for cushion in &self.table.cushions {
                        let mut c_event = EventEnum::HitCushion(HitCushion::new(ball, cushion));
                    }
                }
            }
        }

        if next_event.get_time_until() > ij_time.2 {
            let (first, second) = self.balls.balls.split_at_mut(ij_time.1);
            let b1 = &mut first[ij_time.0];
            let b2 = &mut second[ij_time.1];
            next_event = EventEnum::HitBall(HitBall::new(b1, b2));
        }
        next_event
    }

    // b_event.calculate_time_until();
    // if b_event.get_time_until() < (*next_event).get_time_until() {
    //     next_event = Box::new(b_event);
    // }

    fn step_sim(&mut self) {
        loop {
            let mut event = self.get_next_event();
            let time_delta = event.get_time_until();
            event.apply();
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
