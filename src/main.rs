use halo2curves::ff::Field;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::time::Instant;
use ark_bls12_377::Bls12_377;
pub use ark_bls12_377::Fr as FF_Bls12_377Fr;
use ark_ff::{BigInteger, Field as OtherField, UniformRand};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};

fn main() {
    let start = Instant::now(); // Start the timer
    let mut rng = XorShiftRng::seed_from_u64(3141519u64);
    let a = <halo2curves::bls12381::Fr as halo2curves::ff::Field>::random(&mut rng);
    f2(a);

    let duration = start.elapsed(); // Calculate the elapsed time
    println!("Duration: {:?}", duration);


    let start = Instant::now(); // Start the timer
    let a: FF_Bls12_377Fr = rng.sample(Standard {});
    f1(a);
    let duration = start.elapsed(); // Calculate the elapsed time
    println!("Duration: {:?}", duration);
}

fn f2(a: halo2curves::bls12381::Fr) {
    let mut b = a.clone();
    for _ in 0..1000000i64 {
        b.pow(&[1 << 19u64]);
        b = (b + a) * a;
    }
}

fn f1(a: FF_Bls12_377Fr) {
    let mut b: FF_Bls12_377Fr = a.clone();
    for i in 0..1000000i64 {
        b = b.pow(&[1 << 19u64]);
        b = (b + a) * a;
    }
}


