use ark_bn254::{Bn254, Fr, G1Projective, G2Projective};
use ark_ec::{PrimeGroup, pairing::{Pairing}};
use ark_ff::{UniformRand};
use rand_core::OsRng;
fn main() {
    let a = Fr::rand(&mut OsRng);
    let b = Fr::rand(&mut OsRng);
    let c = a*b;

    let p = G1Projective::generator()*a;
    let q = G2Projective::generator()*b;

    let r = G1Projective::generator()*c;

    let pq = Bn254::pairing(p, q);
    let rq = Bn254::pairing(r, G2Projective::generator());

    assert_eq!(pq, rq);
}