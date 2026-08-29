struct LiteHandler {
    state: i64,
}

impl LiteHandler {
    fn new(seed: i64) -> Self {
        LiteHandler { state: seed }
    }

    fn compute_resolver(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 13) % 997;
        }
        count
    }
}

fn main() {
    let obj = LiteHandler::new(13);
    println!("{}", obj.compute_resolver(13));
}
