use curve25519_dalek::{Scalar, constants::ED25519_BASEPOINT_TABLE};
use jni::JNIEnv;
use jni::objects::{JByteArray, JClass};
use jni::sys::{jbyteArray, jint};
use wacore_libsignal::core::curve::{
    KeyPair as CoreKeyPair, PrivateKey as CorePrivateKey, PublicKey as CorePublicKey,
};

use super::crypto::{bytes_from_jarray, bytes_to_jarray, throw};

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

// ── generateKeyPair ───────────────────────────────────────────────────────────

/// Returns a 65-byte array: [33 bytes pubKey | 32 bytes privKey]
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Curve_generateKeyPair(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
) -> jbyteArray {
    let mut rng = rand::make_rng::<rand::rngs::StdRng>();
    let pair = CoreKeyPair::generate(&mut rng);
    let pub_bytes = pair.public_key.serialize();
    let priv_bytes = pair.private_key.serialize();
    let pub_slice: &[u8] = pub_bytes.as_ref();
    let priv_slice: &[u8] = priv_bytes.as_ref();
    let mut combined = Vec::with_capacity(pub_slice.len() + priv_slice.len());
    combined.extend_from_slice(pub_slice);
    combined.extend_from_slice(priv_slice);
    bytes_to_jarray(&mut env, &combined)
}

// ── calculateAgreement ────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Curve_calculateAgreement(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    pub_key: JByteArray<'_>,
    priv_key: JByteArray<'_>,
) -> jbyteArray {
    let pub_bytes = bytes_from_jarray(&mut env, pub_key);
    let priv_bytes = bytes_from_jarray(&mut env, priv_key);
    let Some(pub_k) = parse_pub_key(&pub_bytes) else {
        return throw(&mut env, "Invalid public key");
    };
    let Ok(priv_k) = CorePrivateKey::deserialize(&priv_bytes) else {
        return throw(&mut env, "Invalid private key");
    };
    match priv_k.calculate_agreement(&pub_k) {
        Ok(secret) => bytes_to_jarray(&mut env, secret.as_ref()),
        Err(e) => throw(&mut env, &e.to_string()),
    }
}

// ── calculateSignature ────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Curve_calculateSignature(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    priv_key: JByteArray<'_>,
    message: JByteArray<'_>,
) -> jbyteArray {
    let priv_bytes = bytes_from_jarray(&mut env, priv_key);
    let msg = bytes_from_jarray(&mut env, message);
    let Ok(priv_k) = CorePrivateKey::deserialize(&priv_bytes) else {
        return throw(&mut env, "Invalid private key");
    };
    let mut rng = rand::make_rng::<rand::rngs::StdRng>();
    match priv_k.calculate_signature(&msg, &mut rng) {
        Ok(sig) => bytes_to_jarray(&mut env, sig.as_ref()),
        Err(e) => throw(&mut env, &e.to_string()),
    }
}

// ── verifySignature ───────────────────────────────────────────────────────────

/// Returns 1 if valid, 0 if invalid, throws on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Curve_verifySignature(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    pub_key: JByteArray<'_>,
    message: JByteArray<'_>,
    signature: JByteArray<'_>,
) -> jint {
    let pub_bytes = bytes_from_jarray(&mut env, pub_key);
    let msg = bytes_from_jarray(&mut env, message);
    let sig_bytes = bytes_from_jarray(&mut env, signature);
    if sig_bytes.len() != 64 {
        let _ = env.throw_new("java/lang/IllegalArgumentException", "Signature must be 64 bytes");
        return -1;
    }
    let sig_arr: &[u8; 64] = sig_bytes.as_slice().try_into().unwrap();
    let Some(pub_k) = parse_pub_key(&pub_bytes) else {
        let _ = env.throw_new("java/lang/IllegalArgumentException", "Invalid public key");
        return -1;
    };
    if pub_k.verify_signature_for_multipart_message(&[&msg], sig_arr) { 1 } else { 0 }
}

// ── getPublicFromPrivateKey ────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Curve_getPublicFromPrivate(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    priv_key: JByteArray<'_>,
) -> jbyteArray {
    let priv_bytes = bytes_from_jarray(&mut env, priv_key);
    let Ok(arr): Result<[u8; 32], _> = priv_bytes.as_slice().try_into() else {
        return throw(&mut env, "Private key must be 32 bytes");
    };
    let mut clamped = arr;
    clamped[0] &= 248;
    clamped[31] &= 127;
    clamped[31] |= 64;
    let scalar = Scalar::from_bytes_mod_order(clamped);
    let pub_point = &scalar * ED25519_BASEPOINT_TABLE;
    let pub_bytes = pub_point.compress().to_bytes();
    let mut result = [0u8; 33];
    result[0] = 0x05;
    result[1..].copy_from_slice(&pub_bytes);
    bytes_to_jarray(&mut env, &result)
}
