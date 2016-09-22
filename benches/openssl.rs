#![feature(test)]

extern crate openssl;
extern crate test;

use test::Bencher;

use openssl::crypto::symm::{Crypter, encrypt};
use openssl::crypto::symm::Mode::Encrypt;
use openssl::crypto::symm::Type::DES_ECB;

#[bench]
fn bench_decrypt(bencher: &mut Bencher) {
    let key = [0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC, 0xDF, 0xF1];
    let message = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let mut crypter = Crypter::new(DES_ECB, Encrypt, &key, None).unwrap();
    crypter.pad(false);
    bencher.iter(|| {
        let mut result = vec![0; 16];
        crypter.update(&message, &mut result);
        crypter.finalize(&mut result).unwrap();
    });
}
