use serde::Serialize;
use tsify_next::Tsify;

/// A Kyber1024 public/secret key pair, serialized with a one-byte key-type prefix.
#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct KyberKeyPair {
    /// Serialized public key (key-type prefix byte + raw key bytes).
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    pub public_key: Vec<u8>,
    /// Serialized secret key (key-type prefix byte + raw key bytes).
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    pub secret_key: Vec<u8>,
}

/// A signed Kyber1024 pre-key (id + key pair + identity signature over the public key).
#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct KyberPreKey {
    pub key_id: u32,
    pub key_pair: KyberKeyPair,
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>,
}
