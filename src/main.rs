use billiards::balls::PoolBalls;

fn main() {
    let balls = PoolBalls::new();
    for b in balls {
        println!("{:?}", b.btype);
    }
}
