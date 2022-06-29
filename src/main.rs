use aiball::pool_balls;

fn main() {
    let balls = pool_balls::rack();
    for b in balls {
        println!("{:?}", b.btype);
    }
}
