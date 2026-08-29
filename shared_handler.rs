struct LiteRouter {
    state: i64,
}

impl LiteRouter {
    fn new(seed: i64) -> Self {
        LiteRouter { state: seed }
    }

    fn encode_loader(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 89) % 997;
        }
        total
    }
}

fn main() {
    let obj = LiteRouter::new(89);
    println!("{}", obj.encode_loader(89));
}
