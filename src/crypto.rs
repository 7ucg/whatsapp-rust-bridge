use aes::Aes256;
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Nonce,
};
use cbc::{
    cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7},
    Decryptor as CbcDecryptor,
    Encryptor as CbcEncryptor,
};
use ctr::{Ctr128BE, cipher::StreamCipher};
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use js_sys::Uint8Array;
use md5::Md5;
use rand::{TryRngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

type Aes256CbcDec = CbcDecryptor<Aes256>;
type Aes256CbcEnc = CbcEncryptor<Aes256>;
type Aes256Ctr = Ctr128BE<Aes256>;
type HmacSha256 = Hmac<Sha256>;

const GCM_TAG_LENGTH: usize = 16;

#[inline]
fn to_uint8array(v: &[u8]) -> Uint8Array {
    let arr = Uint8Array::new_with_length(v.len() as u32);
    arr.copy_from(v);
    arr
}

// ── MD5 ──────────────────────────────────────────────────────────────────────

#[wasm_bindgen(js_name = md5)]
pub fn md5_hash(buffer: &[u8]) -> Uint8Array {
    use md5::Digest as _;
    let mut hasher = Md5::new();
    hasher.update(buffer);
    to_uint8array(&hasher.finalize())
}

// ── SHA-256 ───────────────────────────────────────────────────────────────────

#[wasm_bindgen(js_name = sha256)]
pub fn sha256(buffer: &[u8]) -> Uint8Array {
    let mut hasher = Sha256::new();
    hasher.update(buffer);
    to_uint8array(&hasher.finalize())
}

// ── HMAC-SHA256 ───────────────────────────────────────────────────────────────

#[wasm_bindgen(js_name = hmacSign)]
pub fn hmac_sign(buffer: &[u8], key: &[u8]) -> Result<Uint8Array, JsValue> {
    let mut mac = <HmacSha256 as Mac>::new_from_slice(key)
        .map_err(|_| JsValue::from_str("Invalid HMAC key length"))?;
    mac.update(buffer);
    Ok(to_uint8array(&mac.finalize().into_bytes()))
}

// ── HKDF ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Tsify, Default)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase", default)]
pub struct HkdfInfo {
    #[tsify(type = "Uint8Array | undefined")]
    #[serde(with = "serde_bytes")]
    pub salt: Option<Vec<u8>>,
    #[tsify(type = "Uint8Array | string | undefined")]
    #[serde(with = "hkdf_info_serde")]
    pub info: Option<Vec<u8>>,
}

mod hkdf_info_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(v: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error> {
        v.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<u8>>, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum InfoValue {
            Bytes(#[serde(with = "serde_bytes")] Vec<u8>),
            Text(String),
        }

        match Option::<InfoValue>::deserialize(d)? {
            None => Ok(None),
            Some(InfoValue::Bytes(b)) => Ok(Some(b)),
            Some(InfoValue::Text(s)) => Ok(Some(s.into_bytes())),
        }
    }
}

#[wasm_bindgen(js_name = hkdf)]
pub fn hkdf(buffer: &[u8], expanded_length: usize, info: HkdfInfo) -> Result<Uint8Array, JsValue> {
    let salt_bytes = info.salt.as_deref();
    let info_bytes = info.info.as_deref().unwrap_or(&[]);

    let hk = Hkdf::<Sha256>::new(salt_bytes, buffer);
    let mut okm = vec![0u8; expanded_length];

    hk.expand(info_bytes, &mut okm)
        .map_err(|_| JsValue::from_str("HKDF expansion failed"))?;

    Ok(to_uint8array(&okm))
}

// ── AES-256-GCM ───────────────────────────────────────────────────────────────

/// Encrypts with AES-256-GCM; auth tag (16 bytes) is appended to ciphertext.
#[wasm_bindgen(js_name = aesEncryptGCM)]
pub fn aes_encrypt_gcm(
    plaintext: &[u8],
    key: &[u8],
    iv: &[u8],
    additional_data: &[u8],
) -> Result<Uint8Array, JsValue> {
    if key.len() != 32 {
        return Err(JsValue::from_str("AES-GCM key must be 32 bytes"));
    }
    if iv.len() != 12 {
        return Err(JsValue::from_str("AES-GCM IV must be 12 bytes"));
    }
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let nonce = Nonce::from_slice(iv);
    let payload = Payload {
        msg: plaintext,
        aad: additional_data,
    };
    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|_| JsValue::from_str("AES-GCM encryption failed"))?;
    Ok(to_uint8array(&ciphertext))
}

/// Decrypts AES-256-GCM; expects auth tag (16 bytes) appended to ciphertext.
#[wasm_bindgen(js_name = aesDecryptGCM)]
pub fn aes_decrypt_gcm(
    ciphertext_with_tag: &[u8],
    key: &[u8],
    iv: &[u8],
    additional_data: &[u8],
) -> Result<Uint8Array, JsValue> {
    if key.len() != 32 {
        return Err(JsValue::from_str("AES-GCM key must be 32 bytes"));
    }
    if iv.len() != 12 {
        return Err(JsValue::from_str("AES-GCM IV must be 12 bytes"));
    }
    if ciphertext_with_tag.len() < GCM_TAG_LENGTH {
        return Err(JsValue::from_str("Ciphertext too short"));
    }
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let nonce = Nonce::from_slice(iv);
    let payload = Payload {
        msg: ciphertext_with_tag,
        aad: additional_data,
    };
    let plaintext = cipher
        .decrypt(nonce, payload)
        .map_err(|_| JsValue::from_str("AES-GCM decryption failed (bad auth tag?)"))?;
    Ok(to_uint8array(&plaintext))
}

// ── AES-256-CBC ───────────────────────────────────────────────────────────────

/// Decrypts AES-256-CBC; IV is the first 16 bytes of `buffer`.
#[wasm_bindgen(js_name = aesDecrypt)]
pub fn aes_decrypt(buffer: &[u8], key: &[u8]) -> Result<Uint8Array, JsValue> {
    if buffer.len() < 16 {
        return Err(JsValue::from_str("Buffer too short"));
    }
    let (iv, ciphertext) = buffer.split_at(16);
    aes_decrypt_cbc_iv(ciphertext, key, iv)
}

/// Decrypts AES-256-CBC with an explicit IV.
#[wasm_bindgen(js_name = aesDecryptWithIV)]
pub fn aes_decrypt_with_iv(buffer: &[u8], key: &[u8], iv: &[u8]) -> Result<Uint8Array, JsValue> {
    aes_decrypt_cbc_iv(buffer, key, iv)
}

fn aes_decrypt_cbc_iv(buffer: &[u8], key: &[u8], iv: &[u8]) -> Result<Uint8Array, JsValue> {
    if key.len() != 32 {
        return Err(JsValue::from_str("AES-CBC key must be 32 bytes"));
    }
    if iv.len() != 16 {
        return Err(JsValue::from_str("AES-CBC IV must be 16 bytes"));
    }
    let decryptor = Aes256CbcDec::new_from_slices(key, iv)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let plaintext = decryptor
        .decrypt_padded_vec_mut::<Pkcs7>(buffer)
        .map_err(|_| JsValue::from_str("AES-CBC decryption failed (bad padding?)"))?;
    Ok(to_uint8array(&plaintext))
}

/// Encrypts AES-256-CBC with a random IV; IV is prepended to output.
#[wasm_bindgen(js_name = aesEncrypt)]
pub fn aes_encrypt(buffer: &[u8], key: &[u8]) -> Result<Uint8Array, JsValue> {
    if key.len() != 32 {
        return Err(JsValue::from_str("AES-CBC key must be 32 bytes"));
    }
    let mut iv = [0u8; 16];
    OsRng
        .try_fill_bytes(&mut iv)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let ciphertext = Aes256CbcEnc::new_from_slices(key, &iv)
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .encrypt_padded_vec_mut::<Pkcs7>(buffer);
    let mut result = Vec::with_capacity(16 + ciphertext.len());
    result.extend_from_slice(&iv);
    result.extend_from_slice(&ciphertext);
    Ok(to_uint8array(&result))
}

/// Encrypts AES-256-CBC with a given IV (no IV prefix in output).
#[wasm_bindgen(js_name = aesEncrypWithIV)]
pub fn aes_encrypt_with_iv(buffer: &[u8], key: &[u8], iv: &[u8]) -> Result<Uint8Array, JsValue> {
    if key.len() != 32 {
        return Err(JsValue::from_str("AES-CBC key must be 32 bytes"));
    }
    if iv.len() != 16 {
        return Err(JsValue::from_str("AES-CBC IV must be 16 bytes"));
    }
    let ciphertext = Aes256CbcEnc::new_from_slices(key, iv)
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .encrypt_padded_vec_mut::<Pkcs7>(buffer);
    Ok(to_uint8array(&ciphertext))
}

// ── AES-256-CTR ───────────────────────────────────────────────────────────────

#[wasm_bindgen(js_name = aesEncryptCTR)]
pub fn aes_encrypt_ctr(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Uint8Array, JsValue> {
    aes_ctr(plaintext, key, iv)
}

#[wasm_bindgen(js_name = aesDecryptCTR)]
pub fn aes_decrypt_ctr(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Uint8Array, JsValue> {
    aes_ctr(ciphertext, key, iv)
}

fn aes_ctr(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Uint8Array, JsValue> {
    if key.len() != 32 {
        return Err(JsValue::from_str("AES-CTR key must be 32 bytes"));
    }
    if iv.len() != 16 {
        return Err(JsValue::from_str("AES-CTR IV must be 16 bytes"));
    }
    let mut cipher = Aes256Ctr::new_from_slices(key, iv)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut buf = data.to_vec();
    cipher.apply_keystream(&mut buf);
    Ok(to_uint8array(&buf))
}
