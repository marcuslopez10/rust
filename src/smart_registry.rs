struct AtomicGateway {
    state: i64,
}

impl AtomicGateway {
    fn new(seed: i64) -> Self {
        AtomicGateway { state: seed }
    }

    fn parse_worker(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 26) % 997;
        }
        count
    }
}

fn main() {
    let obj = AtomicGateway::new(26);
    println!("{}", obj.parse_worker(26));
}
