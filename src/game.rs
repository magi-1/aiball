use crate::events::*;
use crate::pool_balls::*;
use crate::pool_table::*;
use crate::ball::*;

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

    fn new() -> Game{
        Game {
            balls: PoolBalls::new(),
            table: PoolTable::new(),
        }
    }

    fn make_move(&mut self) {}

    fn get_next_event(&mut self) -> Option<Box<dyn Event>> {

        let mut next_event: Option<Box<dyn Event>> = None;

        for ball in self.balls.balls.iter_mut() {

            if !ball.is_pocketed() {
                
                if ball.is_moving(){

                    let b_event = StopRolling::new(ball);
                    //mut_compare(&mut next_event, b_event);
                    
                    for pocket in &self.table.pockets {
                        let p_event = HitPocket::new(ball, pocket);
                        //mut_compare(&mut next_event, p_event);
                    }

                    for cushion in &self.table.cushions {
                        let c_event = HitCushion::new(ball, cushion);
                        //mut_compare(&mut next_event, c_event);
                    }

                    // add HitBall Event
                }  
            }
        }
        next_event
    }

    fn step_sim(&mut self) {
        while let Some(mut event) = self.get_next_event() {
            (*event).apply();
            for ball in &mut self.balls.balls {
                ball.update_state(event.get_time_until());
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