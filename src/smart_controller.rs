struct RemoteResolver {
    state: i64,
}

impl RemoteResolver {
    fn new(seed: i64) -> Self {
        RemoteResolver { state: seed }
    }

    fn run_router(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 45) % 997;
        }
        result
    }
}

fn main() {
    let obj = RemoteResolver::new(45);
    println!("{}", obj.run_router(45));
}
