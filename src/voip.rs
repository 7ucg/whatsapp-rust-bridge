//! VoIP / call media-plane bindings (from `wacore::voip`).
//!
//! Pure, no-runtime primitives for WhatsApp calls: the MLow audio codec, the
//! E2E SRTP media pipeline, and the sans-io CallEngine signaling/media driver.

use wacore::voip::{
    CallConfig, CallDirection, CallEngine as CoreCallEngine, CallEvent, Input,
    MediaPipeline as CoreMediaPipeline, MediaPipelineParams, MlowDecoder as CoreDecoder,
    MlowEncoder as CoreEncoder, Output, TxIdSource, NEVER,
};
use wasm_bindgen::prelude::*;

/// OS-RNG STUN transaction-id source (production-safe; consent freshness depends
/// on unpredictable ids).
struct RngTxIds;
impl TxIdSource for RngTxIds {
    fn next_tx_id(&mut self) -> [u8; 12] {
        rand::random()
    }
}

/// Shared: build a `CallConfig` from the JSON the bindings accept. Byte fields
/// (callKey/relayToken/integrityKey) are JSON arrays of bytes.
#[derive(serde::Deserialize)]
struct EngineConfigJson {
    call_id: String,
    /// "incoming" or "outgoing".
    direction: String,
    self_lid: String,
    peer_lid: String,
    call_key: Vec<u8>,
    ssrc: u32,
    samples_per_packet: u32,
    relay_token: Vec<u8>,
    relay_ip: String,
    relay_port: u16,
    integrity_key: Vec<u8>,
    warp_mi_tag_len: usize,
    enable_media: bool,
    enable_sframe: bool,
}

pub(crate) fn call_config_from_json(json: &str) -> Result<CallConfig, String> {
    let c: EngineConfigJson =
        serde_json::from_str(json).map_err(|e| format!("invalid engine config JSON: {e}"))?;
    let direction = match c.direction.as_str() {
        "incoming" => CallDirection::Incoming,
        "outgoing" => CallDirection::Outgoing,
        other => {
            return Err(format!(
                "direction must be incoming|outgoing, got {other:?}"
            ))
        }
    };
    Ok(CallConfig {
        call_id: c.call_id,
        direction,
        self_lid: c.self_lid,
        peer_lid: c.peer_lid,
        call_key: c.call_key,
        ssrc: c.ssrc,
        samples_per_packet: c.samples_per_packet,
        relay_token: c.relay_token,
        relay_ip: c.relay_ip,
        relay_port: c.relay_port,
        integrity_key: c.integrity_key,
        warp_mi_tag_len: c.warp_mi_tag_len,
        enable_media: c.enable_media,
        enable_sframe: c.enable_sframe,
    })
}

/// Output kinds returned by `pollOutput` (drain until `Timeout`).
pub mod output_kind {
    pub const TIMEOUT: i32 = 0;
    pub const TRANSMIT: i32 = 1;
    pub const PLAYOUT: i32 = 2;
    pub const EVENT: i32 = 3;
}

/// Event kinds returned by `eventKind` when `pollOutput` yields `EVENT`.
pub mod event_kind {
    pub const RELAY_ALLOCATED: i32 = 0;
    pub const FOREIGN_AUDIO: i32 = 1;
    pub const RELAY_ALLOCATE_FAILED: i32 = 2;
    pub const RELAY_ALLOCATE_TIMED_OUT: i32 = 3;
}

/// MLow audio encoder (WhatsApp call codec). Encodes f32 PCM frames into MLow
/// wire payloads. Stateful — keep one instance per outgoing stream.
#[wasm_bindgen]
pub struct MlowEncoder {
    inner: CoreEncoder,
}

#[wasm_bindgen]
impl MlowEncoder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MlowEncoder {
        MlowEncoder {
            inner: CoreEncoder::new(),
        }
    }

    /// Encode one PCM frame (f32 samples, -1.0..=1.0) into an MLow payload.
    #[wasm_bindgen]
    pub fn encode(&mut self, pcm: &[f32]) -> Result<Vec<u8>, JsValue> {
        self.inner
            .encode(pcm)
            .map_err(|e| JsValue::from_str(&format!("MlowEncoder.encode failed: {:?}", e)))
    }

    /// Reset encoder state (e.g. on a new call leg).
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.inner.reset();
    }
}

impl Default for MlowEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// MLow audio decoder. Decodes MLow wire payloads back to f32 PCM. Stateful —
/// keep one instance per incoming stream.
#[wasm_bindgen]
pub struct MlowDecoder {
    inner: CoreDecoder,
}

#[wasm_bindgen]
impl MlowDecoder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MlowDecoder {
        MlowDecoder {
            inner: CoreDecoder::new(),
        }
    }

    /// Decode one MLow payload into PCM (f32 samples). Empty input / loss
    /// concealment yields the decoder's PLC output.
    #[wasm_bindgen]
    pub fn decode(&mut self, payload: &[u8]) -> Vec<f32> {
        self.inner.decode(payload)
    }

    /// Set the number of redundant (RED) frames the decoder expects.
    #[wasm_bindgen(js_name = setRedundancy)]
    pub fn set_redundancy(&mut self, n: i32) {
        self.inner.set_redundancy(n);
    }

    /// Reset decoder state.
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.inner.reset();
    }
}

impl Default for MlowDecoder {
    fn default() -> Self {
        Self::new()
    }
}

/// E2E SRTP media pipeline for a call: derives the per-call SRTP keys from the
/// callKey and protects/unprotects audio packets. Stateful (the SRTP context
/// advances per packet) — keep one per call.
#[wasm_bindgen]
pub struct MediaPipeline {
    inner: CoreMediaPipeline,
}

#[wasm_bindgen]
impl MediaPipeline {
    /// Create the pipeline. `callKey` is the negotiated call key; `selfLid` /
    /// `peerLid` are the LID JIDs; `ssrc` the local stream SSRC. Returns an
    /// error if the callKey is too short to derive E2E keys.
    #[wasm_bindgen(js_name = create)]
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        call_key: &[u8],
        self_lid: &str,
        peer_lid: &str,
        ssrc: u32,
        samples_per_packet: u32,
        warp_mi_tag_len: usize,
    ) -> Result<MediaPipeline, JsValue> {
        let params = MediaPipelineParams {
            call_key,
            self_lid,
            peer_lid,
            ssrc,
            samples_per_packet,
            warp_mi_tag_len,
        };
        CoreMediaPipeline::new(&params)
            .map(|inner| MediaPipeline { inner })
            .ok_or_else(|| {
                JsValue::from_str("MediaPipeline.create: callKey too short for E2E keys")
            })
    }

    /// Encrypt + frame an audio payload (MLow/Opus) into an SRTP packet.
    #[wasm_bindgen(js_name = protectAudio)]
    pub fn protect_audio(&mut self, audio_payload: &[u8]) -> Vec<u8> {
        self.inner.protect_audio(audio_payload)
    }

    /// Decrypt an inbound SRTP packet into its audio payload, or `undefined` if
    /// the packet is not a decryptable audio packet.
    #[wasm_bindgen(js_name = unprotectAudio)]
    pub fn unprotect_audio(&mut self, packet: &[u8]) -> Option<Vec<u8>> {
        self.inner.unprotect_audio(packet).map(|(_hdr, pl)| pl)
    }

    /// Re-derive receive keys after the peer answers from a specific device.
    #[wasm_bindgen(js_name = rekeyRecv)]
    pub fn rekey_recv(&mut self, call_key: &[u8], answering_peer_lid: &str) -> bool {
        self.inner.rekey_recv(call_key, answering_peer_lid)
    }
}

/// Sans-io call engine: the signaling + media driver. Feed it inputs
/// (`handle*`), then drain `pollOutput` until it returns `TIMEOUT` (0), taking
/// each output's payload via the matching `take*`/`event*` getter. Drive timers
/// off `pollTimeout()`. All times are monotonic milliseconds (JS `number`).
#[wasm_bindgen]
pub struct CallEngine {
    inner: CoreCallEngine,
    last: Option<Output>,
}

#[wasm_bindgen]
impl CallEngine {
    /// Create the engine from a JSON config string (see `EngineConfigJson`
    /// fields: callId, direction, selfLid, peerLid, callKey[], ssrc,
    /// samplesPerPacket, relayToken[], relayIp, relayPort, integrityKey[],
    /// warpMiTagLen, enableMedia, enableSframe — snake_case keys).
    #[wasm_bindgen(js_name = create)]
    pub fn create(config_json: &str) -> Result<CallEngine, JsValue> {
        let cfg = call_config_from_json(config_json).map_err(|e| JsValue::from_str(&e))?;
        CoreCallEngine::new(cfg, Box::new(RngTxIds))
            .map(|inner| CallEngine { inner, last: None })
            .map_err(|e| JsValue::from_str(&format!("CallEngine.create: {e}")))
    }

    /// Start the call (kick off relay allocate). `now` = monotonic ms.
    pub fn start(&mut self, now: f64) {
        self.inner.start(now as u64);
    }

    /// Feed an inbound relay-channel packet.
    #[wasm_bindgen(js_name = handleRelayPacket)]
    pub fn handle_relay_packet(&mut self, now: f64, packet: &[u8]) {
        self.inner
            .handle_input(now as u64, Input::RelayPacket(packet));
    }

    /// Feed a 60ms mic frame (exactly 960 i16 samples, 16kHz mono).
    #[wasm_bindgen(js_name = handleMicFrame)]
    pub fn handle_mic_frame(&mut self, now: f64, pcm: &[i16]) {
        self.inner.handle_input(now as u64, Input::MicFrame(pcm));
    }

    /// Signal that the armed timer fired.
    #[wasm_bindgen(js_name = handleTimeout)]
    pub fn handle_timeout(&mut self, now: f64) {
        self.inner.handle_input(now as u64, Input::Timeout);
    }

    /// Drain one output. Returns its kind (0=TIMEOUT,1=TRANSMIT,2=PLAYOUT,
    /// 3=EVENT); fetch the payload with the matching getter, then call again
    /// until it returns 0 (TIMEOUT = drained).
    #[wasm_bindgen(js_name = pollOutput)]
    pub fn poll_output(&mut self) -> i32 {
        let out = self.inner.poll_output();
        let kind = match &out {
            Output::Timeout(_) => output_kind::TIMEOUT,
            Output::Transmit(_) => output_kind::TRANSMIT,
            Output::Playout(_) => output_kind::PLAYOUT,
            Output::Event(_) => output_kind::EVENT,
            _ => output_kind::TIMEOUT,
        };
        self.last = Some(out);
        kind
    }

    /// Payload of the last `TRANSMIT` output (bytes to send over the relay).
    #[wasm_bindgen(js_name = takeTransmit)]
    pub fn take_transmit(&mut self) -> Option<Vec<u8>> {
        match self.last.take() {
            Some(Output::Transmit(b)) => Some(b.to_vec()),
            other => {
                self.last = other;
                None
            }
        }
    }

    /// PCM of the last `PLAYOUT` output (i16 samples for the speaker).
    #[wasm_bindgen(js_name = takePlayout)]
    pub fn take_playout(&mut self) -> Option<Vec<i16>> {
        match self.last.take() {
            Some(Output::Playout(p)) => Some(p),
            other => {
                self.last = other;
                None
            }
        }
    }

    /// Deadline (ms) of the last `TIMEOUT` output, or -1 if none/no-timer.
    #[wasm_bindgen(js_name = lastTimeout)]
    pub fn last_timeout(&self) -> f64 {
        match &self.last {
            Some(Output::Timeout(m)) if *m != NEVER => *m as f64,
            _ => -1.0,
        }
    }

    /// Kind of the last `EVENT` (0=RelayAllocated,1=ForeignAudio,
    /// 2=RelayAllocateFailed,3=RelayAllocateTimedOut), or -1.
    #[wasm_bindgen(js_name = eventKind)]
    pub fn event_kind(&self) -> i32 {
        match &self.last {
            Some(Output::Event(e)) => match e {
                CallEvent::RelayAllocated => event_kind::RELAY_ALLOCATED,
                CallEvent::ForeignAudio(_) => event_kind::FOREIGN_AUDIO,
                CallEvent::RelayAllocateFailed(_) => event_kind::RELAY_ALLOCATE_FAILED,
                CallEvent::RelayAllocateTimedOut => event_kind::RELAY_ALLOCATE_TIMED_OUT,
                _ => -1,
            },
            _ => -1,
        }
    }

    /// Payload of a `ForeignAudio` event (a non-MLow inbound frame to decode
    /// with a platform codec).
    #[wasm_bindgen(js_name = takeForeignAudio)]
    pub fn take_foreign_audio(&mut self) -> Option<Vec<u8>> {
        match self.last.take() {
            Some(Output::Event(CallEvent::ForeignAudio(b))) => Some(b.to_vec()),
            other => {
                self.last = other;
                None
            }
        }
    }

    /// STUN error code of a `RelayAllocateFailed` event, or -1.
    #[wasm_bindgen(js_name = eventCode)]
    pub fn event_code(&self) -> i32 {
        match &self.last {
            Some(Output::Event(CallEvent::RelayAllocateFailed(c))) => *c as i32,
            _ => -1,
        }
    }

    /// Next timer deadline (ms), or -1 if no timer is armed.
    #[wasm_bindgen(js_name = pollTimeout)]
    pub fn poll_timeout(&self) -> f64 {
        match self.inner.poll_timeout() {
            Some(m) if m != NEVER => m as f64,
            _ => -1.0,
        }
    }

    /// Re-derive recv keys once the answering device LID is known.
    #[wasm_bindgen(js_name = rekeyRecv)]
    pub fn rekey_recv(&mut self, answering_peer_lid: &str) -> bool {
        self.inner.rekey_recv(answering_peer_lid)
    }

    #[wasm_bindgen(js_name = callId)]
    pub fn call_id(&self) -> String {
        self.inner.call_id().to_string()
    }

    /// 0 = outgoing, 1 = incoming.
    pub fn direction(&self) -> i32 {
        match self.inner.direction() {
            CallDirection::Outgoing => 0,
            CallDirection::Incoming => 1,
            _ => -1,
        }
    }

    #[wasm_bindgen(js_name = isAllocated)]
    pub fn is_allocated(&self) -> bool {
        self.inner.is_allocated()
    }

    #[wasm_bindgen(js_name = isTerminated)]
    pub fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }
}
