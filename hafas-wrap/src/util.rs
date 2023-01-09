//! Misc functions

/// Encrypt a Base64-AES encoded String
/// 
/// This function uses the [`openssl`] library to encrypt a 
/// given String together with a statically set key.
#[allow(dead_code)]
#[must_use]
pub fn encode(secret: &str) -> String {
    const KEY: &[u8; 16] = b"36EA89767DA99804";

    use openssl::symm::{Cipher, Crypter, Mode};

    let plaintext: &[u8] = secret.as_bytes();
    let mut encrypter = Crypter::new(Cipher::aes_128_cbc(), Mode::Encrypt, KEY, None).unwrap();
    let mut ciphertext = vec![0; plaintext.len() + Cipher::aes_128_cbc().block_size()];

    let mut count = encrypter.update(plaintext, &mut ciphertext).unwrap();
    count += encrypter.finalize(&mut ciphertext[count..]).unwrap();
    ciphertext.truncate(count);

    base64::encode(&ciphertext)
}

/// Decrypt a String using md5
/// 
/// This function uses the [`openssl`] library to hash a 
/// given String using the `MD5` digesting algorithm.
#[must_use]
pub fn hash_md5(plain: &String) -> String {
    use openssl::hash::{hash, MessageDigest};
    let digest = &*hash(MessageDigest::md5(), plain.as_bytes()).unwrap();
    digest
        .iter()
        .map(|b| format!("{b:02x}"))
        .fold(String::with_capacity(digest.len() * 2), |s, h| s + &h)
}

/// Some errors that may be returned by [`decode`]
#[derive(thiserror::Error, Debug)]
pub enum AESDecodeError {
    #[error("Error occured whilst decrypting")]
    DecryptError(#[from] openssl::error::ErrorStack),
    #[error("Error occured whilst decoding")]
    DecodeError(#[from] base64::DecodeError),
    #[error("Error occured whilst parsing String")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

/// Decrypt a Base64-AES encoded String.
/// 
/// This function uses the [`openssl`] library to decrypt a 
/// given String together with a statically set key.
/// 
/// # Errors
/// - [`FromUtf8Error`](AESDecodeError::FromUtf8Error) when the string is not correctly utf-8 encoded.
/// - [`DecryptError`](AESDecodeError::DecryptError) when the string is wrongly encrypted.
/// - [`DecodeError`](AESDecodeError::DecodeError) when the string is wrongly base-64 encoded.
pub fn decode(secret: &str) -> Result<String, AESDecodeError> {
    const KEY: &[u8; 16] = b"36EA89767DA99804";

    use openssl::symm::{Cipher, Crypter, Mode};

    let ciphertexts: &[u8] = &base64::decode(secret)?;
    let mut decrypter = Crypter::new(Cipher::aes_128_cbc(), Mode::Decrypt, KEY, None).unwrap();
    let mut plaintext = vec![0; ciphertexts.len() + Cipher::aes_128_cbc().block_size()];

    let mut count = decrypter.update(ciphertexts, &mut plaintext)?;
    count += decrypter.finalize(&mut plaintext[count..])?;
    plaintext.truncate(count);

    Ok(String::from_utf8(plaintext)?)
}

#[cfg(test)]
mod tests {
    use crate::util::hash_md5;

    use super::*;

    #[test]
    fn test_aes() {
        let data = "Test string with äscìí chars that is longer than three blocks";
        let recoded = decode(&encode(data)).unwrap();
        assert_eq!(data, recoded);
    }

    #[test]
    fn hash() {
        let data = String::from("Test String to hash");
        let hash = hash_md5(&data);
        assert_eq!(String::from("8d0512f9ca44e9e0363c79af4b89caf1"), hash);
    }
}
