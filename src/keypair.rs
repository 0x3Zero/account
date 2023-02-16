#![allow(improper_ctypes)]

use marine_rs_sdk::marine;
use ed25519_compact::{KeyPair};
use std::ops::Deref;

#[marine]
#[derive(Debug)]
pub struct Ed25519KeyPair {
    pub pubkey: String,
    pub privkey: String,
}

#[marine]
pub fn generate_keypair() -> Ed25519KeyPair {
    let kp = KeyPair::generate();
    let base64_pk = base64::encode(kp.pk.deref());

    let base64_sk = base64::encode(kp.sk.deref());

    Ed25519KeyPair {
        pubkey: base64_pk,
        privkey: base64_sk,
    }
}