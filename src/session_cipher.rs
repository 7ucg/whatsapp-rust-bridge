use js_sys::{Object, Reflect, Uint8Array};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

use crate::{
    protocol_address::ProtocolAddress,
    storage_adapter::{JsStorageAdapter, SignalStorage},
};
use wacore_libsignal::protocol::{
    self as libsignal, PreKeySignalMessage, PreKeyStore, SessionStore, UsePQRatchet,
};

/// Extracts the sender's identity key from a PreKeySignalMessage for identity-change detection.
/// Returns `undefined` if parsing fails or the message is not a valid PreKeyMessage.
#[wasm_bindgen(js_name = getPreKeyMessageIdentityKey)]
pub fn get_prekey_message_identity_key(ciphertext: &[u8]) -> Option<Uint8Array> {
    if ciphertext.len() < 2 {
        return None;
    }
    // First byte is the version byte — skip it
    let msg = PreKeySignalMessage::try_from(&ciphertext[1..]).ok()?;
    let identity_key_bytes = msg.identity_key().public_key().serialize();
    if identity_key_bytes.len() == 33 {
        let arr = Uint8Array::new_with_length(33);
        arr.copy_from(&identity_key_bytes);
        Some(arr)
    } else {
        None
    }
}

#[inline]
fn bytes_to_uint8array(bytes: &[u8]) -> Uint8Array {
    let result = Uint8Array::new_with_length(bytes.len() as u32);
    result.copy_from(bytes);
    result
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "{ type: number; body: Uint8Array }")]
    pub type EncryptResult;
}

thread_local! {
    static TYPE_KEY: RefCell<JsValue> = RefCell::new(JsValue::from_str("type"));
    static BODY_KEY: RefCell<JsValue> = RefCell::new(JsValue::from_str("body"));
}

#[wasm_bindgen(js_name = SessionCipher)]
pub struct SessionCipher {
    storage_adapter: JsStorageAdapter,
    remote_address: ProtocolAddress,
}

#[wasm_bindgen(js_class = SessionCipher)]
impl SessionCipher {
    #[wasm_bindgen(constructor)]
    pub fn new(storage: SignalStorage, remote_address: &ProtocolAddress) -> Self {
        Self {
            storage_adapter: JsStorageAdapter::new(storage),
            remote_address: ProtocolAddress(remote_address.0.clone()),
        }
    }

    pub async fn encrypt(&mut self, plaintext: &[u8]) -> Result<EncryptResult, JsValue> {
        let mut session_store = self.storage_adapter.clone();
        let mut identity_store = session_store.clone();

        let ciphertext_message = libsignal::message_encrypt(
            plaintext,
            &self.remote_address.0,
            &mut session_store,
            &mut identity_store,
        )
        .await
        .map_err(|e| {
            let msg = format!("SessionCipher.encrypt error: {:?}", e);
            JsValue::from_str(&msg)
        })?;

        let body_array = bytes_to_uint8array(ciphertext_message.serialize());
        let type_id = ciphertext_message.message_type() as u8;

        let result = Object::new();
        TYPE_KEY.with(|k| Reflect::set(&result, &k.borrow(), &(type_id as u32).into()))?;
        BODY_KEY.with(|k| Reflect::set(&result, &k.borrow(), &body_array.into()))?;

        Ok(result.unchecked_into())
    }

    #[wasm_bindgen(js_name = decryptPreKeyWhisperMessage)]
    pub async fn decrypt_prekey_whisper_message(
        &mut self,
        ciphertext: &[u8],
    ) -> Result<Uint8Array, JsValue> {
        let prekey_message = libsignal::PreKeySignalMessage::try_from(ciphertext)
            .map_err(|e| {
                let msg = format!("SessionCipher.decryptPreKeyWhisperMessage failed: Invalid PreKeyMessage format: {}", e);
                JsValue::from_str(&msg)
            })?;

        let mut session_store = self.storage_adapter.clone();
        let mut identity_store = session_store.clone();
        let mut prekey_store = session_store.clone();
        let signed_prekey_store = session_store.clone();

        let result = libsignal::message_decrypt_prekey(
            &prekey_message,
            &self.remote_address.0,
            &mut session_store,
            &mut identity_store,
            &mut prekey_store,
            &signed_prekey_store,
            &mut rand::rng(),
            UsePQRatchet::No,
        )
        .await
        .map_err(|e| {
            let msg = format!("SessionCipher.decryptPreKeyWhisperMessage failed: {:?}", e);
            JsValue::from_str(&msg)
        })?;

        // The decrypt no longer deletes the consumed one-time prekey itself; the
        // caller must remove it now that the promoted session is stored.
        if let Some(prekey_id) = result.consumed_prekey_id {
            prekey_store.remove_pre_key(prekey_id).await.map_err(|e| {
                JsValue::from_str(&format!(
                    "SessionCipher.decryptPreKeyWhisperMessage: remove_pre_key failed: {:?}",
                    e
                ))
            })?;
        }

        Ok(bytes_to_uint8array(&result.plaintext))
    }

    #[wasm_bindgen(js_name = decryptWhisperMessage)]
    pub async fn decrypt_whisper_message(
        &mut self,
        ciphertext: &[u8],
    ) -> Result<Uint8Array, JsValue> {
        let signal_message = libsignal::SignalMessage::try_from(ciphertext).map_err(|e| {
            let msg = format!(
                "SessionCipher.decryptWhisperMessage failed: Invalid WhisperMessage format: {}",
                e
            );
            JsValue::from_str(&msg)
        })?;

        let mut session_store = self.storage_adapter.clone();
        let mut identity_store = session_store.clone();

        let result = libsignal::message_decrypt_signal(
            &signal_message,
            &self.remote_address.0,
            &mut session_store,
            &mut identity_store,
            &mut rand::rng(),
        )
        .await
        .map_err(|e| {
            let msg = format!("SessionCipher.decryptWhisperMessage failed: {:?}", e);
            JsValue::from_str(&msg)
        })?;

        Ok(bytes_to_uint8array(&result.plaintext))
    }

    #[wasm_bindgen(js_name = hasOpenSession)]
    pub async fn has_open_session(&self) -> Result<bool, JsValue> {
        let record = SessionStore::load_session(&self.storage_adapter, &self.remote_address.0)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        match record {
            Some(r) => Ok(r.session_state().is_some()),
            None => Ok(false),
        }
    }
}
