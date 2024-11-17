use std::{fs, io::{Read, Write}, path::Path, vec};
use crate::{read_input, TextSignFormat};
use anyhow::Result;
use base64::{prelude::BASE64_URL_SAFE_NO_PAD as URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use super::process_genpass;

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where 
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}


pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = read_input(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };

    let signed  = URL_SAFE_NO_PAD.encode(&signed);
    
    Ok(signed)
}

pub fn process_text_verify(input: &str, key: &str, format: TextSignFormat, sig: &str) -> Result<bool> {
    let mut reader = read_input(input)?;

    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
    };

    Ok(verified)
}

pub fn process_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

pub fn process_encrypt(input: &str, key: &str, output: &str) -> Result<()> {
    let mut reader = read_input(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let key = fs::read(key)?;
    let key = chacha20poly1305::Key::from_slice(&key);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, buf.as_ref()).map_err(|e| anyhow::anyhow!(e))?;

    let encoded_nonce = URL_SAFE_NO_PAD.encode(&nonce);
    let encoded_ciphertext = URL_SAFE_NO_PAD.encode(&ciphertext);

    match output {
        "-" => {
            println!("nonce: {}", encoded_nonce);
            println!("ciphertext: {}", encoded_ciphertext);
        }
        _ => {
            let mut file = fs::File::create(output)?;
            writeln!(file, "nonce: {}", encoded_nonce)?;
            writeln!(file, "ciphertext: {}", encoded_ciphertext)?;
        }
    }
    Ok(())
}

pub fn process_decrypt(input: &str, key: &str, output: &str) -> Result<()> {
    let mut reader = read_input(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let key = fs::read(key)?;
    let key = chacha20poly1305::Key::from_slice(&key);
    let cipher = ChaCha20Poly1305::new(key);
    let mut lines = buf.lines();
    let nonce = URL_SAFE_NO_PAD.decode(lines.next().unwrap().strip_prefix("nonce: ").unwrap())?;
    let ciphertext = URL_SAFE_NO_PAD.decode(lines.next().unwrap().strip_prefix("ciphertext: ").unwrap())?;
    let nonce = chacha20poly1305::Nonce::from_slice(&nonce);
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).map_err(|e| anyhow::anyhow!(e))?;

    match output {
        "-" => {
            println!("plaintext: {}", String::from_utf8_lossy(&plaintext));
        }
        _ => {
            let mut file = fs::File::create(output)?;
            file.write_all(&plaintext)?;
        }
    }
    Ok(())
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    } 
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        let signing_key = signing_key.to_bytes().to_vec();
        Ok(vec![signing_key, public_key])
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;
        let data = b"hello world";
        let sig = blake3.sign(&mut &data[..]).unwrap();
        println!("sig: {}", URL_SAFE_NO_PAD.encode(&sig));
        assert!(blake3.verify(&mut &data[..], &sig).unwrap());
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let signing_key = Ed25519Signer::load("fixtures/signing_key")?;
        let verifying_key = Ed25519Verifier::load("fixtures/public_key")?;
        let data = b"hello world";
        let sig = signing_key.sign(&mut &data[..])?;
        println!("sig: {}", URL_SAFE_NO_PAD.encode(&sig));
        assert!(verifying_key.verify(&mut &data[..], &sig)?);
        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt() -> Result<()> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        let key = chacha20poly1305::Key::from_slice(&key);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let data = b"hello world";
        let ciphertext = cipher.encrypt(&nonce, data.as_ref()).unwrap();
        let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
        assert_eq!(data.as_ref(), plaintext.as_slice());
        Ok(())
    }
}