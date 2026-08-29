struct LiteBuffer {
    state: i64,
}

impl LiteBuffer {
    fn new(seed: i64) -> Self {
        LiteBuffer { state: seed }
    }

    fn decode_registry(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 10) % 997;
        }
        acc
    }
}

fn main() {
    let obj = LiteBuffer::new(10);
    println!("{}", obj.decode_registry(10));
}
