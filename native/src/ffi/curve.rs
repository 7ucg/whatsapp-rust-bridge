use curve25519_dalek::{constants::ED25519_BASEPOINT_TABLE, Scalar};
use std::slice;
use wacore_libsignal::core::curve::{
    KeyPair as CoreKeyPair, PrivateKey as CorePrivateKey, PublicKey as CorePublicKey,
};

use super::crypto::BridgeError;

// ── generateKeyPair ───────────────────────────────────────────────────────────

/// Generate a Curve25519 key pair.
/// pub_key_out must be 33 bytes (0x05 prefix + 32 bytes).
/// priv_key_out must be 32 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_generate_key_pair(pub_key_out: *mut u8, priv_key_out: *mut u8) -> i32 {
    if pub_key_out.is_null() || priv_key_out.is_null() {
        return BridgeError::NullPointer as i32;
    }
    let mut rng = rand::make_rng::<rand::rngs::StdRng>();
    let pair = CoreKeyPair::generate(&mut rng);
    let pub_bytes = pair.public_key.serialize();
    let priv_bytes = pair.private_key.serialize();
    unsafe {
        std::ptr::copy_nonoverlapping(pub_bytes.as_ptr(), pub_key_out, pub_bytes.len());
        std::ptr::copy_nonoverlapping(priv_bytes.as_ptr(), priv_key_out, priv_bytes.len());
    }
    BridgeError::Ok as i32
}

// ── calculateAgreement ────────────────────────────────────────────────────────

/// Diffie-Hellman shared secret.
/// pub_key: 32 or 33 bytes (with optional 0x05 prefix).
/// priv_key: 32 bytes.
/// out must be 32 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_calculate_agreement(
    pub_key: *const u8,
    pub_key_len: usize,
    priv_key: *const u8,
    out: *mut u8,
) -> i32 {
    if pub_key.is_null() || priv_key.is_null() || out.is_null() {
        return BridgeError::NullPointer as i32;
    }
    if pub_key_len != 32 && pub_key_len != 33 {
        return BridgeError::BadKeyLength as i32;
    }
    let pub_bytes = unsafe { slice::from_raw_parts(pub_key, pub_key_len) };
    let priv_bytes = unsafe { slice::from_raw_parts(priv_key, 32) };

    let priv_k = match CorePrivateKey::deserialize(priv_bytes) {
        Ok(k) => k,
        Err(_) => return BridgeError::BadKeyLength as i32,
    };
    let pub_k = match parse_pub_key(pub_bytes) {
        Some(k) => k,
        None => return BridgeError::BadKeyLength as i32,
    };
    let secret = match priv_k.calculate_agreement(&pub_k) {
        Ok(s) => s,
        Err(_) => return BridgeError::EncryptionFailed as i32,
    };
    unsafe { std::ptr::copy_nonoverlapping(secret.as_ref().as_ptr(), out, 32) };
    BridgeError::Ok as i32
}

// ── calculateSignature ────────────────────────────────────────────────────────

/// Sign message with Ed25519-flavored Curve25519 private key.
/// priv_key: 32 bytes. out must be 64 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_calculate_signature(
    priv_key: *const u8,
    message: *const u8,
    message_len: usize,
    out: *mut u8,
) -> i32 {
    if priv_key.is_null() || message.is_null() || out.is_null() {
        return BridgeError::NullPointer as i32;
    }
    let priv_bytes = unsafe { slice::from_raw_parts(priv_key, 32) };
    let msg = unsafe { slice::from_raw_parts(message, message_len) };
    let priv_k = match CorePrivateKey::deserialize(priv_bytes) {
        Ok(k) => k,
        Err(_) => return BridgeError::BadKeyLength as i32,
    };
    let mut rng = rand::make_rng::<rand::rngs::StdRng>();
    let sig = match priv_k.calculate_signature(msg, &mut rng) {
        Ok(s) => s,
        Err(_) => return BridgeError::EncryptionFailed as i32,
    };
    unsafe { std::ptr::copy_nonoverlapping(sig.as_ref().as_ptr(), out, 64) };
    BridgeError::Ok as i32
}

// ── verifySignature ───────────────────────────────────────────────────────────

/// Verify Ed25519-flavored Curve25519 signature.
/// pub_key: 32 or 33 bytes. signature: 64 bytes.
/// Returns 1 (valid), 0 (invalid), or negative error code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_verify_signature(
    pub_key: *const u8,
    pub_key_len: usize,
    message: *const u8,
    message_len: usize,
    signature: *const u8,
) -> i32 {
    if pub_key.is_null() || message.is_null() || signature.is_null() {
        return BridgeError::NullPointer as i32;
    }
    if pub_key_len != 32 && pub_key_len != 33 {
        return BridgeError::BadKeyLength as i32;
    }
    let pub_bytes = unsafe { slice::from_raw_parts(pub_key, pub_key_len) };
    let msg = unsafe { slice::from_raw_parts(message, message_len) };
    let sig_bytes = unsafe { slice::from_raw_parts(signature, 64) };
    let Ok(sig_arr): Result<&[u8; 64], _> = sig_bytes.try_into() else {
        return BridgeError::BadKeyLength as i32;
    };
    let pub_k = match parse_pub_key(pub_bytes) {
        Some(k) => k,
        None => return BridgeError::BadKeyLength as i32,
    };
    if pub_k.verify_signature_for_multipart_message(&[msg], sig_arr) {
        1
    } else {
        0
    }
}

// ── getPublicFromPrivateKey ────────────────────────────────────────────────────

/// Derive public key from private key.
/// priv_key: 32 bytes. out must be 33 bytes (0x05 prefix included).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_get_public_from_private(priv_key: *const u8, out: *mut u8) -> i32 {
    if priv_key.is_null() || out.is_null() {
        return BridgeError::NullPointer as i32;
    }
    let priv_bytes: &[u8; 32] = match unsafe { slice::from_raw_parts(priv_key, 32) }.try_into() {
        Ok(a) => a,
        Err(_) => return BridgeError::BadKeyLength as i32,
    };
    let mut clamped = *priv_bytes;
    clamped[0] &= 248;
    clamped[31] &= 127;
    clamped[31] |= 64;
    let scalar = Scalar::from_bytes_mod_order(clamped);
    let pub_point = &scalar * ED25519_BASEPOINT_TABLE;
    let pub_bytes = pub_point.compress().to_bytes();
    unsafe {
        *out = 0x05;
        std::ptr::copy_nonoverlapping(pub_bytes.as_ptr(), out.add(1), 32);
    }
    BridgeError::Ok as i32
}

// ── internal ──────────────────────────────────────────────────────────────────

fn parse_pub_key(bytes: &[u8]) -> Option<CorePublicKey> {
    match bytes.len() {
        33 if bytes[0] == 0x05 => CorePublicKey::deserialize(bytes).ok(),
        32 => {
            let mut with_prefix = [0u8; 33];
            with_prefix[0] = 0x05;
            with_prefix[1..].copy_from_slice(bytes);
            CorePublicKey::deserialize(&with_prefix).ok()
        }
        _ => None,
    }
}
