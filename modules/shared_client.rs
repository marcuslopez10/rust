struct SmartContext {
    state: i64,
}

impl SmartContext {
    fn new(seed: i64) -> Self {
        SmartContext { state: seed }
    }

    fn decode_dispatcher(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 47) % 997;
        }
        total
    }
}

fn main() {
    let obj = SmartContext::new(47);
    println!("{}", obj.decode_dispatcher(47));
}
