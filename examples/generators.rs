use ark_bn254::{G1Projective, G2Projective};
use ark_ec::PrimeGroup;
fn main() {

let g1 = G1Projective::generator();
let g2 = G2Projective::generator();
println!("generator g1 {}",g1);
println!("generator g2 {}",g2);

}