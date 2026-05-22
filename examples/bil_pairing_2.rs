use ark_bn254::{Bn254, Fr, G1Projective, G2Projective};
use ark_ec::{PrimeGroup, pairing::{Pairing, PairingOutput}};

fn main() {
    let a = Fr::from(2);
    let b = Fr::from(3);

    let p = G1Projective::generator()*a;
    let q = G2Projective::generator()*b;
    
    let pq = Bn254::pairing(p, q);

    let c = Fr::from(4);
    let d = Fr::from(5);

    let r = G1Projective::generator()*c;
    let s = G2Projective::generator()*d;
    
    let rs= Bn254::pairing(r, s);
    

    let pqrs= PairingOutput(pq.0*rs.0);


    let e = Fr::from(13);
    let f = Fr::from(2);

    let u = G1Projective::generator()*e;
    let v  = G2Projective::generator()*f;

    let uv = Bn254::pairing(u, v);


    assert_eq!(pqrs, uv);
}