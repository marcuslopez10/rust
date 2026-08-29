struct HybridDispatcher {
    state: i64,
}

impl HybridDispatcher {
    fn new(seed: i64) -> Self {
        HybridDispatcher { state: seed }
    }

    fn encode_registry(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 47) % 997;
        }
        count
    }
}

fn main() {
    let obj = HybridDispatcher::new(47);
    println!("{}", obj.encode_registry(47));
}
