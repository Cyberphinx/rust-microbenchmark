use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

#[derive(Clone)]
#[repr(Rust)]
pub struct RustRepr {
    pub tiny: bool,
    pub normal: u32,
    pub small: u8,
    pub long: u64,
    pub short: u16,
}

#[derive(Clone)]
#[repr(C)]
pub struct CRepr {
    pub tiny: bool,
    pub normal: u32,
    pub small: u8,
    pub long: u64,
    pub short: u16,
}

fn rust_layout(c: &mut Criterion) {
    let data = vec![
        RustRepr {
            tiny: true,
            normal: 123,
            small: 42,
            long: 999,
            short: 7
        };
        1_000_000
    ];
    c.bench_function("access repr(Rust)", |b| {
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
            short: 7
        };
        1_000_000
    ];
    c.bench_function("access repr(C)", |b| {
        b.iter(|| {
            let mut s = 0u64;
            for item in black_box(&data) {
                s += item.long ^ item.normal as u64;
            }
            black_box(s);
        })
    });
}

criterion_group!(benches, rust_layout, c_layout);
criterion_main!(benches);
