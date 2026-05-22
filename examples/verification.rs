
use std::ops::Neg;

use ark_bn254::{Bn254, Fr, G1Projective, G2Projective};
use ark_ec::{PrimeGroup, pairing::{Pairing, PairingOutput}};
use ark_ff::Zero;
fn main() {
let a  = Fr::from(4);
let b = Fr::from(3);
let c = Fr::from(6);
let d = Fr::from(2);
let neg_pa = G1Projective::generator()*a.neg();
let pb = G2Projective::generator()*b;
let pc = G1Projective::generator()*c;
let pd = G2Projective::generator()*d;

let ab = Bn254::pairing(neg_pa, pb);
let cd  = Bn254::pairing(pc, pd);
let tot:PairingOutput<ark_ec::bn::Bn<ark_bn254::Config>>  = PairingOutput(ab.0*cd.0);


assert!(tot.is_zero());
println!("{neg_pa}\n{pb}\n{pc}\n{pd}");




}