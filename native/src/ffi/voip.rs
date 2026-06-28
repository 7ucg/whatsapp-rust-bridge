//! C-ABI bindings for the MLow audio codec (`wacore::voip`).
//!
//! Encoder/decoder are stateful: create a handle with `*_new`, drive it with
//! `wa_mlow_encode` / `wa_mlow_decode`, and release it with `*_free`. The
//! encode/decode calls use the workspace `out_buf`/`out_len` convention — pass a
//! null out buffer (or one too small) to query the required length, then call
//! again with a buffer of that size.

use super::crypto::{write_out_pub, BridgeError};
use std::slice;
use wacore::voip::{
    CallConfig, CallDirection, CallEngine, CallEvent, Input, MediaPipeline, MediaPipelineParams,
    MlowDecoder, MlowEncoder, Output, TxIdSource, NEVER,
};

/// SAFETY: `ptr` valid for `len` bytes (or null/0 → empty).
#[inline]
unsafe fn opt_slice<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if ptr.is_null() || len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

/// SAFETY: `ptr` valid for `len` bytes; must be valid UTF-8.
#[inline]
unsafe fn opt_str<'a>(ptr: *const u8, len: usize) -> Option<&'a str> {
    std::str::from_utf8(unsafe { opt_slice(ptr, len) }).ok()
}

// ── encoder ─────────────────────────────────────────────────────────────────

/// Allocate a new MLow encoder. Free with `wa_mlow_encoder_free`.
#[unsafe(no_mangle)]
pub extern "C" fn wa_mlow_encoder_new() -> *mut MlowEncoder {
    Box::into_raw(Box::new(MlowEncoder::new()))
}

/// Free an encoder created by `wa_mlow_encoder_new`. Null is a no-op.
///
/// SAFETY: `enc` must come from `wa_mlow_encoder_new` and not be used after.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_mlow_encoder_free(enc: *mut MlowEncoder) {
    if !enc.is_null() {
        drop(unsafe { Box::from_raw(enc) });
    }
}

/// Reset encoder state (new call leg).
///
/// SAFETY: `enc` must be a valid encoder handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_mlow_encoder_reset(enc: *mut MlowEncoder) {
    if let Some(e) = unsafe { enc.as_mut() } {
        e.reset();
    }
}

/// Encode one PCM frame (`pcm`/`pcm_len` f32 samples) into `out_buf`.
///
/// SAFETY: `enc` valid handle; `pcm` valid for `pcm_len` floats; `out_buf`
/// valid for `*out_len` bytes; `out_len` non-null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_mlow_encode(
    enc: *mut MlowEncoder,
    pcm: *const f32,
    pcm_len: usize,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let Some(e) = (unsafe { enc.as_mut() }) else {
        return BridgeError::NullPointer as i32;
    };
    if out_len.is_null() || (pcm.is_null() && pcm_len != 0) {
        return BridgeError::NullPointer as i32;
    }
    let samples = if pcm_len == 0 {
        &[][..]
    } else {
        unsafe { slice::from_raw_parts(pcm, pcm_len) }
    };
    match e.encode(samples) {
        Ok(bytes) => unsafe { write_out_pub(&bytes, out_buf, out_len) },
        Err(_) => BridgeError::EncryptionFailed as i32,
    }
}

// ── decoder ─────────────────────────────────────────────────────────────────

/// Allocate a new MLow decoder. Free with `wa_mlow_decoder_free`.
#[unsafe(no_mangle)]
pub extern "C" fn wa_mlow_decoder_new() -> *mut MlowDecoder {
    Box::into_raw(Box::new(MlowDecoder::new()))
}

/// Free a decoder created by `wa_mlow_decoder_new`. Null is a no-op.
///
/// SAFETY: `dec` must come from `wa_mlow_decoder_new` and not be used after.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_mlow_decoder_free(dec: *mut MlowDecoder) {
    if !dec.is_null() {
        drop(unsafe { Box::from_raw(dec) });
    }
}

/// Reset decoder state.
///
/// SAFETY: `dec` must be a valid decoder handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_mlow_decoder_reset(dec: *mut MlowDecoder) {
    if let Some(d) = unsafe { dec.as_mut() } {
        d.reset();
    }
}

/// Set the number of redundant (RED) frames the decoder expects.
///
/// SAFETY: `dec` must be a valid decoder handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_mlow_decoder_set_redundancy(dec: *mut MlowDecoder, n: i32) {
    if let Some(d) = unsafe { dec.as_mut() } {
        d.set_redundancy(n);
    }
}

/// Decode one MLow payload into PCM. `out_buf` receives f32 samples and
/// `*out_len` is in **float elements** (not bytes) on input and output. Pass a
/// null/too-small buffer to query the required element count.
///
/// SAFETY: `dec` valid handle; `payload` valid for `payload_len` bytes;
/// `out_buf` valid for `*out_len` floats; `out_len` non-null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_mlow_decode(
    dec: *mut MlowDecoder,
    payload: *const u8,
    payload_len: usize,
    out_buf: *mut f32,
    out_len: *mut usize,
) -> i32 {
    let Some(d) = (unsafe { dec.as_mut() }) else {
        return BridgeError::NullPointer as i32;
    };
    if out_len.is_null() {
        return BridgeError::NullPointer as i32;
    }
    let pl = if payload.is_null() || payload_len == 0 {
        &[][..]
    } else {
        unsafe { slice::from_raw_parts(payload, payload_len) }
    };
    let pcm = d.decode(pl);
    let capacity = unsafe { *out_len };
    if out_buf.is_null() || capacity < pcm.len() {
        unsafe { *out_len = pcm.len() };
        return BridgeError::OutputTooSmall as i32;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(pcm.as_ptr(), out_buf, pcm.len());
        *out_len = pcm.len();
    }
    BridgeError::Ok as i32
}

// ── E2E SRTP media pipeline ──────────────────────────────────────────────────

/// Create a media pipeline from the callKey + LIDs. Returns null on bad UTF-8
/// or a callKey too short to derive E2E keys. Free with `wa_media_pipeline_free`.
///
/// SAFETY: each `*_ptr`/`*_len` pair must be valid; lids must be UTF-8.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_media_pipeline_new(
    call_key: *const u8,
    call_key_len: usize,
    self_lid: *const u8,
    self_lid_len: usize,
    peer_lid: *const u8,
    peer_lid_len: usize,
    ssrc: u32,
    samples_per_packet: u32,
    warp_mi_tag_len: usize,
) -> *mut MediaPipeline {
    let (Some(sl), Some(pl)) = (unsafe { opt_str(self_lid, self_lid_len) }, unsafe {
        opt_str(peer_lid, peer_lid_len)
    }) else {
        return std::ptr::null_mut();
    };
    let params = MediaPipelineParams {
        call_key: unsafe { opt_slice(call_key, call_key_len) },
        self_lid: sl,
        peer_lid: pl,
        ssrc,
        samples_per_packet,
        warp_mi_tag_len,
    };
    match MediaPipeline::new(&params) {
        Some(mp) => Box::into_raw(Box::new(mp)),
        None => std::ptr::null_mut(),
    }
}

/// Free a pipeline. Null is a no-op.
///
/// SAFETY: `mp` must come from `wa_media_pipeline_new` and not be used after.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_media_pipeline_free(mp: *mut MediaPipeline) {
    if !mp.is_null() {
        drop(unsafe { Box::from_raw(mp) });
    }
}

/// Encrypt + frame an audio payload into an SRTP packet (`out_len` in bytes).
///
/// SAFETY: `mp` valid handle; buffers valid for their lengths.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_media_pipeline_protect(
    mp: *mut MediaPipeline,
    payload: *const u8,
    payload_len: usize,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let Some(m) = (unsafe { mp.as_mut() }) else {
        return BridgeError::NullPointer as i32;
    };
    if out_len.is_null() {
        return BridgeError::NullPointer as i32;
    }
    let packet = m.protect_audio(unsafe { opt_slice(payload, payload_len) });
    unsafe { write_out_pub(&packet, out_buf, out_len) }
}

/// Decrypt an SRTP packet into its audio payload. Returns `WA_ERR_DECRYPT_FAIL`
/// (with `*out_len = 0`) if the packet is not a decryptable audio packet.
///
/// SAFETY: `mp` valid handle; buffers valid for their lengths.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_media_pipeline_unprotect(
    mp: *mut MediaPipeline,
    packet: *const u8,
    packet_len: usize,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let Some(m) = (unsafe { mp.as_mut() }) else {
        return BridgeError::NullPointer as i32;
    };
    if out_len.is_null() {
        return BridgeError::NullPointer as i32;
    }
    match m.unprotect_audio(unsafe { opt_slice(packet, packet_len) }) {
        Some((_hdr, payload)) => unsafe { write_out_pub(&payload, out_buf, out_len) },
        None => {
            unsafe { *out_len = 0 };
            BridgeError::DecryptionFailed as i32
        }
    }
}

/// Re-derive receive keys after the peer answers. Returns 1 on success, 0 on
/// no-op/failure, negative on null handle / bad UTF-8.
///
/// SAFETY: `mp` valid handle; `answering_peer_lid` valid UTF-8 for its length.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_media_pipeline_rekey_recv(
    mp: *mut MediaPipeline,
    call_key: *const u8,
    call_key_len: usize,
    answering_peer_lid: *const u8,
    answering_peer_lid_len: usize,
) -> i32 {
    let Some(m) = (unsafe { mp.as_mut() }) else {
        return BridgeError::NullPointer as i32;
    };
    let Some(lid) = (unsafe { opt_str(answering_peer_lid, answering_peer_lid_len) }) else {
        return BridgeError::NullPointer as i32;
    };
    m.rekey_recv(unsafe { opt_slice(call_key, call_key_len) }, lid) as i32
}

// ── sans-io CallEngine ───────────────────────────────────────────────────────

/// OS-RNG STUN transaction-id source (production-safe).
struct RngTxIds;
impl TxIdSource for RngTxIds {
    fn next_tx_id(&mut self) -> [u8; 12] {
        rand::random()
    }
}

#[derive(serde::Deserialize)]
struct EngineConfigJson {
    call_id: String,
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

pub(crate) fn call_config_from_json(json: &str) -> Option<CallConfig> {
    let c: EngineConfigJson = serde_json::from_str(json).ok()?;
    let direction = match c.direction.as_str() {
        "incoming" => CallDirection::Incoming,
        "outgoing" => CallDirection::Outgoing,
        _ => return None,
    };
    Some(CallConfig {
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

/// Opaque engine handle: the engine plus the single output awaiting a `take_*`.
pub struct WaCallEngine {
    pub(crate) inner: CallEngine,
    pub(crate) last: Option<Output>,
}

/// Build an engine from a JSON config (shared by the C-ABI and JNI bindings).
pub(crate) fn new_engine(json: &str) -> Option<WaCallEngine> {
    let cfg = call_config_from_json(json)?;
    CallEngine::new(cfg, Box::new(RngTxIds))
        .ok()
        .map(|inner| WaCallEngine { inner, last: None })
}

/// Output kinds: 0=Timeout(drained), 1=Transmit, 2=Playout, 3=Event.
/// Event kinds: 0=RelayAllocated, 1=ForeignAudio, 2=RelayAllocateFailed, 3=RelayAllocateTimedOut.
/// Create from a JSON config; null on parse error / short callKey / bad endpoint.
///
/// SAFETY: `config_json` valid UTF-8 for `config_json_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_new(
    config_json: *const u8,
    config_json_len: usize,
) -> *mut WaCallEngine {
    let Some(json) = (unsafe { opt_str(config_json, config_json_len) }) else {
        return std::ptr::null_mut();
    };
    match new_engine(json) {
        Some(e) => Box::into_raw(Box::new(e)),
        None => std::ptr::null_mut(),
    }
}

/// SAFETY: `eng` from `wa_call_engine_new`, not used after.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_free(eng: *mut WaCallEngine) {
    if !eng.is_null() {
        drop(unsafe { Box::from_raw(eng) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_start(eng: *mut WaCallEngine, now: u64) {
    if let Some(e) = unsafe { eng.as_mut() } {
        e.inner.start(now);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_handle_relay_packet(
    eng: *mut WaCallEngine,
    now: u64,
    packet: *const u8,
    packet_len: usize,
) {
    if let Some(e) = unsafe { eng.as_mut() } {
        e.inner.handle_input(
            now,
            Input::RelayPacket(unsafe { opt_slice(packet, packet_len) }),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_handle_mic_frame(
    eng: *mut WaCallEngine,
    now: u64,
    pcm: *const i16,
    pcm_len: usize,
) {
    if let Some(e) = unsafe { eng.as_mut() } {
        let frame = if pcm.is_null() || pcm_len == 0 {
            &[][..]
        } else {
            unsafe { slice::from_raw_parts(pcm, pcm_len) }
        };
        e.inner.handle_input(now, Input::MicFrame(frame));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_handle_timeout(eng: *mut WaCallEngine, now: u64) {
    if let Some(e) = unsafe { eng.as_mut() } {
        e.inner.handle_input(now, Input::Timeout);
    }
}

/// Drain one output; returns its kind and stores the payload for `take_*`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_poll_output(eng: *mut WaCallEngine) -> i32 {
    let Some(e) = (unsafe { eng.as_mut() }) else {
        return 0;
    };
    let out = e.inner.poll_output();
    let kind = match &out {
        Output::Timeout(_) => 0,
        Output::Transmit(_) => 1,
        Output::Playout(_) => 2,
        Output::Event(_) => 3,
        _ => 0,
    };
    e.last = Some(out);
    kind
}

/// Copy the last Transmit payload into `out_buf` (bytes). DecryptionFailed-style
/// 0-length if the last output wasn't a Transmit.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_take_transmit(
    eng: *mut WaCallEngine,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let Some(e) = (unsafe { eng.as_mut() }) else {
        return BridgeError::NullPointer as i32;
    };
    if out_len.is_null() {
        return BridgeError::NullPointer as i32;
    }
    match e.last.take() {
        Some(Output::Transmit(b)) => unsafe { write_out_pub(&b, out_buf, out_len) },
        other => {
            e.last = other;
            unsafe { *out_len = 0 };
            BridgeError::DecryptionFailed as i32
        }
    }
}

/// Copy the last Playout PCM into `out_buf` (i16; `out_len` in ELEMENTS).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_take_playout(
    eng: *mut WaCallEngine,
    out_buf: *mut i16,
    out_len: *mut usize,
) -> i32 {
    let Some(e) = (unsafe { eng.as_mut() }) else {
        return BridgeError::NullPointer as i32;
    };
    if out_len.is_null() {
        return BridgeError::NullPointer as i32;
    }
    match e.last.take() {
        Some(Output::Playout(pcm)) => {
            let cap = unsafe { *out_len };
            if out_buf.is_null() || cap < pcm.len() {
                unsafe { *out_len = pcm.len() };
                e.last = Some(Output::Playout(pcm));
                return BridgeError::OutputTooSmall as i32;
            }
            unsafe {
                std::ptr::copy_nonoverlapping(pcm.as_ptr(), out_buf, pcm.len());
                *out_len = pcm.len();
            }
            BridgeError::Ok as i32
        }
        other => {
            e.last = other;
            unsafe { *out_len = 0 };
            BridgeError::DecryptionFailed as i32
        }
    }
}

/// Deadline (ms) of the last Timeout output, or -1.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_last_timeout(eng: *mut WaCallEngine) -> i64 {
    match unsafe { eng.as_ref() }.and_then(|e| e.last.as_ref()) {
        Some(Output::Timeout(m)) if *m != NEVER => *m as i64,
        _ => -1,
    }
}

/// Kind of the last Event, or -1.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_event_kind(eng: *mut WaCallEngine) -> i32 {
    match unsafe { eng.as_ref() }.and_then(|e| e.last.as_ref()) {
        Some(Output::Event(ev)) => match ev {
            CallEvent::RelayAllocated => 0,
            CallEvent::ForeignAudio(_) => 1,
            CallEvent::RelayAllocateFailed(_) => 2,
            CallEvent::RelayAllocateTimedOut => 3,
            _ => -1,
        },
        _ => -1,
    }
}

/// Copy a ForeignAudio event payload into `out_buf` (bytes).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_take_foreign_audio(
    eng: *mut WaCallEngine,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let Some(e) = (unsafe { eng.as_mut() }) else {
        return BridgeError::NullPointer as i32;
    };
    if out_len.is_null() {
        return BridgeError::NullPointer as i32;
    }
    match e.last.take() {
        Some(Output::Event(CallEvent::ForeignAudio(b))) => unsafe {
            write_out_pub(&b, out_buf, out_len)
        },
        other => {
            e.last = other;
            unsafe { *out_len = 0 };
            BridgeError::DecryptionFailed as i32
        }
    }
}

/// STUN error code of a RelayAllocateFailed event, or -1.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_event_code(eng: *mut WaCallEngine) -> i32 {
    match unsafe { eng.as_ref() }.and_then(|e| e.last.as_ref()) {
        Some(Output::Event(CallEvent::RelayAllocateFailed(c))) => *c as i32,
        _ => -1,
    }
}

/// Next armed timer deadline (ms), or -1 if none.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_poll_timeout(eng: *mut WaCallEngine) -> i64 {
    match unsafe { eng.as_ref() }.and_then(|e| e.inner.poll_timeout()) {
        Some(m) if m != NEVER => m as i64,
        _ => -1,
    }
}

/// Re-derive recv keys once the answering device LID is known. 1=ok, 0=no-op, <0=error.
///
/// SAFETY: `answering_peer_lid` valid UTF-8 for its length.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_rekey_recv(
    eng: *mut WaCallEngine,
    answering_peer_lid: *const u8,
    answering_peer_lid_len: usize,
) -> i32 {
    let Some(e) = (unsafe { eng.as_mut() }) else {
        return BridgeError::NullPointer as i32;
    };
    let Some(lid) = (unsafe { opt_str(answering_peer_lid, answering_peer_lid_len) }) else {
        return BridgeError::NullPointer as i32;
    };
    e.inner.rekey_recv(lid) as i32
}

/// Copy the call id (UTF-8) into `out_buf`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_call_id(
    eng: *mut WaCallEngine,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let Some(e) = (unsafe { eng.as_ref() }) else {
        return BridgeError::NullPointer as i32;
    };
    if out_len.is_null() {
        return BridgeError::NullPointer as i32;
    }
    unsafe { write_out_pub(e.inner.call_id().as_bytes(), out_buf, out_len) }
}

/// 0 = outgoing, 1 = incoming, -1 = unknown/null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_direction(eng: *mut WaCallEngine) -> i32 {
    match unsafe { eng.as_ref() } {
        Some(e) => match e.inner.direction() {
            CallDirection::Outgoing => 0,
            CallDirection::Incoming => 1,
            _ => -1,
        },
        None => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_is_allocated(eng: *mut WaCallEngine) -> i32 {
    unsafe { eng.as_ref() }
        .map(|e| e.inner.is_allocated() as i32)
        .unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_call_engine_is_terminated(eng: *mut WaCallEngine) -> i32 {
    unsafe { eng.as_ref() }
        .map(|e| e.inner.is_terminated() as i32)
        .unwrap_or(0)
}
