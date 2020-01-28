//! Module for toy RSA implementation.
//!
//! This code provides functions for 
//! RSA key generation, encryption and decryption.
use toy_rsa_lib::*;
use std::convert::{TryFrom, TryInto};

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

pub fn lambda(p: u64, q: u64) -> u64 {
    return lcm(p - 1, q - 1)
}

#[test]
fn test_lambda() {
    assert_eq!(180, lambda(37, 16));
}

/// Generate a pair of primes in the range `2**30..2**31`
/// suitable for RSA encryption with exponent
/// `EXP`. Warning: this routine has unbounded runtime; it
/// works by generate-and-test, generating pairs of primes
/// `p` `q` and testing that they satisfy `λ(pq) <= EXP` and
/// that `λ(pq)` has no common factors with `EXP`.
pub fn genkey() -> (u32, u32) {
    let mut p = 0_u64;
    let mut q = 0_u64;
    let mut done = false;

    while !done {
        p = u64::try_from(rsa_prime()).unwrap();
        q = u64::try_from(rsa_prime()).unwrap();
        let l = lambda(p, q);

        if EXP < l && gcd(EXP, l) == 1 {
            done = true;
        }
    }
    (p.try_into().unwrap(),q.try_into().unwrap())
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    return modexp(msg as u64, EXP, key)
}

#[test]
fn test_encrypt() {
    assert_eq!(0x164e44b86776d497, encrypt(0xde9c5816141c8ba9, 12345));
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let p = key.0 as u64;
    let q = key.1 as u64;
    let public_key = p * q;
    let l = lambda(p, q);
    let d = modinverse(EXP, l);
    let plain_msg = modexp(msg, d, public_key);
    u32::try_from(plain_msg).unwrap()
}

#[test]
fn test_decrypt() {
    assert_eq!(12345, decrypt((0xed23e6cd, 0xf050a04d), 0x164e44b86776d497));
}
