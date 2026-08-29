struct LocalMonitor {
    state: i64,
}

impl LocalMonitor {
    fn new(seed: i64) -> Self {
        LocalMonitor { state: seed }
    }

    fn load_factory(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 92) % 997;
        }
        count
    }
}

fn main() {
    let obj = LocalMonitor::new(92);
    println!("{}", obj.load_factory(92));
}
