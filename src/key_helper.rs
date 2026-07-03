use js_sys::{TypeError, Uint8Array};
use rand::Rng;
use serde::Serialize;
use tsify_next::Tsify;
use wacore_libsignal::core::curve::{KeyPair as CoreKeyPair, PrivateKey as CorePrivateKey};
use wacore_libsignal::kem::{KeyPair as KemKeyPair, KeyType};
use wasm_bindgen::prelude::*;

pub use crate::curve::KeyPair;
use crate::kyber_prekey::{KyberKeyPair, KyberPreKey};

const PRIVATE_KEY_LENGTH: usize = 32;

#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SignedPreKey {
    pub key_id: u32,
    pub key_pair: KeyPair,
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct PreKey {
    pub key_id: u32,
    pub key_pair: KeyPair,
}

fn map_err(err: impl std::fmt::Display) -> JsValue {
    TypeError::new(&err.to_string()).into()
}

fn rng() -> impl rand::CryptoRng + rand::Rng {
    rand::rng()
}

fn core_key_pair_to_key_pair(pair: CoreKeyPair) -> KeyPair {
    KeyPair {
        pub_key: pair.public_key.serialize().to_vec(),
        priv_key: pair.private_key.serialize().to_vec(),
    }
}

#[wasm_bindgen(js_name = generateIdentityKeyPair)]
pub fn generate_identity_key_pair() -> KeyPair {
    crate::curve::generate_key_pair()
}

#[wasm_bindgen(js_name = generateRegistrationId)]
pub fn generate_registration_id() -> u32 {
    let mut bytes = [0u8; 2];
    rand::rng().fill_bytes(&mut bytes);
    (u16::from_le_bytes(bytes) & 0x3FFF) as u32
}

#[wasm_bindgen(js_name = generateSignedPreKey)]
pub fn generate_signed_pre_key(
    identity_key_pair: KeyPair,
    signed_key_id: u32,
) -> Result<SignedPreKey, JsValue> {
    if identity_key_pair.priv_key.len() != PRIVATE_KEY_LENGTH {
        return Err(TypeError::new("identityKeyPair.privKey must be 32 bytes").into());
    }

    let identity_private_key =
        CorePrivateKey::deserialize(&identity_key_pair.priv_key).map_err(map_err)?;

    let pre_key_pair = CoreKeyPair::generate(&mut rng());
    let pre_key_public_bytes = pre_key_pair.public_key.serialize();

    let signature = identity_private_key
        .calculate_signature(&pre_key_public_bytes, &mut rng())
        .map_err(map_err)?;

    Ok(SignedPreKey {
        key_id: signed_key_id,
        key_pair: core_key_pair_to_key_pair(pre_key_pair),
        signature: signature.to_vec(),
    })
}

#[wasm_bindgen(js_name = generatePreKey)]
pub fn generate_pre_key(key_id: u32) -> PreKey {
    PreKey {
        key_id,
        key_pair: core_key_pair_to_key_pair(CoreKeyPair::generate(&mut rng())),
    }
}

/// Generate a Kyber1024 pre-key signed by the given identity key.
///
/// The returned `KyberPreKey.keyPair.publicKey` and `.secretKey` are each prefixed
/// with the key-type byte `0x08` (Kyber1024) as produced by `kem::Key::serialize()`.
/// The `signature` covers the serialized public key using Ed25519 (Curve25519 private key).
#[wasm_bindgen(js_name = generateKyberPreKey)]
pub fn generate_kyber_pre_key(
    identity_key_pair: KeyPair,
    kyber_key_id: u32,
) -> Result<KyberPreKey, JsValue> {
    let identity_private_key =
        CorePrivateKey::deserialize(&identity_key_pair.priv_key).map_err(map_err)?;

    let kem_pair = KemKeyPair::generate(KeyType::Kyber1024, &mut rng());
    let pk_bytes = kem_pair.public_key.serialize().to_vec();

    let signature = identity_private_key
        .calculate_signature(&pk_bytes, &mut rng())
        .map_err(map_err)?;

    Ok(KyberPreKey {
        key_id: kyber_key_id,
        key_pair: KyberKeyPair {
            public_key: pk_bytes,
            secret_key: kem_pair.secret_key.serialize().to_vec(),
        },
        signature: signature.to_vec(),
    })
}

#[wasm_bindgen(js_name = _serializeIdentityKeyPair)]
pub fn _serialize_identity_key_pair(key_pair: KeyPair) -> Uint8Array {
    let pub_key_tag = (1 << 3) | 2;
    let priv_key_tag = (2 << 3) | 2;

    // Pre-allocate: 1 tag + 1 len + pub_key + 1 tag + 1 len + priv_key
    let capacity = 2 + key_pair.pub_key.len() + 2 + key_pair.priv_key.len();
    let mut buffer = Vec::with_capacity(capacity);

    buffer.push(pub_key_tag);
    buffer.push(key_pair.pub_key.len() as u8);
    buffer.extend_from_slice(&key_pair.pub_key);
    buffer.push(priv_key_tag);
    buffer.push(key_pair.priv_key.len() as u8);
    buffer.extend_from_slice(&key_pair.priv_key);

    Uint8Array::from(buffer.as_slice())
}
