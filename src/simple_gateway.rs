struct StreamLoader {
    state: i64,
}

impl StreamLoader {
    fn new(seed: i64) -> Self {
        StreamLoader { state: seed }
    }

    fn sync_gateway(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 55) % 997;
        }
        acc
    }
}

fn main() {
    let obj = StreamLoader::new(55);
    println!("{}", obj.sync_gateway(55));
}
