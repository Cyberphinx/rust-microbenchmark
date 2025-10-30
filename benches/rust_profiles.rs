use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

#[derive(Clone)]
#[repr(Rust)]
pub struct RustRepr {
    pub long: u64,
    pub normal: u32,
    pub short: u16,
    pub small: u8,
    pub tiny: bool,
}

#[derive(Clone)]
#[repr(C)]
pub struct CRepr {
    pub long: u64,
    pub normal: u32,
    pub short: u16,
    pub small: u8,
    pub tiny: bool,
}

#[derive(Clone)]
pub struct StructOfVecs {
    pub longs: Vec<u64>,
    pub normals: Vec<u32>,
    pub shorts: Vec<u16>,
    pub smalls: Vec<u8>,
    pub tinies: Vec<bool>,
}

impl StructOfVecs {
    pub fn new(n: usize) -> Self {
        Self {
            longs: vec![999u64; n],
            normals: vec![123u32; n],
            shorts: vec![7u16; n],
            smalls: vec![42u8; n],
            tinies: vec![true; n],
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.longs.len()
    }
}

fn rust_layout(c: &mut Criterion) {
    let data = vec![
        RustRepr {
            tiny: true,
            normal: 123,
            small: 42,
            long: 999,
            short: 7,
        };
        1_000_000
    ];
    c.bench_function("repr(Rust)", |b| {
        b.iter(|| {
            let mut s = 0u64;
            for item in black_box(&data) {
                s += item.long ^ item.normal as u64;
            }
            black_box(s);
        })
    });
}

fn c_layout(c: &mut Criterion) {
    let data = vec![
        CRepr {
            tiny: true,
            normal: 123,
            small: 42,
            long: 999,
            short: 7,
        };
        1_000_000
    ];
    c.bench_function("repr(C)", |b| {
        b.iter(|| {
            let mut s = 0u64;
            for item in black_box(&data) {
                s += item.long ^ item.normal as u64;
            }
            black_box(s);
        })
    });
}

fn struct_of_vecs_layout(c: &mut Criterion) {
    const N: usize = 1_000_000;
    let data = StructOfVecs::new(N);

    c.bench_function("Struct of Vecs", |b| {
        b.iter(|| {
            let mut s = 0u64;
            for i in 0..black_box(data.len()) {
                s += data.longs[i] ^ data.normals[i] as u64;
            }
            black_box(s);
        })
    });
}

criterion_group!(benches, rust_layout, c_layout, struct_of_vecs_layout);
criterion_main!(benches);
