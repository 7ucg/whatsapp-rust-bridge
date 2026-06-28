package com.whatsapp.bridge;

/**
 * WhatsApp VoIP / call media-plane primitives backed by Rust via JNI.
 *
 * Currently the MLow audio codec. The encoder and decoder are stateful native
 * objects: create one with {@code mlow*New()}, drive it, and release it with
 * {@code mlow*Free()} (never reuse a handle after freeing).
 *
 * Load the native library once at startup:
 *   NativeLoader.load();
 */
public final class Voip {

    private Voip() {}

    // ── MLow encoder ──────────────────────────────────────────────────────────

    /** Allocate a new MLow encoder; returns an opaque handle. */
    public static native long mlowEncoderNew();

    /** Encode one PCM frame (f32 samples, -1.0..=1.0) into an MLow payload. */
    public static native byte[] mlowEncode(long handle, float[] pcm);

    /** Reset encoder state (e.g. new call leg). */
    public static native void mlowEncoderReset(long handle);

    /** Free an encoder handle. Do not use the handle afterwards. */
    public static native void mlowEncoderFree(long handle);

    // ── MLow decoder ──────────────────────────────────────────────────────────

    /** Allocate a new MLow decoder; returns an opaque handle. */
    public static native long mlowDecoderNew();

    /** Decode one MLow payload into PCM (f32 samples). */
    public static native float[] mlowDecode(long handle, byte[] payload);

    /** Set the number of redundant (RED) frames the decoder expects. */
    public static native void mlowDecoderSetRedundancy(long handle, int n);

    /** Reset decoder state. */
    public static native void mlowDecoderReset(long handle);

    /** Free a decoder handle. Do not use the handle afterwards. */
    public static native void mlowDecoderFree(long handle);

    // ── E2E SRTP media pipeline ────────────────────────────────────────────────

    /**
     * Create an E2E SRTP pipeline for a call. Returns an opaque handle, or 0 if
     * the callKey is too short to derive E2E keys.
     */
    public static native long mediaPipelineNew(
            byte[] callKey, String selfLid, String peerLid,
            int ssrc, int samplesPerPacket, int warpMiTagLen);

    /** Encrypt + frame an audio payload (MLow/Opus) into an SRTP packet. */
    public static native byte[] mediaPipelineProtect(long handle, byte[] audioPayload);

    /** Decrypt an SRTP packet into its audio payload, or null if undecryptable. */
    public static native byte[] mediaPipelineUnprotect(long handle, byte[] packet);

    /** Re-derive receive keys after the peer answers from a device. */
    public static native boolean mediaPipelineRekeyRecv(
            long handle, byte[] callKey, String answeringPeerLid);

    /** Free a pipeline handle. Do not use the handle afterwards. */
    public static native void mediaPipelineFree(long handle);

    // ── CallEngine (sans-io signaling + media driver) ───────────────────────────
    //
    // Drive loop: feed inputs (callEngineHandle*), then drain callEnginePollOutput
    // until it returns 0 (TIMEOUT = drained), fetching each output's payload with
    // the matching take*/event* getter. Arm timers off callEnginePollTimeout().
    // All times are monotonic milliseconds.

    /** Create the engine from a JSON config; returns a handle, or 0 on error. */
    public static native long callEngineNew(String configJson);

    /** Free an engine handle. */
    public static native void callEngineFree(long handle);

    /** Start the call (kick off relay allocate). */
    public static native void callEngineStart(long handle, long nowMs);

    /** Feed an inbound relay-channel packet. */
    public static native void callEngineHandleRelayPacket(long handle, long nowMs, byte[] packet);

    /** Feed a 60ms mic frame (exactly 960 shorts, 16kHz mono). */
    public static native void callEngineHandleMicFrame(long handle, long nowMs, short[] pcm);

    /** Signal that the armed timer fired. */
    public static native void callEngineHandleTimeout(long handle, long nowMs);

    /** Drain one output: 0=TIMEOUT(drained), 1=TRANSMIT, 2=PLAYOUT, 3=EVENT. */
    public static native int callEnginePollOutput(long handle);

    /** Payload of the last TRANSMIT output, or null. */
    public static native byte[] callEngineTakeTransmit(long handle);

    /** PCM of the last PLAYOUT output (16kHz mono shorts), or null. */
    public static native short[] callEngineTakePlayout(long handle);

    /** Deadline (ms) of the last TIMEOUT output, or -1. */
    public static native long callEngineLastTimeout(long handle);

    /** Kind of the last EVENT (0=Allocated,1=ForeignAudio,2=AllocFailed,3=AllocTimedOut), or -1. */
    public static native int callEngineEventKind(long handle);

    /** Payload of a ForeignAudio event, or null. */
    public static native byte[] callEngineTakeForeignAudio(long handle);

    /** STUN error code of a RelayAllocateFailed event, or -1. */
    public static native int callEngineEventCode(long handle);

    /** Next armed timer deadline (ms), or -1. */
    public static native long callEnginePollTimeout(long handle);

    /** Re-derive recv keys once the answering device LID is known. */
    public static native boolean callEngineRekeyRecv(long handle, String answeringPeerLid);

    /** The call id. */
    public static native String callEngineCallId(long handle);

    /** 0 = outgoing, 1 = incoming. */
    public static native int callEngineDirection(long handle);

    /** Whether the relay allocate has succeeded (media path live). */
    public static native boolean callEngineIsAllocated(long handle);

    /** Whether the call has terminated. */
    public static native boolean callEngineIsTerminated(long handle);
}
