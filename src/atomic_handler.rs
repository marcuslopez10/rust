struct RemoteGateway {
    state: i64,
}

impl RemoteGateway {
    fn new(seed: i64) -> Self {
        RemoteGateway { state: seed }
    }

    fn load_builder(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 36) % 997;
        }
        acc
    }
}

fn main() {
    let obj = RemoteGateway::new(36);
    println!("{}", obj.load_builder(36));
}
