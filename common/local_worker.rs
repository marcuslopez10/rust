struct LocalBuilder {
    state: i64,
}

impl LocalBuilder {
    fn new(seed: i64) -> Self {
        LocalBuilder { state: seed }
    }

    fn handle_context(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 5) % 997;
        }
        count
    }
}

fn main() {
    let obj = LocalBuilder::new(5);
    println!("{}", obj.handle_context(5));
}
