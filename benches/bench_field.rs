use ark_bls12_377::Bls12_377;
pub use ark_bls12_377::Fr as FF_Bls12_377Fr;
use ark_ff::{BigInteger, UniformRand};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use rand_xorshift::XorShiftRng;
use rand::SeedableRng;

fn bench_field(ct: &mut Criterion) {
    // ct.bench_function("ARK-FF", |b| {
    //     b.iter(|| {
    //         f();
    //     })
    // });
    //
    //let mut rng = thread_rng();
    let mut rng = XorShiftRng::seed_from_u64(3141519u64);
    //
    // ct.bench_function("BLS-ARK", |b| {
    //     b.iter_batched(
    //         || {
    //             let mut a: FF_Bls12_377Fr = rng.sample(Standard {});
    //             a
    //         },
    //         |a: FF_Bls12_377Fr| {
    //             f1(a);
    //         },
    //         BatchSize::SmallInput,
    //     )
    // });

    ct.bench_function("HALO 2 CURVES", |b| {
        b.iter_batched(
            || {
                let mut a =  <halo2curves::bls12381::Fr as halo2curves::ff::Field>::random(&mut rng);
                //let mut a= halo2curves::bls12381::Fr::rand(&mut rng);
                a
            },
            |a: halo2curves::bls12381::Fr| {
                f2(a);
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(ark_ff_arithmetic, bench_field);
// fn f() {
//     let mut a = ark_ff::BigInt::<4>::one();
//     let mut b = ark_ff::BigInt::<4>::one();
//
//     for _ in 0..10000000 {
//         a.add_with_carry(&mut b);
//         b = a.clone();
//         a.mul2();
//     }
//
//     std::hint::black_box(a);
//     std::hint::black_box(b);
// }
//
// fn f1(a: FF_Bls12_377Fr) {
//     let mut b = a.clone();
//     for i in 0..10000000 {
//         b = (b + a) * a;
//     }
//     std::hint::black_box(b);
// }

fn f2(a: halo2curves::bls12381::Fr) {
    let mut b = a.clone();
    for i in 0..10000000 {
        b = (b + a) * a;
    }

    std::hint::black_box(b);
}

criterion_main!(ark_ff_arithmetic);
