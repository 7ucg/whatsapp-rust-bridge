//! JNI bindings for the MLow audio codec — mirrors com.whatsapp.bridge.Voip
//!
//! The encoder/decoder are stateful native objects; Java holds an opaque `long`
//! handle returned by `*New`, passes it to each call, and releases it with
//! `*Free`.

use jni::objects::{JByteArray, JClass, JFloatArray, JShortArray, JString};
use jni::sys::{jbyteArray, jfloatArray, jint, jlong, jshortArray};
use jni::JNIEnv;

use super::crypto::{bytes_from_jarray, bytes_to_jarray};
use crate::ffi::voip::{new_engine, WaCallEngine};
use wacore::voip::{CallDirection, CallEvent, Input, Output, NEVER};
use wacore::voip::{MediaPipeline, MediaPipelineParams, MlowDecoder, MlowEncoder};

fn string_from_jstring(env: &mut JNIEnv<'_>, s: JString<'_>) -> String {
    env.get_string(&s).map(|js| js.into()).unwrap_or_default()
}

fn shorts_from_jarray(env: &mut JNIEnv<'_>, arr: JShortArray<'_>) -> Vec<i16> {
    let len = env.get_array_length(&arr).unwrap_or(0);
    if len <= 0 {
        return vec![];
    }
    let mut buf = vec![0i16; len as usize];
    let _ = env.get_short_array_region(&arr, 0, &mut buf);
    buf
}

fn shorts_to_jarray(env: &mut JNIEnv<'_>, data: &[i16]) -> jshortArray {
    match env.new_short_array(data.len() as i32) {
        Ok(arr) => {
            let _ = env.set_short_array_region(&arr, 0, data);
            arr.into_raw()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

fn floats_from_jarray(env: &mut JNIEnv<'_>, arr: JFloatArray<'_>) -> Vec<f32> {
    let len = env.get_array_length(&arr).unwrap_or(0);
    if len <= 0 {
        return vec![];
    }
    let mut buf = vec![0f32; len as usize];
    let _ = env.get_float_array_region(&arr, 0, &mut buf);
    buf
}

fn floats_to_jarray(env: &mut JNIEnv<'_>, data: &[f32]) -> jfloatArray {
    match env.new_float_array(data.len() as i32) {
        Ok(arr) => {
            let _ = env.set_float_array_region(&arr, 0, data);
            arr.into_raw()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

// ── encoder ─────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mlowEncoderNew(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
) -> jlong {
    Box::into_raw(Box::new(MlowEncoder::new())) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mlowEncoderFree(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) {
    if handle != 0 {
        drop(unsafe { Box::from_raw(handle as *mut MlowEncoder) });
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mlowEncoderReset(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) {
    if let Some(e) = unsafe { (handle as *mut MlowEncoder).as_mut() } {
        e.reset();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mlowEncode<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
    pcm: JFloatArray<'a>,
) -> jbyteArray {
    let Some(e) = (unsafe { (handle as *mut MlowEncoder).as_mut() }) else {
        return std::ptr::null_mut();
    };
    let samples = floats_from_jarray(&mut env, pcm);
    match e.encode(&samples) {
        Ok(bytes) => bytes_to_jarray(&mut env, &bytes),
        Err(_) => std::ptr::null_mut(),
    }
}

// ── decoder ─────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mlowDecoderNew(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
) -> jlong {
    Box::into_raw(Box::new(MlowDecoder::new())) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mlowDecoderFree(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) {
    if handle != 0 {
        drop(unsafe { Box::from_raw(handle as *mut MlowDecoder) });
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mlowDecoderReset(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) {
    if let Some(d) = unsafe { (handle as *mut MlowDecoder).as_mut() } {
        d.reset();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mlowDecoderSetRedundancy(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
    n: jint,
) {
    if let Some(d) = unsafe { (handle as *mut MlowDecoder).as_mut() } {
        d.set_redundancy(n);
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mlowDecode<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
    payload: JByteArray<'a>,
) -> jfloatArray {
    let Some(d) = (unsafe { (handle as *mut MlowDecoder).as_mut() }) else {
        return std::ptr::null_mut();
    };
    let bytes = bytes_from_jarray(&mut env, payload);
    let pcm = d.decode(&bytes);
    floats_to_jarray(&mut env, &pcm)
}

// ── E2E SRTP media pipeline ──────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mediaPipelineNew<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    call_key: JByteArray<'a>,
    self_lid: JString<'a>,
    peer_lid: JString<'a>,
    ssrc: jint,
    samples_per_packet: jint,
    warp_mi_tag_len: jint,
) -> jlong {
    let ck = bytes_from_jarray(&mut env, call_key);
    let sl = string_from_jstring(&mut env, self_lid);
    let pl = string_from_jstring(&mut env, peer_lid);
    let params = MediaPipelineParams {
        call_key: &ck,
        self_lid: &sl,
        peer_lid: &pl,
        ssrc: ssrc as u32,
        samples_per_packet: samples_per_packet as u32,
        warp_mi_tag_len: warp_mi_tag_len as usize,
    };
    match MediaPipeline::new(&params) {
        Some(mp) => Box::into_raw(Box::new(mp)) as jlong,
        None => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mediaPipelineFree(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) {
    if handle != 0 {
        drop(unsafe { Box::from_raw(handle as *mut MediaPipeline) });
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mediaPipelineProtect<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
    payload: JByteArray<'a>,
) -> jbyteArray {
    let Some(m) = (unsafe { (handle as *mut MediaPipeline).as_mut() }) else {
        return std::ptr::null_mut();
    };
    let bytes = bytes_from_jarray(&mut env, payload);
    let packet = m.protect_audio(&bytes);
    bytes_to_jarray(&mut env, &packet)
}

/// Returns the decrypted payload, or null if the packet is not decryptable audio.
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mediaPipelineUnprotect<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
    packet: JByteArray<'a>,
) -> jbyteArray {
    let Some(m) = (unsafe { (handle as *mut MediaPipeline).as_mut() }) else {
        return std::ptr::null_mut();
    };
    let bytes = bytes_from_jarray(&mut env, packet);
    match m.unprotect_audio(&bytes) {
        Some((_hdr, payload)) => bytes_to_jarray(&mut env, &payload),
        None => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_mediaPipelineRekeyRecv<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
    call_key: JByteArray<'a>,
    answering_peer_lid: JString<'a>,
) -> jni::sys::jboolean {
    let Some(m) = (unsafe { (handle as *mut MediaPipeline).as_mut() }) else {
        return 0;
    };
    let ck = bytes_from_jarray(&mut env, call_key);
    let lid = string_from_jstring(&mut env, answering_peer_lid);
    m.rekey_recv(&ck, &lid) as jni::sys::jboolean
}

// ── sans-io CallEngine ───────────────────────────────────────────────────────

#[inline]
fn engine<'h>(handle: jlong) -> Option<&'h mut WaCallEngine> {
    unsafe { (handle as *mut WaCallEngine).as_mut() }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineNew<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    config_json: JString<'a>,
) -> jlong {
    let json = string_from_jstring(&mut env, config_json);
    match new_engine(&json) {
        Some(e) => Box::into_raw(Box::new(e)) as jlong,
        None => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineFree(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) {
    if handle != 0 {
        drop(unsafe { Box::from_raw(handle as *mut WaCallEngine) });
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineStart(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
    now: jlong,
) {
    if let Some(e) = engine(handle) {
        e.inner.start(now as u64);
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineHandleRelayPacket<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
    now: jlong,
    packet: JByteArray<'a>,
) {
    let bytes = bytes_from_jarray(&mut env, packet);
    if let Some(e) = engine(handle) {
        e.inner.handle_input(now as u64, Input::RelayPacket(&bytes));
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineHandleMicFrame<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
    now: jlong,
    pcm: JShortArray<'a>,
) {
    let frame = shorts_from_jarray(&mut env, pcm);
    if let Some(e) = engine(handle) {
        e.inner.handle_input(now as u64, Input::MicFrame(&frame));
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineHandleTimeout(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
    now: jlong,
) {
    if let Some(e) = engine(handle) {
        e.inner.handle_input(now as u64, Input::Timeout);
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEnginePollOutput(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jint {
    let Some(e) = engine(handle) else { return 0 };
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

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineTakeTransmit<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
) -> jbyteArray {
    let Some(e) = engine(handle) else {
        return std::ptr::null_mut();
    };
    match e.last.take() {
        Some(Output::Transmit(b)) => bytes_to_jarray(&mut env, &b),
        other => {
            e.last = other;
            std::ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineTakePlayout<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
) -> jshortArray {
    let Some(e) = engine(handle) else {
        return std::ptr::null_mut();
    };
    match e.last.take() {
        Some(Output::Playout(pcm)) => shorts_to_jarray(&mut env, &pcm),
        other => {
            e.last = other;
            std::ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineLastTimeout(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jlong {
    match engine(handle).and_then(|e| e.last.as_ref()) {
        Some(Output::Timeout(m)) if *m != NEVER => *m as jlong,
        _ => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineEventKind(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jint {
    match engine(handle).and_then(|e| e.last.as_ref()) {
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

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineTakeForeignAudio<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
) -> jbyteArray {
    let Some(e) = engine(handle) else {
        return std::ptr::null_mut();
    };
    match e.last.take() {
        Some(Output::Event(CallEvent::ForeignAudio(b))) => bytes_to_jarray(&mut env, &b),
        other => {
            e.last = other;
            std::ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineEventCode(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jint {
    match engine(handle).and_then(|e| e.last.as_ref()) {
        Some(Output::Event(CallEvent::RelayAllocateFailed(c))) => *c as jint,
        _ => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEnginePollTimeout(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jlong {
    match engine(handle).and_then(|e| e.inner.poll_timeout()) {
        Some(m) if m != NEVER => m as jlong,
        _ => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineRekeyRecv<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
    answering_peer_lid: JString<'a>,
) -> jni::sys::jboolean {
    let lid = string_from_jstring(&mut env, answering_peer_lid);
    match engine(handle) {
        Some(e) => e.inner.rekey_recv(&lid) as jni::sys::jboolean,
        None => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineCallId<'a>(
    env: JNIEnv<'a>,
    _class: JClass<'a>,
    handle: jlong,
) -> jni::sys::jstring {
    let id = engine(handle)
        .map(|e| e.inner.call_id().to_string())
        .unwrap_or_default();
    match env.new_string(id) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineDirection(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jint {
    match engine(handle) {
        Some(e) => match e.inner.direction() {
            CallDirection::Outgoing => 0,
            CallDirection::Incoming => 1,
            _ => -1,
        },
        None => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineIsAllocated(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jni::sys::jboolean {
    engine(handle)
        .map(|e| e.inner.is_allocated() as jni::sys::jboolean)
        .unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Voip_callEngineIsTerminated(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jni::sys::jboolean {
    engine(handle)
        .map(|e| e.inner.is_terminated() as jni::sys::jboolean)
        .unwrap_or(0)
}
