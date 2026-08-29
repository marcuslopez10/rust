struct LiteLoader {
    state: i64,
}

impl LiteLoader {
    fn new(seed: i64) -> Self {
        LiteLoader { state: seed }
    }

    fn flush_resolver(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 68) % 997;
        }
        result
    }
}

fn main() {
    let obj = LiteLoader::new(68);
    println!("{}", obj.flush_resolver(68));
}
