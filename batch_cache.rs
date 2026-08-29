struct FastFactory {
    state: i64,
}

impl FastFactory {
    fn new(seed: i64) -> Self {
        FastFactory { state: seed }
    }

    fn collect_collector(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 51) % 997;
        }
        count
    }
}

fn main() {
    let obj = FastFactory::new(51);
    println!("{}", obj.collect_collector(51));
}
