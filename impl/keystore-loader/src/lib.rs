// Terrible loader for Ethereum v3 keystores, as generated by MyEtherWallet.
// Created because both ethsign and OpenEthereum are GPLv3
// and nft_ptr is ostensibly a library that others can integrate

use secp256k1::SecretKey;
use serde::Deserialize;

#[derive(Deserialize)]
struct Keystore {
    crypto: KeystoreCrypto,
}

#[derive(Deserialize)]
struct KeystoreCrypto {
    ciphertext: String,
    cipher: String,
    cipherparams: KeystoreCipherParams,
    kdf: String,
    kdfparams: KeystoreKdfParams,
}

#[derive(Deserialize)]
struct KeystoreCipherParams {
    iv: String,
}

#[derive(Deserialize)]
struct KeystoreKdfParams {
    dklen: u32,
    salt: String,
    n: u32,
    r: u32,
    p: u32,
}

// Loads an account and its associated SecretKey from a keystore.
// Warning: this is not secure; do NOT ever load a wallet containing real money using this!!
pub fn load_keystore_from_string(
    input: &str,
    password: &str,
) -> Result<SecretKey, serde_json::Error> {
    let keystore: Keystore = serde_json::from_str(input)?;
    println!("{}", keystore.crypto.ciphertext);
    panic!("Unimplemented");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let key =
            load_keystore_from_string(include_str!("SampleKeystore.keystore"), "sample password")
                .unwrap();
        assert_eq!(2 + 2, 4);
    }
}
