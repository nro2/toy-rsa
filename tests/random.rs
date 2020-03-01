use rand::Rng;
use toy_rsa::*;
use toy_rsa_lib::*;
pub const EXP: u64 = 65_537;

#[test]
fn test_genkey() {
    let key = genkey();
    let lambda = lambda(key.0, key.1);
    assert_eq!(gcd(EXP, lambda), 1);
    assert!(EXP < lambda);
}

#[test]
fn test_rsa_fixed_message() {
    let gen = genkey();
    let testmess = 747364;
    let _p: u64 = gen.0.into();
    let _q: u64 = gen.1.into();
    let new_key: u64 = _p * _q;
    let encrypted = encrypt(new_key, testmess);
    let decrypted = decrypt(gen, encrypted);
    assert_eq!(decrypted, testmess, "did not match before and after")
}

#[test]
fn test_rsa_random_message() {
    let mut rng = rand::thread_rng();
    let max = u32::max_value();
    let min = max / 2;
    let gen = genkey();
    let testmess = rng.gen_range(min, max) | 1;
    let _p: u64 = gen.0.into();
    let _q: u64 = gen.1.into();
    let new_key: u64 = _p * _q;
    let encrypted = encrypt(new_key, testmess);
    let decrypted = decrypt(gen, encrypted);
    assert_eq!(decrypted, testmess, "did not match before and after")
}
