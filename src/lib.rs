use std::convert::TryInto;
use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Generate a pair of primes in the range `2**30..2**31`
/// suitable for RSA encryption with exponent
/// `EXP`. Warning: this routine has unbounded runtime; it
/// works by generate-and-test, generating pairs of primes
/// `p` `q` and testing that they satisfy `λ(pq) <= EXP` and
/// that `λ(pq)` has no common factors with `EXP`.
pub fn genkey() -> (u32, u32) {
    let mut p: u32 = rsa_prime();
    let mut q: u32 = rsa_prime();
    let mut lam: u64 = lambda(p, q);
    while !((EXP < lam) && (gcd(EXP, lam) == 1)) {
        p = rsa_prime();
        q = rsa_prime();
        lam = lambda(p, q);
    }
    (p, q)
}
/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg.into(), EXP, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d = modinverse(EXP, lambda(key.0, key.1));
    let key0: u64 = key.0.into();
    let key1: u64 = key.1.into();
    modexp(msg, d, key0 * key1).try_into().unwrap()
}

pub fn lambda(p: u32, q: u32) -> u64 {
    lcm((p - 1).into(), (q - 1).into())
}
