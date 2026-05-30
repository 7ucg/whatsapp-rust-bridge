use js_sys::Uint8Array;
use wacore_binary::consts::NOISE_PATTERN_XX as NOISE_MODE;
use wacore_binary::marshal::marshal_ref;
use wacore_noise::framing::{FrameDecoder, encode_frame_into};
use wacore_noise::{IkFallbackInputs, IkHandshakeState, IkServerHelloOutcome, XxFallbackHandshakeState};
use wacore_noise::{NoiseCipher, NoiseHandshake, build_handshake_header};
use wacore_libsignal::core::curve::KeyPair as CoreKeyPair;
use wasm_bindgen::prelude::*;

use crate::binary::{EncodingNode, decode_node, js_to_node_ref};

/// NoiseSession implements the Noise_XX_25519_AESGCM_SHA256 protocol pattern
/// with combined binary encoding/decoding operations for reduced WASM boundary crossings.
#[wasm_bindgen]
pub struct NoiseSession {
    handshake: Option<NoiseHandshake>,
    enc_cipher: Option<NoiseCipher>,
    dec_cipher: Option<NoiseCipher>,
    read_counter: u32,
    write_counter: u32,
    is_finished: bool,
    intro_header: Option<Vec<u8>>,
    frame_decoder: FrameDecoder,
    encode_scratch: Vec<u8>,
}

#[wasm_bindgen]
impl NoiseSession {
    #[wasm_bindgen(constructor)]
    pub fn new(
        public_key: &[u8],
        noise_header: &[u8],
        routing_info: Option<Vec<u8>>,
    ) -> Result<NoiseSession, JsValue> {
        let mut handshake = NoiseHandshake::new(NOISE_MODE, noise_header)
            .map_err(|e| JsValue::from_str(&format!("NoiseHandshake init failed: {}", e)))?;

        handshake.authenticate(public_key);

        let (intro_header, _) = build_handshake_header(routing_info.as_deref());

        Ok(NoiseSession {
            handshake: Some(handshake),
            enc_cipher: None,
            dec_cipher: None,
            read_counter: 0,
            write_counter: 0,
            is_finished: false,
            intro_header: Some(intro_header),
            frame_decoder: FrameDecoder::new(),
            encode_scratch: Vec::with_capacity(4096),
        })
    }

    /// Updates the session hash with the given data (no-op after handshake).
    pub fn authenticate(&mut self, data: &[u8]) {
        if let Some(ref mut handshake) = self.handshake {
            handshake.authenticate(data);
        }
    }

    #[inline]
    fn encrypt_vec(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, JsValue> {
        if self.is_finished {
            let cipher = self
                .enc_cipher
                .as_ref()
                .ok_or_else(|| JsValue::from_str("Encryption cipher not initialized"))?;
            let counter = self.write_counter;
            self.write_counter += 1;
            cipher
                .encrypt_with_counter(counter, plaintext)
                .map_err(|e| JsValue::from_str(&format!("Encryption failed: {}", e)))
        } else {
            self.handshake
                .as_mut()
                .ok_or_else(|| JsValue::from_str("Handshake not initialized"))?
                .encrypt(plaintext)
                .map_err(|e| JsValue::from_str(&format!("Encryption failed: {}", e)))
        }
    }

    #[inline]
    fn decrypt_vec(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
        if self.is_finished {
            let cipher = self
                .dec_cipher
                .as_ref()
                .ok_or_else(|| JsValue::from_str("Decryption cipher not initialized"))?;
            let counter = self.read_counter;
            self.read_counter += 1;
            let mut buf = ciphertext.to_vec();
            cipher
                .decrypt_in_place_with_counter(counter, &mut buf)
                .map(|_| buf)
                .map_err(|e| JsValue::from_str(&format!("Decryption failed: {}", e)))
        } else {
            self.handshake
                .as_mut()
                .ok_or_else(|| JsValue::from_str("Handshake not initialized"))?
                .decrypt(ciphertext)
                .map_err(|e| JsValue::from_str(&format!("Decryption failed: {}", e)))
        }
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Uint8Array, JsValue> {
        let ciphertext = self.encrypt_vec(plaintext)?;
        let result = Uint8Array::new_with_length(ciphertext.len() as u32);
        result.copy_from(&ciphertext);
        Ok(result)
    }

    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Uint8Array, JsValue> {
        let plaintext = self.decrypt_vec(ciphertext)?;
        let result = Uint8Array::new_with_length(plaintext.len() as u32);
        result.copy_from(&plaintext);
        Ok(result)
    }

    #[wasm_bindgen(js_name = mixIntoKey)]
    pub fn mix_into_key(&mut self, data: &[u8]) -> Result<(), JsValue> {
        let handshake = self
            .handshake
            .as_mut()
            .ok_or_else(|| JsValue::from_str("NoiseHandshake not initialized"))?;
        handshake
            .mix_into_key(data)
            .map_err(|e| JsValue::from_str(&format!("mixIntoKey failed: {}", e)))?;
        Ok(())
    }

    #[wasm_bindgen(js_name = finishInit)]
    pub fn finish_init(&mut self) -> Result<(), JsValue> {
        let handshake = self
            .handshake
            .take()
            .ok_or_else(|| JsValue::from_str("NoiseHandshake not initialized"))?;

        let (write_cipher, read_cipher) = handshake
            .finish()
            .map_err(|e| JsValue::from_str(&format!("finishInit failed: {}", e)))?;

        self.enc_cipher = Some(write_cipher);
        self.dec_cipher = Some(read_cipher);
        self.read_counter = 0;
        self.write_counter = 0;
        self.is_finished = true;
        Ok(())
    }

    #[wasm_bindgen(getter, js_name = isFinished)]
    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    #[wasm_bindgen(js_name = encodeFrameRaw)]
    pub fn encode_frame_raw(&mut self, data: &[u8]) -> Result<Uint8Array, JsValue> {
        let encrypted = if self.is_finished {
            self.encrypt_vec(data)?
        } else {
            data.to_vec()
        };

        let header = self.intro_header.take();
        encode_frame_into(&encrypted, header.as_deref(), &mut self.encode_scratch)
            .map_err(|e| JsValue::from_str(&format!("Frame encoding failed: {}", e)))?;

        let result = Uint8Array::new_with_length(self.encode_scratch.len() as u32);
        result.copy_from(&self.encode_scratch);
        Ok(result)
    }

    #[wasm_bindgen(js_name = encodeFrame)]
    pub fn encode_frame(&mut self, node: EncodingNode) -> Result<Uint8Array, JsValue> {
        let node_ref = js_to_node_ref(&node)?;
        let encoded_bytes = marshal_ref(&node_ref)
            .map_err(|e| JsValue::from_str(&format!("Marshal error: {}", e)))?;

        let encrypted = if self.is_finished {
            self.encrypt_vec(&encoded_bytes)?
        } else {
            encoded_bytes
        };

        let header = self.intro_header.take();
        encode_frame_into(&encrypted, header.as_deref(), &mut self.encode_scratch)
            .map_err(|e| JsValue::from_str(&format!("Frame encoding failed: {}", e)))?;

        let result = Uint8Array::new_with_length(self.encode_scratch.len() as u32);
        result.copy_from(&self.encode_scratch);
        Ok(result)
    }

    #[wasm_bindgen(js_name = decodeFrame)]
    pub fn decode_frame(&mut self, new_data: &[u8]) -> Result<js_sys::Array, JsValue> {
        self.frame_decoder.feed(new_data);
        let decoded_frames = js_sys::Array::new();

        while let Some(frame_data) = self.frame_decoder.decode_frame() {
            if self.is_finished {
                let decrypted_bytes = self.decrypt_vec(&frame_data)?;
                let node = decode_node(decrypted_bytes)?;
                decoded_frames.push(&node.into());
            } else {
                let result = Uint8Array::new_with_length(frame_data.len() as u32);
                result.copy_from(&frame_data);
                decoded_frames.push(&result.into());
            }
        }

        Ok(decoded_frames)
    }

    #[wasm_bindgen(getter, js_name = bufferedBytes)]
    pub fn buffered_bytes(&self) -> usize {
        self.frame_decoder.buffered_len()
    }

    #[wasm_bindgen(js_name = clearBuffer)]
    pub fn clear_buffer(&mut self) {
        self.frame_decoder.clear();
    }

    #[wasm_bindgen(js_name = getHash)]
    pub fn get_hash(&self) -> Uint8Array {
        if let Some(ref handshake) = self.handshake {
            let hash = handshake.hash();
            let result = Uint8Array::new_with_length(hash.len() as u32);
            result.copy_from(hash);
            result
        } else {
            Uint8Array::new_with_length(0)
        }
    }

    #[wasm_bindgen(js_name = processHandshakeInit)]
    pub fn process_handshake_init(
        &mut self,
        server_ephemeral: &[u8],
        server_static_encrypted: &[u8],
        server_payload_encrypted: &[u8],
        private_key: &[u8],
    ) -> Result<Uint8Array, JsValue> {
        let handshake = self
            .handshake
            .as_mut()
            .ok_or_else(|| JsValue::from_str("NoiseHandshake not initialized"))?;

        handshake.authenticate(server_ephemeral);

        handshake
            .mix_shared_secret(private_key, server_ephemeral)
            .map_err(|e| JsValue::from_str(&format!("mix_shared_secret failed: {}", e)))?;

        let dec_static = handshake
            .decrypt(server_static_encrypted)
            .map_err(|e| JsValue::from_str(&format!("decrypt static failed: {}", e)))?;

        handshake
            .mix_shared_secret(private_key, &dec_static)
            .map_err(|e| JsValue::from_str(&format!("mix_shared_secret failed: {}", e)))?;

        let cert_payload = handshake
            .decrypt(server_payload_encrypted)
            .map_err(|e| JsValue::from_str(&format!("decrypt payload failed: {}", e)))?;

        let result = Uint8Array::new_with_length(cert_payload.len() as u32);
        result.copy_from(&cert_payload);
        Ok(result)
    }

    #[wasm_bindgen(js_name = processHandshakeFinish)]
    pub fn process_handshake_finish(
        &mut self,
        noise_public_key: &[u8],
        noise_private_key: &[u8],
        server_ephemeral: &[u8],
    ) -> Result<Uint8Array, JsValue> {
        let handshake = self
            .handshake
            .as_mut()
            .ok_or_else(|| JsValue::from_str("NoiseHandshake not initialized"))?;

        let encrypted_key = handshake
            .encrypt(noise_public_key)
            .map_err(|e| JsValue::from_str(&format!("encrypt failed: {}", e)))?;

        handshake
            .mix_shared_secret(noise_private_key, server_ephemeral)
            .map_err(|e| JsValue::from_str(&format!("mix_shared_secret failed: {}", e)))?;

        let result = Uint8Array::new_with_length(encrypted_key.len() as u32);
        result.copy_from(&encrypted_key);
        Ok(result)
    }
}

// ── NoiseIkSession ────────────────────────────────────────────────────────────

/// Noise_IK_25519_AESGCM_SHA256 handshake — faster reconnect using a cached
/// server static key. Falls back to XX automatically if the server rejects.
///
/// Usage:
///   const ik = new NoiseIkSession(staticPub32, staticPriv32, serverStaticPub32, payload, prologue);
///   const clientHello = ik.buildClientHello();
///   // send clientHello framed over the wire, then:
///   const result = ik.readServerHello(serverHelloBytes);
///   if (result.fallback) {
///     // use result.fallback (NoiseSession) to continue as XX fallback
///   } else {
///     // IK succeeded — use result.writeCipher / readCipher
///   }
#[wasm_bindgen]
pub struct NoiseIkSession {
    state: Option<IkHandshakeState>,
    prologue: Vec<u8>,
}

#[wasm_bindgen]
impl NoiseIkSession {
    /// Create a new IK session.
    /// @param staticPub      Client's static public key (33 bytes, 0x05 prefix)
    /// @param staticPriv     Client's static private key (32 bytes)
    /// @param serverStaticPub Server's static public key (32 bytes, no prefix)
    /// @param clientPayload  The payload to send 0-RTT (e.g. client hello proto)
    /// @param prologue       Noise prologue bytes (WA header)
    #[wasm_bindgen(constructor)]
    pub fn new(
        static_pub: &[u8],
        static_priv: &[u8],
        server_static_pub: &[u8],
        client_payload: Vec<u8>,
        prologue: &[u8],
    ) -> Result<NoiseIkSession, JsValue> {
        let pub_key = wacore_libsignal::core::curve::PublicKey::deserialize(static_pub)
            .map_err(|e| JsValue::from_str(&format!("Invalid static pub key: {}", e)))?;
        let priv_key = wacore_libsignal::core::curve::PrivateKey::deserialize(static_priv)
            .map_err(|e| JsValue::from_str(&format!("Invalid static priv key: {}", e)))?;
        let kp = CoreKeyPair { public_key: pub_key, private_key: priv_key };

        let server_pub: [u8; 32] = server_static_pub
            .try_into()
            .map_err(|_| JsValue::from_str("Server static pub key must be 32 bytes"))?;

        let state = IkHandshakeState::new(kp, server_pub, client_payload, prologue)
            .map_err(|e| JsValue::from_str(&format!("IkHandshakeState::new failed: {}", e)))?;

        Ok(NoiseIkSession { state: Some(state), prologue: prologue.to_vec() })
    }

    /// Build the IK ClientHello bytes (framed, ready to send).
    #[wasm_bindgen(js_name = buildClientHello)]
    pub fn build_client_hello(&mut self) -> Result<Uint8Array, JsValue> {
        let state = self
            .state
            .as_mut()
            .ok_or_else(|| JsValue::from_str("IK session already consumed"))?;
        let bytes = state
            .build_client_hello()
            .map_err(|e| JsValue::from_str(&format!("buildClientHello failed: {}", e)))?;
        let arr = Uint8Array::new_with_length(bytes.len() as u32);
        arr.copy_from(&bytes);
        Ok(arr)
    }

    /// Process the server's response.
    /// Returns a JS object: `{ success: true, writeCipher, readCipher }` on IK success,
    /// or `{ success: false, fallbackSession: NoiseSession }` when the server requests XX fallback.
    #[wasm_bindgen(js_name = readServerHello)]
    pub fn read_server_hello(
        &mut self,
        response_bytes: &[u8],
        routing_info: Option<Vec<u8>>,
    ) -> Result<JsValue, JsValue> {
        let state = self
            .state
            .take()
            .ok_or_else(|| JsValue::from_str("IK session already consumed"))?;

        match state
            .read_server_hello(response_bytes)
            .map_err(|e| JsValue::from_str(&format!("readServerHello failed: {}", e)))?
        {
            IkServerHelloOutcome::Continue(outcome) => {
                // IK succeeded — return the two ciphers as a plain JS object
                let obj = js_sys::Object::new();
                let _ = js_sys::Reflect::set(&obj, &"success".into(), &JsValue::TRUE);

                // We can't expose NoiseCipher directly — store them in a completed NoiseSession
                // by constructing one with is_finished = true
                let session = NoiseSession::from_ciphers(outcome.write_cipher, outcome.read_cipher, routing_info);
                let _ = js_sys::Reflect::set(&obj, &"session".into(), &JsValue::from(session));
                Ok(obj.into())
            }
            IkServerHelloOutcome::Fallback(fallback_inputs) => {
                let obj = js_sys::Object::new();
                let _ = js_sys::Reflect::set(&obj, &"success".into(), &JsValue::FALSE);
                let fallback = NoiseXxFallbackSession::new(*fallback_inputs, &self.prologue, routing_info)?;
                let _ = js_sys::Reflect::set(&obj, &"fallback".into(), &JsValue::from(fallback));
                Ok(obj.into())
            }
        }
    }
}

// ── NoiseSession::from_ciphers (internal constructor) ────────────────────────

impl NoiseSession {
    fn from_ciphers(
        enc_cipher: NoiseCipher,
        dec_cipher: NoiseCipher,
        routing_info: Option<Vec<u8>>,
    ) -> NoiseSession {
        let (intro_header, _) = build_handshake_header(routing_info.as_deref());
        NoiseSession {
            handshake: None,
            enc_cipher: Some(enc_cipher),
            dec_cipher: Some(dec_cipher),
            read_counter: 0,
            write_counter: 0,
            is_finished: true,
            intro_header: Some(intro_header),
            frame_decoder: FrameDecoder::new(),
            encode_scratch: Vec::with_capacity(4096),
        }
    }
}

// ── NoiseXxFallbackSession ────────────────────────────────────────────────────

/// Noise XXfallback session — used when an IK attempt is rejected by the server.
/// Reuses the ephemeral already on the wire to avoid an extra round-trip.
#[wasm_bindgen]
pub struct NoiseXxFallbackSession {
    state: Option<XxFallbackHandshakeState>,
    routing_info: Option<Vec<u8>>,
}

#[wasm_bindgen]
impl NoiseXxFallbackSession {
    fn new(inputs: IkFallbackInputs, prologue: &[u8], routing_info: Option<Vec<u8>>) -> Result<Self, JsValue> {
        let state = XxFallbackHandshakeState::from_ik_failure(inputs, prologue)
            .map_err(|e| JsValue::from_str(&format!("XxFallback init failed: {}", e)))?;
        Ok(Self { state: Some(state), routing_info })
    }

    /// Build the client finish message (send this over the wire).
    #[wasm_bindgen(js_name = buildClientFinish)]
    pub fn build_client_finish(&mut self) -> Result<Uint8Array, JsValue> {
        let state = self
            .state
            .as_mut()
            .ok_or_else(|| JsValue::from_str("XxFallback already consumed"))?;
        let bytes = state
            .build_client_finish()
            .map_err(|e| JsValue::from_str(&format!("buildClientFinish failed: {}", e)))?;
        let arr = Uint8Array::new_with_length(bytes.len() as u32);
        arr.copy_from(&bytes);
        Ok(arr)
    }

    /// Finalize the handshake — returns a ready `NoiseSession`.
    #[wasm_bindgen(js_name = finish)]
    pub fn finish(&mut self) -> Result<NoiseSession, JsValue> {
        let state = self
            .state
            .take()
            .ok_or_else(|| JsValue::from_str("XxFallback already consumed"))?;
        let outcome = state
            .finish()
            .map_err(|e| JsValue::from_str(&format!("XxFallback finish failed: {}", e)))?;
        Ok(NoiseSession::from_ciphers(outcome.write_cipher, outcome.read_cipher, self.routing_info.take()))
    }
}
