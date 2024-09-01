use criterion::{criterion_group, criterion_main, Criterion, black_box};
use serde::{Serialize, Deserialize};
use std::collections::{VecDeque, BTreeMap};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Level {
    pub symbol: String,
    pub price:i64,
    pub level: VecDeque<u64>,
}

impl Default for Level {
    fn default() -> Self {
        Level {
            symbol: String::new(),
            price: 0,
            level: VecDeque::new(),
        }
    }
}

impl Level {
    pub fn new(symbol: String, price: i64) -> Self {
        Level {
            symbol,
            price,
            level: VecDeque::new(),
        }
    }

    pub fn binary_size(&self) -> usize {
        self.symbol.len() + 8 + self.level.len() * 8
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct Ladder {
    pub levels: BTreeMap<i64, Level>,
}

impl std::fmt::Debug for Ladder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.levels {
            writeln!(f, "price: {}, symbol: {}, level: {:?}", k, v.symbol, v.level)?;
        }
        Ok(())
    }
}

impl Ladder {
    pub fn new() -> Self {
        Ladder {
            levels: BTreeMap::new(),
        }
    }

    pub fn test_data() -> Self {
        let mut ladder = Ladder::new();
        for i in 0..10 {
            let mut level = Level::default();
            level.symbol = format!("BTC/USD");
            level.price = 1000 + i;
            level.level.push_back(1000);
            level.level.push_back(2000);
            ladder.levels.insert(1000 + i, level);
        }
        ladder
    }

    fn binary_size(&self) -> usize {
        self.levels.iter().map(|(k, v)| k.to_string().len() + v.binary_size()).sum()
    }

    pub fn byte_serialize(&self) -> Vec<u8> {
        let size = self.binary_size();
        let mut buf = Vec::with_capacity(size);
        for (k, v) in &self.levels {
            buf.extend_from_slice(&k.to_le_bytes());
            buf.extend_from_slice(&v.symbol.len().to_le_bytes());
            buf.extend_from_slice(v.symbol.as_bytes());
            buf.extend_from_slice(&v.price.to_le_bytes());
            buf.extend_from_slice(&(v.level.len() as u64).to_le_bytes());
            for l in &v.level {
                buf.extend_from_slice(&l.to_le_bytes());
            }
        }
        buf   
    }

}

#[derive(Clone, Serialize, Deserialize)]
struct SmallVec {
    pub data: Vec<u64>,
}

impl Default for SmallVec {
    fn default() -> Self {
        SmallVec {
            data: (1..=20).collect(),
        }
    }
}

impl SmallVec {
    pub fn binary_size(&self) -> usize {
        self.data.len() * 8
    }

    pub fn byte_serialize(&self) -> Vec<u8> {
        let size = self.binary_size();
        let mut buf = Vec::with_capacity(size);
        for l in &self.data {
            buf.extend_from_slice(&l.to_le_bytes());
        }
        buf
    }
}

fn bench_ladder(c: &mut Criterion) {
    let ladder = Ladder::test_data();
    println!("ladder: {:?}", ladder);
    let mut group = c.benchmark_group("cloning");

    group.bench_function("cloning", |b| b.iter(|| {
        black_box(ladder.clone());
    }));

    group.bench_function("binary_size", |b| b.iter(|| {
        black_box(ladder.byte_serialize());
    }));

    group.finish();
}

criterion_group!(benches, bench_ladder);
criterion_main!(benches);