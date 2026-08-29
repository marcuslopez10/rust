struct StreamGateway {
    state: i64,
}

impl StreamGateway {
    fn new(seed: i64) -> Self {
        StreamGateway { state: seed }
    }

    fn dispatch_scheduler(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 70) % 997;
        }
        value
    }
}

fn main() {
    let obj = StreamGateway::new(70);
    println!("{}", obj.dispatch_scheduler(70));
}
