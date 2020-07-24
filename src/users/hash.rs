use data_encoding::HEXUPPER;

use ring::{digest, pbkdf2};

use std::env;
use std::num::NonZeroU32;

pub fn hash(message: &str) -> String {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let salt = env::var("hash_secret").expect("Hash Secret");

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt.as_bytes(),
        message.as_bytes(),
        &mut pbkdf2_hash,
    );

    HEXUPPER.encode(&pbkdf2_hash).to_owned()
}
