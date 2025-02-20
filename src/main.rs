use ark_bls12_377::Bls12_377;
pub use ark_bls12_377::Fr as FF_Bls12_377Fr;
use ark_ff::{BigInteger, FftField, Field as OtherField, PrimeField, UniformRand, Zero};
use halo2curves::ff::{Field, PrimeField as OtherPrimeField};
use std::time::Instant;

type Fr = crrl::field::ModInt256<
    725501752471715841u64,
    6461107452199829505u64,
    6968279316240510977u64,
    1345280370688173398u64,
>;

fn main() {
    let start = Instant::now(); // Start the timer
                                //let mut rng = XorShiftRng::seed_from_u64(3141519u64);
    let a = halo2curves::bls12377::Fr::from_u128(330730498080212014249511436292689429673u128);
    f0(a);

    let duration = start.elapsed(); // Calculate the elapsed time
    println!("Halo2 Duration: {:?}", duration);

    let start = Instant::now(); // Start the timer
    let a: FF_Bls12_377Fr = FF_Bls12_377Fr::from(330730498080212014249511436292689429673u128);
    f1(a);
    let duration = start.elapsed(); // Calculate the elapsed time
    println!("Ark BLS Duration: {:?}", duration);

    let start = Instant::now(); // Start the timer
    let a: Fr = Fr::from_u128(330730498080212014249511436292689429673u128);
    f3(a);
    let duration = start.elapsed(); // Calculate the elapsed time

    println!("crrl Duration: {:?}", duration);
}

fn f0(a: halo2curves::bls12377::Fr) {
    let mut b = a.clone();
    for i in 0..1000000i64 {
        b = b * b;
        b = b * b;
        b = b * b;
        b = b * b;
        b = b * b;
        b = (b + a) * halo2curves::bls12377::Fr::from_u128(i as u128);
    }

    println!("{:?}", b.is_zero())
    //println!("{:?}", hex::encode(b.to_bytes().as_slice()))
}

fn f2(a: halo2curves::bls12381::Fr) {
    let mut b = a.clone();
    for i in 0..1000000i64 {
        b = b * b;
        b = b * b;
        b = b * b;
        b = b * b;
        b = b * b;
        b = (b + a) * halo2curves::bls12381::Fr::from_u128(i as u128);
    }

}

fn f1(a: FF_Bls12_377Fr) {
    let mut b: FF_Bls12_377Fr = a.clone();
    for i in 0..1_000_000i64 {
        b = b * b;
        b = b * b;
        b = b * b;
        b = b * b;
        b = b * b;
        b = (b + a) * FF_Bls12_377Fr::from(i);
    }
    println!("{:?}", b.is_zero())
    //println!("{:?}", hex::encode(b.into_bigint().to_bytes_le().as_slice()))
}

fn f3(a: Fr) {
    let mut b = a.clone();
    for i in 0..1_000_000i64 {
        b = b * b;
        b = b * b;
        b = b * b;
        b = b * b;
        b = b * b;
        b = (b + a) * Fr::from_i64(i);
    }

    println!("{:?}", b.iszero())
    //println!("{:?}", hex::encode(b.encode32()))
}
