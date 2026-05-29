use rand_core::{OsRng, TryRngCore};
use std::slice;
use wacore_libsignal::core::curve::{KeyPair as CoreKeyPair, PrivateKey as CorePrivateKey};

use super::crypto::BridgeError;

// ── generateRegistrationId ────────────────────────────────────────────────────

/// Generate a random 14-bit registration ID (0..16383).
/// Writes the u32 (little-endian) into out_id.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_generate_registration_id(out_id: *mut u32) -> i32 {
    if out_id.is_null() {
        return BridgeError::NullPointer as i32;
    }
    let mut bytes = [0u8; 2];
    OsRng.try_fill_bytes(&mut bytes).expect("OsRng failed");
    unsafe { *out_id = (u16::from_le_bytes(bytes) & 0x3FFF) as u32 };
    BridgeError::Ok as i32
}

// ── generatePreKey ────────────────────────────────────────────────────────────

/// Generate a pre-key pair for a given key_id.
/// pub_key_out: 33 bytes, priv_key_out: 32 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_generate_pre_key(
    key_id: u32,
    pub_key_out: *mut u8,
    priv_key_out: *mut u8,
    key_id_out: *mut u32,
) -> i32 {
    if pub_key_out.is_null() || priv_key_out.is_null() || key_id_out.is_null() {
        return BridgeError::NullPointer as i32;
    }
    let mut rng = rand::make_rng::<rand::rngs::StdRng>();
    let pair = CoreKeyPair::generate(&mut rng);
    let pub_bytes = pair.public_key.serialize();
    let priv_bytes = pair.private_key.serialize();
    unsafe {
        std::ptr::copy_nonoverlapping(pub_bytes.as_ptr(), pub_key_out, pub_bytes.len());
        std::ptr::copy_nonoverlapping(priv_bytes.as_ptr(), priv_key_out, priv_bytes.len());
        *key_id_out = key_id;
    }
    BridgeError::Ok as i32
}

// ── generateSignedPreKey ──────────────────────────────────────────────────────

/// Generate a signed pre-key.
/// identity_priv_key: 32-byte identity private key.
/// pub_key_out: 33 bytes, priv_key_out: 32 bytes, signature_out: 64 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_generate_signed_pre_key(
    identity_priv_key: *const u8,
    signed_key_id: u32,
    pub_key_out: *mut u8,
    priv_key_out: *mut u8,
    signature_out: *mut u8,
    key_id_out: *mut u32,
) -> i32 {
    if identity_priv_key.is_null()
        || pub_key_out.is_null()
        || priv_key_out.is_null()
        || signature_out.is_null()
        || key_id_out.is_null()
    {
        return BridgeError::NullPointer as i32;
    }
    let id_priv_bytes = unsafe { slice::from_raw_parts(identity_priv_key, 32) };
    let id_priv = match CorePrivateKey::deserialize(id_priv_bytes) {
        Ok(k) => k,
        Err(_) => return BridgeError::BadKeyLength as i32,
    };
    let mut rng = rand::make_rng::<rand::rngs::StdRng>();
    let pair = CoreKeyPair::generate(&mut rng);
    let pub_bytes = pair.public_key.serialize();
    let priv_bytes = pair.private_key.serialize();
    let sig = match id_priv.calculate_signature(&pub_bytes, &mut rng) {
        Ok(s) => s,
        Err(_) => return BridgeError::EncryptionFailed as i32,
    };
    unsafe {
        std::ptr::copy_nonoverlapping(pub_bytes.as_ptr(), pub_key_out, pub_bytes.len());
        std::ptr::copy_nonoverlapping(priv_bytes.as_ptr(), priv_key_out, priv_bytes.len());
        std::ptr::copy_nonoverlapping(sig.as_ref().as_ptr(), signature_out, 64);
        *key_id_out = signed_key_id;
    }
    BridgeError::Ok as i32
}
