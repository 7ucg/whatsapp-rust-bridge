use jni::JNIEnv;
use jni::objects::JByteArray;
use jni::sys::{jbyteArray, jint};
use rand_core::{OsRng, TryRngCore};
use wacore_libsignal::core::curve::{KeyPair as CoreKeyPair, PrivateKey as CorePrivateKey};

use super::crypto::{bytes_from_jarray, bytes_to_jarray, throw};

// ── generateRegistrationId ────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_KeyHelper_generateRegistrationId(
    _env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
) -> jint {
    let mut bytes = [0u8; 2];
    OsRng.try_fill_bytes(&mut bytes).expect("OsRng failed");
    (u16::from_le_bytes(bytes) & 0x3FFF) as jint
}

// ── generatePreKey ────────────────────────────────────────────────────────────

/// Returns 65 bytes: [33 pubKey | 32 privKey]
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_KeyHelper_generatePreKey(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    _key_id: jint,
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

// ── generateSignedPreKey ──────────────────────────────────────────────────────

/// Returns 129 bytes: [33 pubKey | 32 privKey | 64 signature]
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_KeyHelper_generateSignedPreKey(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    identity_priv_key: JByteArray<'_>,
    _signed_key_id: jint,
) -> jbyteArray {
    let id_priv_bytes = bytes_from_jarray(&mut env, identity_priv_key);
    let Ok(id_priv) = CorePrivateKey::deserialize(&id_priv_bytes) else {
        return throw(&mut env, "Invalid identity private key");
    };
    let mut rng = rand::make_rng::<rand::rngs::StdRng>();
    let pair = CoreKeyPair::generate(&mut rng);
    let pub_bytes = pair.public_key.serialize();
    let priv_bytes = pair.private_key.serialize();
    let pub_slice: &[u8] = pub_bytes.as_ref();
    let priv_slice: &[u8] = priv_bytes.as_ref();
    let Ok(sig) = id_priv.calculate_signature(pub_slice, &mut rng) else {
        return throw(&mut env, "Signature failed");
    };
    let mut combined = Vec::with_capacity(pub_slice.len() + priv_slice.len() + 64);
    combined.extend_from_slice(pub_slice);
    combined.extend_from_slice(priv_slice);
    combined.extend_from_slice(sig.as_ref());
    bytes_to_jarray(&mut env, &combined)
}
