//! JNI bindings — mirrors com.whatsapp.bridge.Crypto
//!
//! All methods map to the Java class:
//!   package com.whatsapp.bridge;
//!   public class Crypto { ... }
//!
//! Java method signature example:
//!   public static native byte[] md5(byte[] data);

use jni::JNIEnv;
use jni::objects::JByteArray;
use jni::sys::{jbyteArray, jint, jsize};

use aes::Aes256;
use aes_gcm::{Aes256Gcm, Nonce, aead::{Aead, KeyInit, Payload}};
use cbc::{
    Decryptor as CbcDecryptor, Encryptor as CbcEncryptor,
    cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7},
};
use ctr::{Ctr128BE, cipher::StreamCipher};
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use md5::Md5;
use rand_core::{OsRng, TryRngCore};
use sha2::{Digest, Sha256};

type Aes256CbcDec = CbcDecryptor<Aes256>;
type Aes256CbcEnc = CbcEncryptor<Aes256>;
type Aes256Ctr = Ctr128BE<Aes256>;
type HmacSha256 = Hmac<Sha256>;

// ── helpers ───────────────────────────────────────────────────────────────────

pub(super) fn bytes_from_jarray(env: &mut JNIEnv<'_>, arr: JByteArray<'_>) -> Vec<u8> {
    let len = env.get_array_length(&arr).unwrap_or(0);
    if len == 0 {
        return vec![];
    }
    let mut buf = vec![0i8; len as usize];
    env.get_byte_array_region(&arr, 0, &mut buf).unwrap();
    buf.into_iter().map(|b| b as u8).collect()
}

pub(super) fn bytes_to_jarray<'a>(env: &mut JNIEnv<'a>, data: &[u8]) -> jbyteArray {
    let arr = env
        .new_byte_array(data.len() as jsize)
        .expect("new_byte_array failed");
    let signed: Vec<i8> = data.iter().map(|&b| b as i8).collect();
    env.set_byte_array_region(&arr, 0, &signed)
        .expect("set_byte_array_region failed");
    arr.into_raw()
}

pub(super) fn throw(env: &mut JNIEnv<'_>, msg: &str) -> jbyteArray {
    let _ = env.throw_new("java/lang/RuntimeException", msg);
    std::ptr::null_mut()
}


// ── MD5 ──────────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_md5(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    data: JByteArray<'_>,
) -> jbyteArray {
    use md5::Digest as _;
    let input = bytes_from_jarray(&mut env, data);
    let mut h = Md5::new();
    h.update(&input);
    bytes_to_jarray(&mut env, &h.finalize())
}

// ── SHA-256 ───────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_sha256(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    data: JByteArray<'_>,
) -> jbyteArray {
    let input = bytes_from_jarray(&mut env, data);
    let mut h = Sha256::new();
    h.update(&input);
    bytes_to_jarray(&mut env, &h.finalize())
}

// ── HMAC-SHA256 ───────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_hmacSha256(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    data: JByteArray<'_>,
    key: JByteArray<'_>,
) -> jbyteArray {
    let d = bytes_from_jarray(&mut env, data);
    let k = bytes_from_jarray(&mut env, key);
    let Ok(mut mac) = <HmacSha256 as Mac>::new_from_slice(&k) else {
        return throw(&mut env, "Invalid HMAC key length");
    };
    mac.update(&d);
    bytes_to_jarray(&mut env, &mac.finalize().into_bytes())
}

// ── HKDF ─────────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_hkdf(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    ikm: JByteArray<'_>,
    salt: JByteArray<'_>,
    info: JByteArray<'_>,
    out_len: jint,
) -> jbyteArray {
    let ikm_bytes = bytes_from_jarray(&mut env, ikm);
    let salt_bytes = bytes_from_jarray(&mut env, salt);
    let info_bytes = bytes_from_jarray(&mut env, info);
    let salt_opt: Option<&[u8]> = if salt_bytes.is_empty() {
        None
    } else {
        Some(&salt_bytes)
    };
    let hk = Hkdf::<Sha256>::new(salt_opt, &ikm_bytes);
    let mut okm = vec![0u8; out_len as usize];
    if hk.expand(&info_bytes, &mut okm).is_err() {
        return throw(&mut env, "HKDF expansion failed");
    }
    bytes_to_jarray(&mut env, &okm)
}

// ── AES-256-GCM ───────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_aesEncryptGcm(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    plaintext: JByteArray<'_>,
    key: JByteArray<'_>,
    iv: JByteArray<'_>,
    aad: JByteArray<'_>,
) -> jbyteArray {
    let pt = bytes_from_jarray(&mut env, plaintext);
    let k = bytes_from_jarray(&mut env, key);
    let nonce_bytes = bytes_from_jarray(&mut env, iv);
    let aad_bytes = bytes_from_jarray(&mut env, aad);
    if k.len() != 32 {
        return throw(&mut env, "AES-GCM key must be 32 bytes");
    }
    if nonce_bytes.len() != 12 {
        return throw(&mut env, "AES-GCM IV must be 12 bytes");
    }
    let Ok(cipher) = Aes256Gcm::new_from_slice(&k) else {
        return throw(&mut env, "AES-GCM init failed");
    };
    let nonce = Nonce::from_slice(&nonce_bytes);
    let payload = Payload { msg: &pt, aad: &aad_bytes };
    match cipher.encrypt(nonce, payload) {
        Ok(ct) => bytes_to_jarray(&mut env, &ct),
        Err(_) => throw(&mut env, "AES-GCM encryption failed"),
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_aesDecryptGcm(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    ciphertext: JByteArray<'_>,
    key: JByteArray<'_>,
    iv: JByteArray<'_>,
    aad: JByteArray<'_>,
) -> jbyteArray {
    let ct = bytes_from_jarray(&mut env, ciphertext);
    let k = bytes_from_jarray(&mut env, key);
    let nonce_bytes = bytes_from_jarray(&mut env, iv);
    let aad_bytes = bytes_from_jarray(&mut env, aad);
    if k.len() != 32 {
        return throw(&mut env, "AES-GCM key must be 32 bytes");
    }
    if nonce_bytes.len() != 12 {
        return throw(&mut env, "AES-GCM IV must be 12 bytes");
    }
    let Ok(cipher) = Aes256Gcm::new_from_slice(&k) else {
        return throw(&mut env, "AES-GCM init failed");
    };
    let nonce = Nonce::from_slice(&nonce_bytes);
    let payload = Payload { msg: &ct, aad: &aad_bytes };
    match cipher.decrypt(nonce, payload) {
        Ok(pt) => bytes_to_jarray(&mut env, &pt),
        Err(_) => throw(&mut env, "AES-GCM decryption failed (bad auth tag?)"),
    }
}

// ── AES-256-CBC ───────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_aesEncryptCbc(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    plaintext: JByteArray<'_>,
    key: JByteArray<'_>,
) -> jbyteArray {
    let pt = bytes_from_jarray(&mut env, plaintext);
    let k = bytes_from_jarray(&mut env, key);
    if k.len() != 32 {
        return throw(&mut env, "AES-CBC key must be 32 bytes");
    }
    let mut iv = [0u8; 16];
    OsRng.try_fill_bytes(&mut iv).expect("OsRng failed");
    let Ok(enc) = Aes256CbcEnc::new_from_slices(&k, &iv) else {
        return throw(&mut env, "AES-CBC init failed");
    };
    let ct = enc.encrypt_padded_vec_mut::<Pkcs7>(&pt);
    let mut result = Vec::with_capacity(16 + ct.len());
    result.extend_from_slice(&iv);
    result.extend_from_slice(&ct);
    bytes_to_jarray(&mut env, &result)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_aesDecryptCbc(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    ciphertext: JByteArray<'_>,
    key: JByteArray<'_>,
) -> jbyteArray {
    let ct_full = bytes_from_jarray(&mut env, ciphertext);
    let k = bytes_from_jarray(&mut env, key);
    if k.len() != 32 {
        return throw(&mut env, "AES-CBC key must be 32 bytes");
    }
    if ct_full.len() < 16 {
        return throw(&mut env, "Ciphertext too short");
    }
    let (iv, ct) = ct_full.split_at(16);
    let Ok(dec) = Aes256CbcDec::new_from_slices(&k, iv) else {
        return throw(&mut env, "AES-CBC init failed");
    };
    match dec.decrypt_padded_vec_mut::<Pkcs7>(ct) {
        Ok(pt) => bytes_to_jarray(&mut env, &pt),
        Err(_) => throw(&mut env, "AES-CBC decryption failed (bad padding?)"),
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_aesEncryptCbcWithIv(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    plaintext: JByteArray<'_>,
    key: JByteArray<'_>,
    iv: JByteArray<'_>,
) -> jbyteArray {
    let pt = bytes_from_jarray(&mut env, plaintext);
    let k = bytes_from_jarray(&mut env, key);
    let iv_bytes = bytes_from_jarray(&mut env, iv);
    if k.len() != 32 {
        return throw(&mut env, "AES-CBC key must be 32 bytes");
    }
    if iv_bytes.len() != 16 {
        return throw(&mut env, "AES-CBC IV must be 16 bytes");
    }
    let Ok(enc) = Aes256CbcEnc::new_from_slices(&k, &iv_bytes) else {
        return throw(&mut env, "AES-CBC init failed");
    };
    bytes_to_jarray(&mut env, &enc.encrypt_padded_vec_mut::<Pkcs7>(&pt))
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_aesDecryptCbcWithIv(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    ciphertext: JByteArray<'_>,
    key: JByteArray<'_>,
    iv: JByteArray<'_>,
) -> jbyteArray {
    let ct = bytes_from_jarray(&mut env, ciphertext);
    let k = bytes_from_jarray(&mut env, key);
    let iv_bytes = bytes_from_jarray(&mut env, iv);
    if k.len() != 32 {
        return throw(&mut env, "AES-CBC key must be 32 bytes");
    }
    if iv_bytes.len() != 16 {
        return throw(&mut env, "AES-CBC IV must be 16 bytes");
    }
    let Ok(dec) = Aes256CbcDec::new_from_slices(&k, &iv_bytes) else {
        return throw(&mut env, "AES-CBC init failed");
    };
    match dec.decrypt_padded_vec_mut::<Pkcs7>(&ct) {
        Ok(pt) => bytes_to_jarray(&mut env, &pt),
        Err(_) => throw(&mut env, "AES-CBC decryption failed"),
    }
}

// ── AES-256-CTR ───────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Crypto_aesCtr(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    data: JByteArray<'_>,
    key: JByteArray<'_>,
    iv: JByteArray<'_>,
) -> jbyteArray {
    let d = bytes_from_jarray(&mut env, data);
    let k = bytes_from_jarray(&mut env, key);
    let iv_bytes = bytes_from_jarray(&mut env, iv);
    if k.len() != 32 {
        return throw(&mut env, "AES-CTR key must be 32 bytes");
    }
    if iv_bytes.len() != 16 {
        return throw(&mut env, "AES-CTR IV must be 16 bytes");
    }
    let Ok(mut cipher) = Aes256Ctr::new_from_slices(&k, &iv_bytes) else {
        return throw(&mut env, "AES-CTR init failed");
    };
    let mut buf = d;
    cipher.apply_keystream(&mut buf);
    bytes_to_jarray(&mut env, &buf)
}
