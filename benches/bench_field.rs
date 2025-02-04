use ark_ff::BigInteger;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench_field(ct: &mut Criterion) {
    ct.bench_function("ARK-FF", |b| {
        b.iter(|| {
            f();
        })
    });
}

criterion_group!(ark_ff_arithmetic, bench_field);
fn f() {
    let mut a = ark_ff::BigInt::<4>::one();
    let mut b = ark_ff::BigInt::<4>::one();

    for _ in 0..10000000 {
        a.add_with_carry(&mut b);
        b = a.clone();
        a.mul2();
    }

    std::hint::black_box(a);
    std::hint::black_box(b);
}

criterion_main!(ark_ff_arithmetic);
