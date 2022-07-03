use aiball::pool_balls::PoolBalls;

fn main() {
    let balls = PoolBalls::new();
    for b in balls.balls {
        println!("{:?}", b.btype);
    }
}
