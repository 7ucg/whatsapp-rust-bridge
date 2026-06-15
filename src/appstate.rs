use js_sys::Uint8Array;
use prost::Message;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use wacore_appstate::{
    ExpandedAppStateKeys as RustExpandedAppStateKeys, LTHash, WAPATCH_INTEGRITY,
    collect_key_ids_from_patch_list, decode_record, encode_record, expand_app_state_keys,
};
use waproto::whatsapp as wa;

#[inline]
fn bytes_to_uint8array(bytes: &[u8]) -> Uint8Array {
    let arr = Uint8Array::new_with_length(bytes.len() as u32);
    arr.copy_from(bytes);
    arr
}

#[wasm_bindgen]
pub struct LTHashAntiTampering {
    inner: &'static LTHash,
}

#[wasm_bindgen]
impl LTHashAntiTampering {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: &WAPATCH_INTEGRITY,
        }
    }

    #[wasm_bindgen(js_name = subtractThenAdd)]
    pub fn subtract_then_add(
        &self,
        base: &[u8],
        subtract: Vec<Uint8Array>,
        add: Vec<Uint8Array>,
    ) -> Result<Uint8Array, JsValue> {
        if base.len() != 128 {
            return Err(JsValue::from_str(&format!(
                "Base hash must be 128 bytes, got {}",
                base.len()
            )));
        }

        // Pre-allocate with known capacity to avoid reallocations
        let mut subtract_vecs: Vec<Vec<u8>> = Vec::with_capacity(subtract.len());
        for arr in &subtract {
            let len = arr.length() as usize;
            let mut vec = vec![0u8; len];
            arr.copy_to(&mut vec);
            subtract_vecs.push(vec);
        }

        let mut add_vecs: Vec<Vec<u8>> = Vec::with_capacity(add.len());
        for arr in &add {
            let len = arr.length() as usize;
            let mut vec = vec![0u8; len];
            arr.copy_to(&mut vec);
            add_vecs.push(vec);
        }

        let result = self
            .inner
            .subtract_then_add(base, &subtract_vecs, &add_vecs);

        Ok(bytes_to_uint8array(&result))
    }
}

impl Default for LTHashAntiTampering {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ExpandedAppStateKeys {
    inner: RustExpandedAppStateKeys,
}

#[wasm_bindgen]
impl ExpandedAppStateKeys {
    #[wasm_bindgen(getter, js_name = indexKey)]
    pub fn index_key(&self) -> Uint8Array {
        bytes_to_uint8array(&self.inner.index)
    }

    #[wasm_bindgen(getter, js_name = valueEncryptionKey)]
    pub fn value_encryption_key(&self) -> Uint8Array {
        bytes_to_uint8array(&self.inner.value_encryption)
    }

    #[wasm_bindgen(getter, js_name = valueMacKey)]
    pub fn value_mac_key(&self) -> Uint8Array {
        bytes_to_uint8array(&self.inner.value_mac)
    }

    #[wasm_bindgen(getter, js_name = snapshotMacKey)]
    pub fn snapshot_mac_key(&self) -> Uint8Array {
        bytes_to_uint8array(&self.inner.snapshot_mac)
    }

    #[wasm_bindgen(getter, js_name = patchMacKey)]
    pub fn patch_mac_key(&self) -> Uint8Array {
        bytes_to_uint8array(&self.inner.patch_mac)
    }
}

#[wasm_bindgen(js_name = expandAppStateKeys)]
pub fn expand_app_state_keys_wasm(key_data: &[u8]) -> ExpandedAppStateKeys {
    let inner = expand_app_state_keys(key_data);
    ExpandedAppStateKeys { inner }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct LTHashState {
    version: u64,
    #[serde(with = "serde_bytes")]
    hash: Vec<u8>,
    #[serde(skip)]
    index_value_map: std::collections::HashMap<String, Vec<u8>>,
}

#[wasm_bindgen]
impl LTHashState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            version: 0,
            hash: vec![0u8; 128],
            index_value_map: std::collections::HashMap::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn version(&self) -> u64 {
        self.version
    }

    #[wasm_bindgen(setter)]
    pub fn set_version(&mut self, version: u64) {
        self.version = version;
    }

    #[wasm_bindgen(getter)]
    pub fn hash(&self) -> Uint8Array {
        bytes_to_uint8array(&self.hash)
    }

    #[wasm_bindgen(setter)]
    pub fn set_hash(&mut self, hash: Vec<u8>) {
        if hash.len() != 128 {
            wasm_bindgen::throw_str(&format!("Hash must be 128 bytes, got {}", hash.len()));
        }
        self.hash = hash;
    }

    #[wasm_bindgen(js_name = getValueMac)]
    pub fn get_value_mac(&self, index_mac_base64: &str) -> Option<Uint8Array> {
        self.index_value_map
            .get(index_mac_base64)
            .map(|v| bytes_to_uint8array(v))
    }

    #[wasm_bindgen(js_name = setValueMac)]
    pub fn set_value_mac(&mut self, index_mac_base64: &str, value_mac: Vec<u8>) {
        self.index_value_map
            .insert(index_mac_base64.to_string(), value_mac);
    }

    #[wasm_bindgen(js_name = deleteValueMac)]
    pub fn delete_value_mac(&mut self, index_mac_base64: &str) -> bool {
        self.index_value_map.remove(index_mac_base64).is_some()
    }

    #[wasm_bindgen(js_name = hasValueMac)]
    pub fn has_value_mac(&self, index_mac_base64: &str) -> bool {
        self.index_value_map.contains_key(index_mac_base64)
    }

    #[wasm_bindgen(js_name = clone)]
    pub fn clone_state(&self) -> LTHashState {
        self.clone()
    }
}

impl Default for LTHashState {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_key_length(key: &[u8], expected: usize, name: &str) -> Result<(), JsValue> {
    if key.len() != expected {
        return Err(JsValue::from_str(&format!(
            "{} must be {} bytes, got {}",
            name,
            expected,
            key.len()
        )));
    }
    Ok(())
}

fn create_mac(
    algo: &str,
    key: &[u8],
) -> Result<wacore_libsignal::crypto::CryptographicMac, JsValue> {
    wacore_libsignal::crypto::CryptographicMac::new(algo, key)
        .map_err(|e| JsValue::from_str(&format!("Failed to create MAC: {}", e)))
}

#[wasm_bindgen(js_name = generateContentMac)]
pub fn generate_content_mac(
    operation: u8,
    data: &[u8],
    key_id: &[u8],
    key: &[u8],
) -> Result<Uint8Array, JsValue> {
    validate_key_length(key, 32, "Value MAC key")?;

    let op_byte = [operation];
    let key_data_length = ((key_id.len() + 1) as u64).to_be_bytes();

    let mut mac = create_mac("HmacSha512", key)?;
    mac.update(&op_byte);
    mac.update(key_id);
    mac.update(data);
    mac.update(&key_data_length);
    let mac_full = mac.finalize();

    Ok(bytes_to_uint8array(&mac_full[..32]))
}

#[wasm_bindgen(js_name = generateSnapshotMac)]
pub fn generate_snapshot_mac(
    lt_hash: &[u8],
    version: u64,
    name: &str,
    key: &[u8],
) -> Result<Uint8Array, JsValue> {
    validate_key_length(lt_hash, 128, "LT-Hash")?;
    validate_key_length(key, 32, "Snapshot MAC key")?;

    let mut mac = create_mac("HmacSha256", key)?;
    mac.update(lt_hash);
    mac.update(&version.to_be_bytes());
    mac.update(name.as_bytes());

    Ok(bytes_to_uint8array(&mac.finalize()))
}

#[wasm_bindgen(js_name = generatePatchMac)]
pub fn generate_patch_mac(
    snapshot_mac: &[u8],
    value_macs: Vec<Uint8Array>,
    version: u64,
    name: &str,
    key: &[u8],
) -> Result<Uint8Array, JsValue> {
    validate_key_length(key, 32, "Patch MAC key")?;

    let mut mac = create_mac("HmacSha256", key)?;
    mac.update(snapshot_mac);

    // Use stack buffer for value MACs (typically 32 bytes) to avoid heap allocations
    let mut value_mac_buf = [0u8; 64];
    for value_mac in &value_macs {
        let len = value_mac.length() as usize;
        if len <= 64 {
            value_mac.copy_to(&mut value_mac_buf[..len]);
            mac.update(&value_mac_buf[..len]);
        } else {
            let mut vec = vec![0u8; len];
            value_mac.copy_to(&mut vec);
            mac.update(&vec);
        }
    }

    mac.update(&version.to_be_bytes());
    mac.update(name.as_bytes());

    Ok(bytes_to_uint8array(&mac.finalize()))
}

#[wasm_bindgen(js_name = generateIndexMac)]
pub fn generate_index_mac(index_bytes: &[u8], key: &[u8]) -> Result<Uint8Array, JsValue> {
    validate_key_length(key, 32, "Index key")?;

    let mut mac = create_mac("HmacSha256", key)?;
    mac.update(index_bytes);

    Ok(bytes_to_uint8array(&mac.finalize()))
}

// ── Decoded mutation result ───────────────────────────────────────────────────

/// A decoded app-state mutation — the result of decrypting one record.
#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct DecodedMutation {
    /// The operation type: 0 = SET, 1 = REMOVE
    pub operation: i32,
    /// The index components (JSON array path, e.g. ["contact","123@s.whatsapp.net"])
    pub index: Vec<String>,
    /// The index MAC bytes (base64 in JSON, Uint8Array in JS)
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    pub index_mac: Vec<u8>,
    /// The value MAC bytes
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    pub value_mac: Vec<u8>,
    /// The decrypted action value as protobuf-encoded bytes (SyncActionValue)
    #[tsify(type = "Uint8Array | undefined")]
    #[serde(with = "serde_bytes")]
    pub action_bytes: Vec<u8>,
}

// ── decodeRecord ─────────────────────────────────────────────────────────────

/// Decrypt and decode a single app-state record.
///
/// @param recordBytes   Protobuf-encoded `SyncdRecord` bytes
/// @param keys          Expanded app-state keys (from `expandAppStateKeys`)
/// @param keyId         The key ID bytes (used for MAC validation)
/// @param operation     0 = SET, 1 = REMOVE
/// @param validateMacs  Whether to verify MACs (set false to skip for speed)
#[wasm_bindgen(js_name = decodeAppStateRecord)]
pub fn decode_app_state_record(
    record_bytes: &[u8],
    keys: &ExpandedAppStateKeys,
    key_id: &[u8],
    operation: i32,
    validate_macs: bool,
) -> Result<DecodedMutation, JsValue> {
    let record = wa::SyncdRecord::decode(record_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to decode SyncdRecord: {}", e)))?;

    let op = wa::syncd_mutation::SyncdOperation::try_from(operation)
        .map_err(|_| JsValue::from_str(&format!("Invalid operation: {}", operation)))?;

    let (mutation, macs) = decode_record(op, &record, &keys.inner, key_id, validate_macs)
        .map_err(|e| JsValue::from_str(&format!("decode_record failed: {}", e)))?;

    let action_bytes = mutation
        .action_value
        .map(|v| {
            let mut buf = Vec::new();
            v.encode(&mut buf).unwrap_or(());
            buf
        })
        .unwrap_or_default();

    Ok(DecodedMutation {
        operation: op as i32,
        index: mutation.index,
        index_mac: macs.index_mac,
        value_mac: macs.value_mac,
        action_bytes,
    })
}

// ── collectKeyIds ─────────────────────────────────────────────────────────────

// ── encodeRecord ──────────────────────────────────────────────────────────────

/// Encoded mutation result returned by `encodeAppStateMutation`.
#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct EncodedMutation {
    /// Protobuf-encoded `SyncdMutation` bytes — add to your patch's mutations array.
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    pub mutation_bytes: Vec<u8>,
    /// The value MAC (32 bytes) — needed for `LTHashState.setValueMac`.
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    pub value_mac: Vec<u8>,
    /// The index MAC (from the encoded record) — needed for `LTHashState.setValueMac` key.
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    pub index_mac: Vec<u8>,
}

/// Encode and encrypt a mutation into a `SyncdMutation` (ready to include in a patch).
///
/// @param operation     0 = SET, 1 = REMOVE
/// @param indexBytes    The index as JSON bytes, e.g. `["contact","123@s.whatsapp.net"]`
/// @param actionBytes   Protobuf-encoded `SyncActionValue` bytes
/// @param keys          Expanded app-state keys (from `expandAppStateKeys`)
/// @param keyId         The key ID bytes
/// @param iv            16-byte IV for AES-CBC encryption (use random bytes)
/// @param version       Per-action schema version stamped into the mutation
///                      (mirrors WA Web; e.g. label_edit/label_jid = 3, 0 otherwise)
#[wasm_bindgen(js_name = encodeAppStateMutation)]
pub fn encode_app_state_mutation(
    operation: i32,
    index_bytes: &[u8],
    action_bytes: &[u8],
    keys: &ExpandedAppStateKeys,
    key_id: &[u8],
    iv: &[u8],
    version: i32,
) -> Result<EncodedMutation, JsValue> {
    let op = wa::syncd_mutation::SyncdOperation::try_from(operation)
        .map_err(|_| JsValue::from_str(&format!("Invalid operation: {}", operation)))?;

    let iv_arr: [u8; 16] = iv
        .try_into()
        .map_err(|_| JsValue::from_str("IV must be exactly 16 bytes"))?;

    let action = wa::SyncActionValue::decode(action_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to decode SyncActionValue: {}", e)))?;

    let (mutation, value_mac) =
        encode_record(op, index_bytes, &action, &keys.inner, key_id, &iv_arr, version);

    // Extract index_mac from the encoded record
    let index_mac = mutation
        .record
        .as_ref()
        .and_then(|r| r.index.as_ref())
        .and_then(|i| i.blob.clone())
        .unwrap_or_default();

    let mutation_bytes = mutation.encode_to_vec();

    Ok(EncodedMutation {
        mutation_bytes,
        value_mac: value_mac.to_vec(),
        index_mac,
    })
}

/// Extract all unique key IDs from a list of patches (and optionally a snapshot).
///
/// @param snapshotBytes  Optional protobuf-encoded `SyncdSnapshot` bytes (pass empty Uint8Array to skip)
/// @param patchesBytes   Array of protobuf-encoded `SyncdPatch` bytes
/// @returns              Array of key-ID byte arrays that need to be fetched
#[wasm_bindgen(js_name = collectAppStateKeyIds)]
pub fn collect_app_state_key_ids(
    snapshot_bytes: &[u8],
    patches_bytes: Vec<Uint8Array>,
) -> Result<js_sys::Array, JsValue> {
    let snapshot = if snapshot_bytes.is_empty() {
        None
    } else {
        Some(
            wa::SyncdSnapshot::decode(snapshot_bytes)
                .map_err(|e| JsValue::from_str(&format!("Failed to decode snapshot: {}", e)))?,
        )
    };

    let patches = patches_bytes
        .iter()
        .map(|arr| {
            let mut buf = vec![0u8; arr.length() as usize];
            arr.copy_to(&mut buf);
            wa::SyncdPatch::decode(buf.as_slice())
                .map_err(|e| JsValue::from_str(&format!("Failed to decode patch: {}", e)))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let key_ids = collect_key_ids_from_patch_list(snapshot.as_ref(), &patches);

    let result = js_sys::Array::new();
    for kid in key_ids {
        let arr = Uint8Array::from(kid.as_slice());
        result.push(&arr);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lt_hash_new() {
        let lt_hash = LTHashAntiTampering::new();
        assert_eq!(lt_hash.inner.hkdf_size, 128);
    }

    #[test]
    fn test_expand_app_state_keys() {
        let key = [7u8; 32];
        let expanded = expand_app_state_keys_wasm(&key);

        assert_eq!(expanded.index_key().length(), 32);
        assert_eq!(expanded.value_encryption_key().length(), 32);
        assert_eq!(expanded.value_mac_key().length(), 32);
        assert_eq!(expanded.snapshot_mac_key().length(), 32);
        assert_eq!(expanded.patch_mac_key().length(), 32);
    }

    #[test]
    fn test_lt_hash_state_new() {
        let state = LTHashState::new();
        assert_eq!(state.version(), 0);
        assert_eq!(state.hash.len(), 128);
    }
}
