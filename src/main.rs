/*
 * @Author: Mr.Car
 * @Date: 2025-07-08 18:10:31
 */
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{EncodePrivateKey, EncodePublicKey}};
use rsa::pkcs1v15::{SigningKey, VerifyingKey, Signature};
use rsa::signature::{RandomizedSigner, Verifier, SignatureEncoding};
use rand::thread_rng;
use sha2::Sha256;
use sha2::Digest;
use base64::{engine::general_purpose, Engine as _};
use std::time::Instant;
use std::convert::TryFrom;

// 生成RSA密钥对
fn generate_rsa_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = thread_rng();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("生成私钥失败");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

// POW: 找到一个nonce，使得sha256(nickname+nonce)前4位为0
fn find_nonce_with_pow(nickname: &str, zeros: usize) -> (u64, String) {
    let mut nonce = 0u64;
    let prefix = "0".repeat(zeros);
    let start = Instant::now();
    loop {
        let msg = format!("{}{}", nickname, nonce);
        let hash = Sha256::digest(msg.as_bytes());
        let hash_hex = hex::encode(&hash);
        if hash_hex.starts_with(&prefix) {
            println!("找到nonce: {}，耗时: {:?}", nonce, start.elapsed());
            return (nonce, hash_hex);
        }
        nonce += 1;
    }
}

// 用私钥签名
fn sign_with_private_key(private_key: &RsaPrivateKey, message: &str) -> String {
    let mut rng = thread_rng();
    let signing_key = SigningKey::<Sha256>::new_unprefixed(private_key.clone());
    let digest = Sha256::digest(message.as_bytes());
    let signature = signing_key.sign_with_rng(&mut rng, &digest);
    general_purpose::STANDARD.encode(signature.to_bytes())
}

// 用公钥验证签名
fn verify_with_public_key(public_key: &RsaPublicKey, message: &str, signature_b64: &str) -> bool {
    let verifying_key = VerifyingKey::<Sha256>::new_unprefixed(public_key.clone());
    let signature_bytes = match general_purpose::STANDARD.decode(signature_b64) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    let signature = match Signature::try_from(&signature_bytes[..]) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    let digest = Sha256::digest(message.as_bytes());
    verifying_key.verify(&digest, &signature).is_ok()
}

fn main() {
    // 1. 生成密钥对
    let (private_key, public_key) = generate_rsa_keypair();
    println!("私钥(PEM):\n{:?}", private_key.to_pkcs8_pem(Default::default()).unwrap());
    println!("公钥(PEM):\n{:?}", public_key.to_public_key_pem(Default::default()).unwrap());

    // 2. POW
    let nickname = "Mr.Care"; // 这里替换为你的昵称
    let (nonce, hash_hex) = find_nonce_with_pow(nickname, 4); // 4个前导0
    println!("POW结果: nonce = {}, hash = {}", nonce, hash_hex);

    // 3. 签名
    let message = format!("{}{}", nickname, nonce);
    let signature = sign_with_private_key(&private_key, &message);
    println!("签名(base64): {}", signature);

    // 4. 验证
    let is_valid = verify_with_public_key(&public_key, &message, &signature);
    println!("签名验证结果: {}", is_valid);
}
