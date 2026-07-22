//! VoIP / call media-plane bindings (from `wacore::voip`).
//!
//! Pure, no-runtime primitives for WhatsApp calls: the MLow audio codec, the
//! E2E SRTP media pipeline, and the sans-io CallEngine signaling/media driver.

use wacore::voip::relay_parse;
use wacore::voip::{
    AudioConfig, CallConfig, CallDirection, CallEngine as CoreCallEngine, CallEvent, Input,
    MediaPipeline as CoreMediaPipeline, MediaPipelineParams, MlowDecoder as CoreDecoder,
    MlowEncoder as CoreEncoder, Output, TxIdSource, NEVER,
};
use wasm_bindgen::prelude::*;

/// `serde_wasm_bindgen::to_value` renders a Rust map/struct as a JS `Map` by
/// default; every other JSON-shaped export in this bridge is a plain object,
/// so route these through the same serializer with that switched off.
fn to_js_object<T: serde::Serialize + ?Sized>(value: &T) -> Result<JsValue, JsValue> {
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
    value
        .serialize(&serializer)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parse the `<relay>` block out of an encoded `<ack>` stanza (as produced by this
/// bridge's own binary-node encoder) and return the fields `CallEngine.create()`
/// needs to allocate the media relay: `relay_ip`, `relay_port`, `relay_token`,
/// `integrity_key`, `warp_mi_tag_len`. Signaling (offer/accept/ringing) stays on
/// the JS side; this only lifts the one relay-allocation block that CallEngine
/// can't derive itself, since it takes pre-parsed config, not raw stanzas.
#[wasm_bindgen(js_name = parseRelayFromAckNode)]
pub fn parse_relay_from_ack_node(encoded_ack_node: &[u8]) -> Result<JsValue, JsValue> {
    // Same framing `decodeNode` uses: a leading data-type byte (bit 2 = zlib), then the
    // marshaled node. encodeNode() (what JS re-encodes an already-parsed node with to get
    // here) produces exactly this shape.
    let unpacked = wacore_binary::util::unpack(encoded_ack_node)
        .map_err(|e| JsValue::from_str(&format!("unpack error: {e}")))?;
    let node = wacore_binary::marshal::unmarshal_ref(&unpacked)
        .map_err(|e| JsValue::from_str(&format!("decode error: {e}")))?;
    let relay_data = relay_parse::parse_relay_data_from_ack(&node)
        .ok_or_else(|| JsValue::from_str("no <relay> child on this node"))?;
    let endpoint = relay_parse::get_media_relay_endpoint(&relay_data)
        .ok_or_else(|| JsValue::from_str("relay has no usable media endpoint"))?;
    let (relay_ip, relay_port) = relay_parse::get_primary_ipv4_address(endpoint)
        .ok_or_else(|| JsValue::from_str("relay endpoint has no IPv4 address"))?;
    let relay_token = relay_data
        .relay_tokens
        .get(endpoint.token_id as usize)
        .filter(|t| !t.is_empty())
        .cloned()
        .ok_or_else(|| JsValue::from_str("relay has no token for this endpoint"))?;
    let integrity_key = relay_data
        .relay_key_ascii
        .clone()
        .ok_or_else(|| JsValue::from_str("relay has no <key> (STUN integrity key)"))?;
    let warp_mi_tag_len = relay_data.warp_mi_tag_len.unwrap_or(4);

    #[derive(serde::Serialize)]
    struct RelayConfig {
        relay_ip: String,
        relay_port: u16,
        relay_token: Vec<u8>,
        integrity_key: Vec<u8>,
        warp_mi_tag_len: u32,
    }
    let out = RelayConfig {
        relay_ip,
        relay_port,
        relay_token,
        integrity_key,
        warp_mi_tag_len,
    };
    to_js_object(&out)
}

/// Parse a `<call>` stanza (offer/preaccept/accept/reject/terminate/transport/
/// relaylatency/video) into a plain JSON object for JS, converting every `Jid`
/// field to its string form (`user@server`) instead of the raw `{user, server,
/// agent, device, integrator}` shape `serde` would give it. Returns `null` for a
/// stanza with no known action child (forward-compat: a future server action).
///
/// `own_jid` (optional, `user@server` string) selects which `<enc>` in an offer
/// is ours when the offer is multi-device; omit it for a single-device offer.
///
/// The returned `media` field (present only on an `<offer>` that carries an
/// `<enc>` for us) has `enc_type`/`version`/`ciphertext` (the Signal ciphertext
/// to decrypt via your own session — this bridge does not manage Signal state)
/// and, when the offer carried one, a `relay` block shaped exactly like
/// `parseRelayFromAckNode`'s output.
#[wasm_bindgen(js_name = parseCallStanza)]
pub fn parse_call_stanza_js(
    encoded_node: &[u8],
    own_jid: Option<String>,
) -> Result<JsValue, JsValue> {
    use wacore::types::call::CallAction;

    let unpacked = wacore_binary::util::unpack(encoded_node)
        .map_err(|e| JsValue::from_str(&format!("unpack error: {e}")))?;
    let node = wacore_binary::marshal::unmarshal_ref(&unpacked)
        .map_err(|e| JsValue::from_str(&format!("decode error: {e}")))?;
    let Some(incoming) = wacore::stanza::call::parse_call_stanza(&node)
        .map_err(|e| JsValue::from_str(&e.to_string()))?
    else {
        return Ok(JsValue::NULL);
    };

    let jid_str = |j: &wacore_binary::Jid| j.to_string();
    let audio_json = |codecs: &[wacore::types::call::CallAudioCodec]| {
        codecs
            .iter()
            .map(|c| serde_json::json!({ "enc": c.enc, "rate": c.rate }))
            .collect::<Vec<_>>()
    };

    let action = match &incoming.action {
        CallAction::Offer {
            call_id,
            call_creator,
            caller_pn,
            caller_country_code,
            device_class,
            joinable,
            is_video,
            audio,
            group_jid,
        } => serde_json::json!({
            "type": "offer",
            "call_id": call_id,
            "call_creator": jid_str(call_creator),
            "caller_pn": caller_pn.as_ref().map(jid_str),
            "caller_country_code": caller_country_code,
            "device_class": device_class,
            "joinable": joinable,
            "is_video": is_video,
            "audio": audio_json(audio),
            "group_jid": group_jid.as_ref().map(jid_str),
        }),
        CallAction::OfferNotice {
            call_id,
            call_creator,
            is_video,
            is_group,
        } => serde_json::json!({
            "type": "offer_notice",
            "call_id": call_id,
            "call_creator": jid_str(call_creator),
            "is_video": is_video,
            "is_group": is_group,
        }),
        CallAction::PreAccept {
            call_id,
            call_creator,
            audio,
        } => serde_json::json!({
            "type": "preaccept",
            "call_id": call_id,
            "call_creator": jid_str(call_creator),
            "audio": audio_json(audio),
        }),
        CallAction::Accept {
            call_id,
            call_creator,
            audio,
        } => serde_json::json!({
            "type": "accept",
            "call_id": call_id,
            "call_creator": jid_str(call_creator),
            "audio": audio_json(audio),
        }),
        CallAction::Reject {
            call_id,
            call_creator,
        } => serde_json::json!({
            "type": "reject",
            "call_id": call_id,
            "call_creator": jid_str(call_creator),
        }),
        CallAction::Terminate {
            call_id,
            call_creator,
            reason,
            duration,
            audio_duration,
        } => serde_json::json!({
            "type": "terminate",
            "call_id": call_id,
            "call_creator": jid_str(call_creator),
            "reason": reason,
            "duration": duration,
            "audio_duration": audio_duration,
        }),
        CallAction::Transport {
            call_id,
            call_creator,
            p2p_cand_round,
            transport_message_type,
        } => serde_json::json!({
            "type": "transport",
            "call_id": call_id,
            "call_creator": jid_str(call_creator),
            "p2p_cand_round": p2p_cand_round,
            "transport_message_type": transport_message_type,
        }),
        CallAction::RelayLatency {
            call_id,
            call_creator,
        } => serde_json::json!({
            "type": "relaylatency",
            "call_id": call_id,
            "call_creator": jid_str(call_creator),
        }),
        CallAction::VideoState {
            call_id,
            call_creator,
            state,
            orientation,
            dec,
        } => serde_json::json!({
            "type": "video",
            "call_id": call_id,
            "call_creator": jid_str(call_creator),
            "state": state.code(),
            "orientation": orientation,
            "dec": dec,
        }),
        // CallAction is #[non_exhaustive]: a future server action falls back to a bare
        // type+call_id+call_creator so JS can still see the call_id and ignore the rest,
        // rather than the whole parse failing.
        other => serde_json::json!({
            "type": other.action_kind(),
            "call_id": other.call_id(),
            "call_creator": jid_str(other.call_creator()),
        }),
    };

    let mut out = serde_json::json!({
        "from": jid_str(&incoming.from),
        "stanza_id": incoming.stanza_id,
        "notify": incoming.notify,
        "platform": incoming.platform,
        "version": incoming.version,
        "timestamp": incoming.timestamp.timestamp(),
        "offline": incoming.offline,
        "action": action,
    });

    if let Some(media) = &incoming.media {
        let own = own_jid.as_deref().and_then(|s| s.parse().ok());
        let enc = media.enc_for(own.as_ref());
        let mut media_json = serde_json::json!({});
        if let Some(enc) = enc {
            media_json["enc_type"] = serde_json::json!(enc.enc_type);
            media_json["version"] = serde_json::json!(enc.version);
            media_json["ciphertext"] = serde_json::json!(enc.ciphertext);
        }
        if let Some(relay_data) = &media.relay {
            if let Some(endpoint) = relay_parse::get_media_relay_endpoint(relay_data) {
                if let Some((relay_ip, relay_port)) =
                    relay_parse::get_primary_ipv4_address(endpoint)
                {
                    let relay_token = relay_data
                        .relay_tokens
                        .get(endpoint.token_id as usize)
                        .filter(|t| !t.is_empty())
                        .cloned();
                    media_json["relay"] = serde_json::json!({
                        "relay_ip": relay_ip,
                        "relay_port": relay_port,
                        "relay_token": relay_token,
                        "integrity_key": relay_data.relay_key_ascii,
                        "warp_mi_tag_len": relay_data.warp_mi_tag_len.unwrap_or(4),
                    });
                }
            }
        }
        if enc.is_some() || media.relay.is_some() {
            out["media"] = media_json;
        }
    }

    to_js_object(&out)
}

/// Convert a built `Node` into the plain `{tag, attrs, content}` shape `sock.sendNode()`
/// already expects (the same shape `encodeBinaryNode`'s caller builds by hand), so a
/// caller never has to touch this bridge's internal `Node`/`Attrs`/`NodeContent` types.
fn node_to_js_value(node: &wacore_binary::Node) -> JsValue {
    let obj = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("tag"),
        &JsValue::from_str(&node.tag),
    );

    let attrs = js_sys::Object::new();
    for (k, v) in node.attrs.iter() {
        let s = match v {
            wacore_binary::NodeValue::String(s) => s.to_string(),
            wacore_binary::NodeValue::Jid(j) => j.to_string(),
        };
        let _ = js_sys::Reflect::set(&attrs, &JsValue::from_str(k), &JsValue::from_str(&s));
    }
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("attrs"), &attrs);

    // encodeNode() treats an explicit `content: null` as real (empty) content and chokes on
    // it — omit the key entirely when there's none, matching what a hand-built JS node with
    // no content looks like. Byte content must be a real Uint8Array (encodeNode rejects a
    // plain number array), so this builds the JsValue tree directly rather than round-tripping
    // through serde_json::Value, which has no byte-array type of its own.
    if let Some(content) = &node.content {
        let value: JsValue = match content {
            wacore_binary::NodeContent::Bytes(b) => js_sys::Uint8Array::from(b.as_slice()).into(),
            wacore_binary::NodeContent::String(s) => JsValue::from_str(s),
            wacore_binary::NodeContent::Nodes(ns) => {
                let arr = js_sys::Array::new();
                for n in ns.iter() {
                    arr.push(&node_to_js_value(n));
                }
                arr.into()
            }
        };
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("content"), &value);
    }
    obj.into()
}

fn parse_jid(s: &str) -> Result<wacore_binary::Jid, JsValue> {
    s.parse()
        .map_err(|_| JsValue::from_str(&format!("invalid JID: {s}")))
}

#[derive(serde::Deserialize)]
struct OfferDeviceKeyJson {
    device_jid: String,
    ciphertext: Vec<u8>,
    enc_type: String,
}

#[derive(serde::Deserialize)]
struct OfferParamsJson {
    call_id: String,
    to: String,
    call_creator: String,
    device_keys: Vec<OfferDeviceKeyJson>,
    #[serde(default)]
    privacy_token: Option<Vec<u8>>,
    #[serde(default)]
    capability: Option<Vec<u8>>,
    #[serde(default)]
    device_identity: Option<Vec<u8>>,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    multi_device: bool,
    #[serde(default)]
    video: bool,
    audio_rates: Vec<String>,
}

/// Build a `<call><offer>...</offer></call>` stanza for an outbound call. `device_keys`
/// carries one entry per destination device with the callKey already Signal-encrypted
/// for it (this bridge builds/parses stanzas only — it doesn't touch Signal sessions).
/// Returns a plain `{tag, attrs, content}` object ready for `sock.sendNode()`.
#[wasm_bindgen(js_name = buildOfferStanza)]
pub fn build_offer_stanza(params_json: &str) -> Result<JsValue, JsValue> {
    let p: OfferParamsJson = serde_json::from_str(params_json)
        .map_err(|e| JsValue::from_str(&format!("invalid offer params JSON: {e}")))?;

    let to = parse_jid(&p.to)?;
    let call_creator = parse_jid(&p.call_creator)?;
    let mut device_keys = Vec::with_capacity(p.device_keys.len());
    for dk in &p.device_keys {
        device_keys.push(wacore::stanza::call::OfferDeviceKey {
            device_jid: parse_jid(&dk.device_jid)?,
            ciphertext: dk.ciphertext.clone(),
            enc_type: dk.enc_type.clone(),
        });
    }
    let audio_rates: Vec<&str> = p.audio_rates.iter().map(String::as_str).collect();

    let node = wacore::stanza::call::build_offer(&wacore::stanza::call::OfferParams {
        call_id: &p.call_id,
        to: &to,
        call_creator: &call_creator,
        device_keys: &device_keys,
        privacy_token: p.privacy_token.as_deref(),
        capability: p.capability.as_deref(),
        device_identity: p.device_identity.as_deref(),
        id: p.id.as_deref(),
        multi_device: p.multi_device,
        video: p.video,
        audio_rates: &audio_rates,
    });

    Ok(node_to_js_value(&node))
}

#[derive(serde::Deserialize)]
struct AcceptParamsJson {
    call_id: String,
    to: String,
    id: String,
    call_creator: String,
    audio_rates: Vec<String>,
    #[serde(default)]
    relay_te: Option<Vec<u8>>,
    #[serde(default)]
    rte: Option<Vec<u8>>,
    #[serde(default)]
    voip_settings: Option<Vec<u8>>,
    #[serde(default)]
    capability: Option<Vec<u8>>,
    #[serde(default)]
    video: bool,
    #[serde(default)]
    peer_abtest_bucket: Option<String>,
    #[serde(default)]
    peer_abtest_bucket_id_list: Option<String>,
}

/// Build a `<call><accept>...</accept></call>` stanza to answer an incoming offer.
#[wasm_bindgen(js_name = buildAcceptStanza)]
pub fn build_accept_stanza(params_json: &str) -> Result<JsValue, JsValue> {
    let p: AcceptParamsJson = serde_json::from_str(params_json)
        .map_err(|e| JsValue::from_str(&format!("invalid accept params JSON: {e}")))?;

    let to = parse_jid(&p.to)?;
    let call_creator = parse_jid(&p.call_creator)?;
    let audio_rates: Vec<&str> = p.audio_rates.iter().map(String::as_str).collect();

    let node = wacore::stanza::call::build_accept(&wacore::stanza::call::AcceptParams {
        call_id: &p.call_id,
        to: &to,
        id: &p.id,
        call_creator: &call_creator,
        audio_rates: &audio_rates,
        relay_te: p.relay_te.as_deref(),
        rte: p.rte.as_deref(),
        voip_settings: p.voip_settings.as_deref(),
        capability: p.capability.as_deref(),
        video: p.video,
        peer_abtest_bucket: p.peer_abtest_bucket.as_deref(),
        peer_abtest_bucket_id_list: p.peer_abtest_bucket_id_list.as_deref(),
    });

    Ok(node_to_js_value(&node))
}

#[derive(serde::Deserialize)]
struct PreacceptParamsJson {
    call_id: String,
    to: String,
    call_creator: String,
    wrapper_id: String,
    audio_rates: Vec<String>,
    #[serde(default)]
    video: bool,
}

/// Build a `<call><preaccept>...</preaccept></call>` stanza: the early "ringing,
/// about to answer" ack sent before the real `<accept>`.
#[wasm_bindgen(js_name = buildPreacceptStanza)]
pub fn build_preaccept_stanza(params_json: &str) -> Result<JsValue, JsValue> {
    let p: PreacceptParamsJson = serde_json::from_str(params_json)
        .map_err(|e| JsValue::from_str(&format!("invalid preaccept params JSON: {e}")))?;

    let to = parse_jid(&p.to)?;
    let call_creator = parse_jid(&p.call_creator)?;
    let audio_rates: Vec<&str> = p.audio_rates.iter().map(String::as_str).collect();

    let node = wacore::stanza::call::build_preaccept(
        &p.call_id,
        &to,
        &call_creator,
        &p.wrapper_id,
        &audio_rates,
        p.video,
    );

    Ok(node_to_js_value(&node))
}

#[derive(serde::Deserialize)]
struct TerminateParamsJson {
    call_id: String,
    to: String,
    #[serde(default)]
    id: Option<String>,
    call_creator: String,
    #[serde(default)]
    reason: Option<String>,
}

/// Build a `<call><terminate>...</terminate></call>` stanza to end a call (hangup,
/// reject an offer already accepted elsewhere, etc).
#[wasm_bindgen(js_name = buildTerminateStanza)]
pub fn build_terminate_stanza(params_json: &str) -> Result<JsValue, JsValue> {
    let p: TerminateParamsJson = serde_json::from_str(params_json)
        .map_err(|e| JsValue::from_str(&format!("invalid terminate params JSON: {e}")))?;

    let to = parse_jid(&p.to)?;
    let call_creator = parse_jid(&p.call_creator)?;

    let node = wacore::stanza::call::build_terminate(&wacore::stanza::call::TerminateParams {
        call_id: &p.call_id,
        to: &to,
        id: p.id.as_deref(),
        call_creator: &call_creator,
        reason: p.reason.as_deref(),
    });

    Ok(node_to_js_value(&node))
}

#[derive(serde::Deserialize)]
struct RejectParamsJson {
    call_id: String,
    to: String,
    call_creator: String,
    wrapper_id: String,
}

/// Build a `<call><reject>...</reject></call>` stanza to decline an incoming offer.
#[wasm_bindgen(js_name = buildRejectStanza)]
pub fn build_reject_stanza(params_json: &str) -> Result<JsValue, JsValue> {
    let p: RejectParamsJson = serde_json::from_str(params_json)
        .map_err(|e| JsValue::from_str(&format!("invalid reject params JSON: {e}")))?;

    let to = parse_jid(&p.to)?;
    let call_creator = parse_jid(&p.call_creator)?;

    let node = wacore::stanza::call::build_reject(&p.call_id, &to, &call_creator, &p.wrapper_id);

    Ok(node_to_js_value(&node))
}

/// Derive the local participant SSRC `CallEngine.create()`'s config needs, the same way
/// `wacore`'s own `CallConfig::from_relay` does internally (HKDF-SHA256 over the call id and
/// participant LID) — exposed so a JS caller building the config by hand doesn't have to
/// reimplement this bit-exact derivation. `slot_word` is 0 for audio, 1 for video.
#[wasm_bindgen(js_name = deriveCallParticipantSsrc)]
pub fn derive_call_participant_ssrc(call_id: &str, self_lid: &str, slot_word: u32) -> u32 {
    let participant_id = wacore::voip::ssrc::format_e2e_srtp_participant_id(self_lid);
    wacore::voip::ssrc::derive_wasm_participant_ssrc(call_id, &participant_id, slot_word)
}

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
        // buffa/voip rework: the per-packet framing now lives in AudioConfig.
        audio: AudioConfig::MLOW_PCM,
        relay_token: c.relay_token,
        relay_ip: c.relay_ip,
        relay_port: c.relay_port,
        integrity_key: c.integrity_key,
        warp_mi_tag_len: c.warp_mi_tag_len,
        enable_media: c.enable_media,
        enable_video: false,
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

    /// Start the call (kick off relay allocate). `now` = monotonic ms,
    /// `wallclockMs` = unix epoch ms (the engine stamps signaling with it).
    pub fn start(&mut self, now: f64, wallclock_ms: f64) {
        self.inner.start(now as u64, wallclock_ms as u64);
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
