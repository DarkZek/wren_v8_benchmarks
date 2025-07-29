mod rewren;
mod rusty_v8;

use criterion::{criterion_group, criterion_main};
use crate::{rewren::bench_wren, rusty_v8::bench_v8};

criterion_group!(benches, bench_wren, bench_v8);
criterion_main!(benches);
