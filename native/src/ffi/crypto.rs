use aes::Aes256;
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Nonce,
};
use cbc::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit},
    Decryptor as CbcDecryptor, Encryptor as CbcEncryptor,
};
use ctr::{cipher::StreamCipher, Ctr128BE};
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use md5::Md5;
use rand_core::{OsRng, TryRngCore};
use sha2::{Digest, Sha256};
use std::slice;

type Aes256CbcDec = CbcDecryptor<Aes256>;
type Aes256CbcEnc = CbcEncryptor<Aes256>;
type Aes256Ctr = Ctr128BE<Aes256>;
type HmacSha256 = Hmac<Sha256>;

/// Error codes returned by all native bridge functions.
/// 0 = success; negative = error.
#[repr(i32)]
pub enum BridgeError {
    Ok = 0,
    NullPointer = -1,
    BadKeyLength = -2,
    BadIvLength = -3,
    EncryptionFailed = -4,
    DecryptionFailed = -5,
    OutputTooSmall = -6,
    HkdfFailed = -7,
    RngFailed = -8,
}

// ── internal helpers ─────────────────────────────────────────────────────────

/// SAFETY: caller must guarantee ptr is valid for `len` bytes.
#[inline]
unsafe fn in_slice<'a>(ptr: *const u8, len: usize) -> Option<&'a [u8]> {
    if ptr.is_null() || len == 0 {
        None
    } else {
        Some(unsafe { slice::from_raw_parts(ptr, len) })
    }
}

/// Write `src` into `out_buf[..src.len()]`.
/// Returns BridgeError::OutputTooSmall if *out_len < src.len().
/// On success writes actual length into *out_len.
///
/// SAFETY: caller must guarantee out_buf is valid for *out_len bytes.
#[inline]
pub(crate) unsafe fn write_out_pub(src: &[u8], out_buf: *mut u8, out_len: *mut usize) -> i32 {
    unsafe { write_out(src, out_buf, out_len) }
}

#[inline]
unsafe fn write_out(src: &[u8], out_buf: *mut u8, out_len: *mut usize) -> i32 {
    if out_buf.is_null() || out_len.is_null() {
        return BridgeError::NullPointer as i32;
    }
    let capacity = unsafe { *out_len };
    if capacity < src.len() {
        unsafe { *out_len = src.len() };
        return BridgeError::OutputTooSmall as i32;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), out_buf, src.len());
        *out_len = src.len();
    }
    BridgeError::Ok as i32
}

// ── MD5 ──────────────────────────────────────────────────────────────────────

/// Compute MD5 hash.
/// out_buf must be at least 16 bytes; *out_len is set to 16 on success.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_md5(
    data: *const u8,
    data_len: usize,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let Some(input) = (unsafe { in_slice(data, data_len) }) else {
        return BridgeError::NullPointer as i32;
    };
    use md5::Digest as _;
    let mut h = Md5::new();
    h.update(input);
    unsafe { write_out(&h.finalize(), out_buf, out_len) }
}

// ── SHA-256 ───────────────────────────────────────────────────────────────────

/// Compute SHA-256 hash.
/// out_buf must be at least 32 bytes; *out_len is set to 32 on success.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_sha256(
    data: *const u8,
    data_len: usize,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let Some(input) = (unsafe { in_slice(data, data_len) }) else {
        return BridgeError::NullPointer as i32;
    };
    let mut h = Sha256::new();
    h.update(input);
    unsafe { write_out(&h.finalize(), out_buf, out_len) }
}

// ── HMAC-SHA256 ───────────────────────────────────────────────────────────────

/// Compute HMAC-SHA256.
/// out_buf must be at least 32 bytes; *out_len is set to 32 on success.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_hmac_sha256(
    data: *const u8,
    data_len: usize,
    key: *const u8,
    key_len: usize,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let (Some(d), Some(k)) = (unsafe { in_slice(data, data_len) }, unsafe {
        in_slice(key, key_len)
    }) else {
        return BridgeError::NullPointer as i32;
    };
    let Ok(mut mac) = <HmacSha256 as Mac>::new_from_slice(k) else {
        return BridgeError::BadKeyLength as i32;
    };
    mac.update(d);
    unsafe { write_out(&mac.finalize().into_bytes(), out_buf, out_len) }
}

// ── HKDF ─────────────────────────────────────────────────────────────────────

/// HKDF-SHA256 expand.
/// salt_ptr / salt_len may be null/0 (no salt).
/// info_ptr / info_len may be null/0 (empty info).
/// out_buf must be at least out_len bytes; *out_len is unchanged on success.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_hkdf(
    ikm: *const u8,
    ikm_len: usize,
    salt: *const u8,
    salt_len: usize,
    info: *const u8,
    info_len: usize,
    out_buf: *mut u8,
    out_len: usize,
) -> i32 {
    let Some(ikm_slice) = (unsafe { in_slice(ikm, ikm_len) }) else {
        return BridgeError::NullPointer as i32;
    };
    let salt_slice = if salt.is_null() || salt_len == 0 {
        None
    } else {
        Some(unsafe { slice::from_raw_parts(salt, salt_len) })
    };
    let info_slice: &[u8] = if info.is_null() || info_len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(info, info_len) }
    };
    if out_buf.is_null() || out_len == 0 {
        return BridgeError::NullPointer as i32;
    }
    let hk = Hkdf::<Sha256>::new(salt_slice, ikm_slice);
    let out = unsafe { slice::from_raw_parts_mut(out_buf, out_len) };
    match hk.expand(info_slice, out) {
        Ok(()) => BridgeError::Ok as i32,
        Err(_) => BridgeError::HkdfFailed as i32,
    }
}

// ── AES-256-GCM ───────────────────────────────────────────────────────────────

/// Encrypt with AES-256-GCM. Auth tag (16 bytes) is appended to ciphertext.
/// key must be 32 bytes, iv must be 12 bytes.
/// *out_len must be >= plaintext_len + 16; set to actual output length on success.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_aes_encrypt_gcm(
    plaintext: *const u8,
    plaintext_len: usize,
    key: *const u8,
    iv: *const u8,
    aad: *const u8,
    aad_len: usize,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let (Some(pt), Some(k), Some(nonce_bytes)) = (
        unsafe { in_slice(plaintext, plaintext_len) },
        unsafe { in_slice(key, 32) },
        unsafe { in_slice(iv, 12) },
    ) else {
        return BridgeError::NullPointer as i32;
    };
    let aad_slice: &[u8] = if aad.is_null() || aad_len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(aad, aad_len) }
    };
    let Ok(cipher) = Aes256Gcm::new_from_slice(k) else {
        return BridgeError::BadKeyLength as i32;
    };
    let nonce = Nonce::from_slice(nonce_bytes);
    let payload = Payload {
        msg: pt,
        aad: aad_slice,
    };
    match cipher.encrypt(nonce, payload) {
        Ok(ct) => unsafe { write_out(&ct, out_buf, out_len) },
        Err(_) => BridgeError::EncryptionFailed as i32,
    }
}

/// Decrypt AES-256-GCM. Expects auth tag (16 bytes) appended to ciphertext.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_aes_decrypt_gcm(
    ciphertext: *const u8,
    ciphertext_len: usize,
    key: *const u8,
    iv: *const u8,
    aad: *const u8,
    aad_len: usize,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let (Some(ct), Some(k), Some(nonce_bytes)) = (
        unsafe { in_slice(ciphertext, ciphertext_len) },
        unsafe { in_slice(key, 32) },
        unsafe { in_slice(iv, 12) },
    ) else {
        return BridgeError::NullPointer as i32;
    };
    let aad_slice: &[u8] = if aad.is_null() || aad_len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(aad, aad_len) }
    };
    let Ok(cipher) = Aes256Gcm::new_from_slice(k) else {
        return BridgeError::BadKeyLength as i32;
    };
    let nonce = Nonce::from_slice(nonce_bytes);
    let payload = Payload {
        msg: ct,
        aad: aad_slice,
    };
    match cipher.decrypt(nonce, payload) {
        Ok(pt) => unsafe { write_out(&pt, out_buf, out_len) },
        Err(_) => BridgeError::DecryptionFailed as i32,
    }
}

// ── AES-256-CBC ───────────────────────────────────────────────────────────────

/// Encrypt AES-256-CBC with a random IV; IV is prepended to output.
/// key must be 32 bytes. *out_len must be >= plaintext_len + 32 (IV + padding).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_aes_encrypt_cbc(
    plaintext: *const u8,
    plaintext_len: usize,
    key: *const u8,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let (Some(pt), Some(k)) = (unsafe { in_slice(plaintext, plaintext_len) }, unsafe {
        in_slice(key, 32)
    }) else {
        return BridgeError::NullPointer as i32;
    };
    let mut iv = [0u8; 16];
    OsRng.try_fill_bytes(&mut iv).expect("OsRng failed");
    let Ok(enc) = Aes256CbcEnc::new_from_slices(k, &iv) else {
        return BridgeError::BadKeyLength as i32;
    };
    let ct = enc.encrypt_padded_vec_mut::<Pkcs7>(pt);
    let mut result = Vec::with_capacity(16 + ct.len());
    result.extend_from_slice(&iv);
    result.extend_from_slice(&ct);
    unsafe { write_out(&result, out_buf, out_len) }
}

/// Decrypt AES-256-CBC. IV is the first 16 bytes of ciphertext.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_aes_decrypt_cbc(
    ciphertext: *const u8,
    ciphertext_len: usize,
    key: *const u8,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    if ciphertext_len < 16 {
        return BridgeError::BadIvLength as i32;
    }
    let (Some(ct_full), Some(k)) = (unsafe { in_slice(ciphertext, ciphertext_len) }, unsafe {
        in_slice(key, 32)
    }) else {
        return BridgeError::NullPointer as i32;
    };
    let (iv, ct) = ct_full.split_at(16);
    let Ok(dec) = Aes256CbcDec::new_from_slices(k, iv) else {
        return BridgeError::BadKeyLength as i32;
    };
    match dec.decrypt_padded_vec_mut::<Pkcs7>(ct) {
        Ok(pt) => unsafe { write_out(&pt, out_buf, out_len) },
        Err(_) => BridgeError::DecryptionFailed as i32,
    }
}

/// Encrypt AES-256-CBC with explicit IV (no IV prefix in output).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_aes_encrypt_cbc_iv(
    plaintext: *const u8,
    plaintext_len: usize,
    key: *const u8,
    iv: *const u8,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let (Some(pt), Some(k), Some(iv_slice)) = (
        unsafe { in_slice(plaintext, plaintext_len) },
        unsafe { in_slice(key, 32) },
        unsafe { in_slice(iv, 16) },
    ) else {
        return BridgeError::NullPointer as i32;
    };
    let Ok(enc) = Aes256CbcEnc::new_from_slices(k, iv_slice) else {
        return BridgeError::BadKeyLength as i32;
    };
    let ct = enc.encrypt_padded_vec_mut::<Pkcs7>(pt);
    unsafe { write_out(&ct, out_buf, out_len) }
}

/// Decrypt AES-256-CBC with explicit IV.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_aes_decrypt_cbc_iv(
    ciphertext: *const u8,
    ciphertext_len: usize,
    key: *const u8,
    iv: *const u8,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let (Some(ct), Some(k), Some(iv_slice)) = (
        unsafe { in_slice(ciphertext, ciphertext_len) },
        unsafe { in_slice(key, 32) },
        unsafe { in_slice(iv, 16) },
    ) else {
        return BridgeError::NullPointer as i32;
    };
    let Ok(dec) = Aes256CbcDec::new_from_slices(k, iv_slice) else {
        return BridgeError::BadKeyLength as i32;
    };
    match dec.decrypt_padded_vec_mut::<Pkcs7>(ct) {
        Ok(pt) => unsafe { write_out(&pt, out_buf, out_len) },
        Err(_) => BridgeError::DecryptionFailed as i32,
    }
}

// ── AES-256-CTR ───────────────────────────────────────────────────────────────

/// AES-256-CTR encrypt/decrypt (symmetric — same function for both).
/// key must be 32 bytes, iv must be 16 bytes.
/// out_buf must be >= data_len bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_aes_ctr(
    data: *const u8,
    data_len: usize,
    key: *const u8,
    iv: *const u8,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let (Some(d), Some(k), Some(iv_slice)) = (
        unsafe { in_slice(data, data_len) },
        unsafe { in_slice(key, 32) },
        unsafe { in_slice(iv, 16) },
    ) else {
        return BridgeError::NullPointer as i32;
    };
    let Ok(mut cipher) = Aes256Ctr::new_from_slices(k, iv_slice) else {
        return BridgeError::BadKeyLength as i32;
    };
    let mut buf = d.to_vec();
    cipher.apply_keystream(&mut buf);
    unsafe { write_out(&buf, out_buf, out_len) }
}
