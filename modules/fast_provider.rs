struct LiteDispatcher {
    state: i64,
}

impl LiteDispatcher {
    fn new(seed: i64) -> Self {
        LiteDispatcher { state: seed }
    }

    fn load_collector(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 32) % 997;
        }
        value
    }
}

fn main() {
    let obj = LiteDispatcher::new(32);
    println!("{}", obj.load_collector(32));
}
