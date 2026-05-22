use ark_bn254::{Fr, G1Projective, G2Projective};
use ark_ec::PrimeGroup;
use ark_ff::{BigInteger, PrimeField, UniformRand};
use rand_core::OsRng;
fn main() {
// The two generators of the groups G1 and GG2.

let g1 = G1Projective::generator();
let g2  = G2Projective::generator();


// The prime order q of the two groups
let order = Fr::MODULUS; // This is a big int.
// We make the order into a field element.
//let order = Fr::new(order); // This is mathematically correct. As a field element `order = 0 (mod q)`
println!("The order of the groups {}",order);

let r = Fr::rand(&mut OsRng);
println!("A random element from the field {}",r);
// We encode r as a big int and we add the order.
let mut r_bigint = r.into_bigint();
r_bigint.add_with_carry(&order);

println!("r + order as a big int {}",r_bigint);

let x = g1*r;
let y = g1.mul_bigint(r_bigint);
assert_eq!(x,y);

let x = g2*r;
let y = g2.mul_bigint(r_bigint);
assert_eq!(x,y);

}