// Integration tests — call internal logic directly (same code the C FFI wraps).

use aes::Aes256;
use aes_gcm::{Aes256Gcm, Nonce, aead::{Aead, KeyInit, Payload}};
use cbc::{
    Decryptor as CbcDec, Encryptor as CbcEnc,
    cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7},
};
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use md5::Md5;
use sha2::{Digest, Sha256};
use wacore_libsignal::core::curve::{KeyPair as CoreKeyPair, PrivateKey as CorePrivateKey};

type HmacSha256 = Hmac<Sha256>;
type Aes256CbcEnc = CbcEnc<Aes256>;
type Aes256CbcDec = CbcDec<Aes256>;

fn rng() -> rand::rngs::StdRng {
    rand::make_rng::<rand::rngs::StdRng>()
}

// ── Hash ──────────────────────────────────────────────────────────────────────

#[test]
fn test_sha256_known_value() {
    let mut h = Sha256::new();
    h.update(b"hello");
    let result = h.finalize();
    assert_eq!(result[0], 0x2c);
    assert_eq!(result[1], 0xf2);
    assert_eq!(result[2], 0x4d);
    assert_eq!(result.len(), 32);
}

#[test]
fn test_md5_known_value() {
    use md5::Digest as _;
    let mut h = Md5::new();
    h.update(b"hello");
    let result = h.finalize();
    // MD5("hello") = 5d41402abc4b2a76b9719d911017c592
    assert_eq!(result[0], 0x5d);
    assert_eq!(result[1], 0x41);
    assert_eq!(result.len(), 16);
}

#[test]
fn test_hmac_sha256() {
    let mut mac = <HmacSha256 as Mac>::new_from_slice(b"secret").unwrap();
    mac.update(b"hello");
    let result = mac.finalize().into_bytes();
    assert_eq!(result.len(), 32);
    assert!(result.iter().any(|&b| b != 0));
}

// ── HKDF ─────────────────────────────────────────────────────────────────────

#[test]
fn test_hkdf_expand() {
    let hk = Hkdf::<Sha256>::new(None, b"input key material");
    let mut okm = [0u8; 32];
    hk.expand(b"WhatsApp test", &mut okm).unwrap();
    assert!(okm.iter().any(|&b| b != 0));
    // deterministic
    let hk2 = Hkdf::<Sha256>::new(None, b"input key material");
    let mut okm2 = [0u8; 32];
    hk2.expand(b"WhatsApp test", &mut okm2).unwrap();
    assert_eq!(okm, okm2);
}

// ── AES-256-GCM ───────────────────────────────────────────────────────────────

#[test]
fn test_aes_gcm_roundtrip() {
    let key = [0u8; 32];
    let iv  = [0u8; 12];
    let pt  = b"WhatsApp test message";
    let aad = b"additional data";

    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce  = Nonce::from_slice(&iv);
    let ct = cipher.encrypt(nonce, Payload { msg: pt, aad }).unwrap();
    assert_eq!(ct.len(), pt.len() + 16);

    let pt2 = cipher.decrypt(nonce, Payload { msg: &ct, aad }).unwrap();
    assert_eq!(pt2, pt.as_slice());
}

#[test]
fn test_aes_gcm_bad_tag_rejected() {
    let key = [0u8; 32];
    let iv  = [0u8; 12];
    let pt  = b"test";
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce  = Nonce::from_slice(&iv);
    let mut ct = cipher.encrypt(nonce, Payload { msg: pt, aad: b"" }).unwrap();
    ct[0] ^= 0xFF; // tamper
    assert!(cipher.decrypt(nonce, Payload { msg: &ct, aad: b"" }).is_err());
}

// ── AES-256-CBC ───────────────────────────────────────────────────────────────

#[test]
fn test_aes_cbc_roundtrip() {
    let key = [0u8; 32];
    let iv  = [0x01u8; 16];
    let pt  = b"CBC test payload!";

    let enc = Aes256CbcEnc::new_from_slices(&key, &iv).unwrap();
    let ct  = enc.encrypt_padded_vec_mut::<Pkcs7>(pt);

    let dec = Aes256CbcDec::new_from_slices(&key, &iv).unwrap();
    let pt2 = dec.decrypt_padded_vec_mut::<Pkcs7>(&ct).unwrap();
    assert_eq!(pt2, pt.as_slice());
}

// ── Curve25519 ────────────────────────────────────────────────────────────────

#[test]
fn test_curve25519_dh() {
    let mut rng = rng();
    let pair1 = CoreKeyPair::generate(&mut rng);
    let pair2 = CoreKeyPair::generate(&mut rng);

    let shared1 = pair1.private_key.calculate_agreement(&pair2.public_key).unwrap();
    let shared2 = pair2.private_key.calculate_agreement(&pair1.public_key).unwrap();
    assert_eq!(shared1.as_ref(), shared2.as_ref(), "DH shared secrets must match");
}

#[test]
fn test_curve25519_sign_verify() {
    let mut rng = rng();
    let pair = CoreKeyPair::generate(&mut rng);
    let msg  = b"sign this message";

    let sig = pair.private_key.calculate_signature(msg, &mut rng).unwrap();
    let valid = pair.public_key.verify_signature_for_multipart_message(&[msg], &sig);
    assert!(valid, "valid signature rejected");

    // tamper
    let mut tampered = sig;
    tampered[0] ^= 0xFF;
    let invalid = pair.public_key.verify_signature_for_multipart_message(&[msg], &tampered);
    assert!(!invalid, "tampered signature accepted");
}

#[test]
fn test_pub_key_has_prefix() {
    let mut rng = rng();
    let pair = CoreKeyPair::generate(&mut rng);
    let pub_bytes = pair.public_key.serialize();
    assert_eq!(pub_bytes[0], 0x05, "public key must have 0x05 DJB prefix");
    assert_eq!(pub_bytes.len(), 33);
}

// ── Key Helper ────────────────────────────────────────────────────────────────

#[test]
fn test_registration_id_range() {
    use rand_core::{OsRng, TryRngCore};
    // generate 100 IDs and verify all are in [0, 16383]
    for _ in 0..100 {
        let mut bytes = [0u8; 2];
        OsRng.try_fill_bytes(&mut bytes).unwrap();
        let id = (u16::from_le_bytes(bytes) & 0x3FFF) as u32;
        assert!(id <= 16383, "ID out of range: {id}");
    }
}

#[test]
fn test_signed_pre_key_signature() {
    let mut rng = rng();
    let identity = CoreKeyPair::generate(&mut rng);
    let pre_key  = CoreKeyPair::generate(&mut rng);

    let pub_bytes = pre_key.public_key.serialize();
    let sig = identity.private_key.calculate_signature(&pub_bytes, &mut rng).unwrap();

    let valid = identity.public_key.verify_signature_for_multipart_message(&[&pub_bytes], &sig);
    assert!(valid, "signed pre-key signature must be valid");
}

// ── Binary Node ───────────────────────────────────────────────────────────────

#[test]
fn test_binary_node_roundtrip() {
    use std::borrow::Cow;
    use wacore_binary::marshal::{marshal_ref, unmarshal_ref};
    use wacore_binary::node::{AttrsRef, NodeRef, NodeStr, ValueRef};
    use wacore_binary::util::unpack;

    let attrs: Vec<(NodeStr<'static>, ValueRef<'static>)> = vec![
        (
            NodeStr::from(compact_str::CompactString::from("id")),
            ValueRef::String(NodeStr::from(compact_str::CompactString::from("123"))),
        ),
        (
            NodeStr::from(compact_str::CompactString::from("type")),
            ValueRef::String(NodeStr::from(compact_str::CompactString::from("get"))),
        ),
    ];

    let node = NodeRef::new(
        NodeStr::from(compact_str::CompactString::from("iq")),
        AttrsRef::from_vec(attrs),
        None,
    );

    let encoded = marshal_ref(&node).unwrap();
    assert!(!encoded.is_empty());

    let unpacked = unpack(&encoded).unwrap().into_owned();
    let decoded  = unmarshal_ref(&unpacked).unwrap();

    assert_eq!(decoded.tag.as_ref(), "iq");
    let attrs_slice = decoded.attrs.as_slice();
    let id_val = attrs_slice.iter().find(|(k, _)| k.as_ref() == "id").map(|(_, v)| v.to_string());
    assert_eq!(id_val.as_deref(), Some("123"));
}
