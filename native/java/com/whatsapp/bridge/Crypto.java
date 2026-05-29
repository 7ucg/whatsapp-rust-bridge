package com.whatsapp.bridge;

/**
 * WhatsApp cryptographic primitives backed by Rust via JNI.
 *
 * Load the native library once at startup:
 *   System.loadLibrary("whatsapp_bridge");
 * or via NativeLoader:
 *   NativeLoader.load();
 */
public final class Crypto {

    private Crypto() {}

    // ── Hashing ───────────────────────────────────────────────────────────────

    /** MD5 hash. Returns 16 bytes. */
    public static native byte[] md5(byte[] data);

    /** SHA-256 hash. Returns 32 bytes. */
    public static native byte[] sha256(byte[] data);

    /** HMAC-SHA256. Returns 32 bytes. */
    public static native byte[] hmacSha256(byte[] data, byte[] key);

    // ── HKDF ─────────────────────────────────────────────────────────────────

    /**
     * HKDF-SHA256 expand.
     * @param ikm    input key material
     * @param salt   may be empty/null (no salt)
     * @param info   may be empty/null (empty info)
     * @param outLen desired output length in bytes
     */
    public static native byte[] hkdf(byte[] ikm, byte[] salt, byte[] info, int outLen);

    // ── AES-256-GCM ───────────────────────────────────────────────────────────

    /**
     * AES-256-GCM encrypt. Auth tag (16 bytes) is appended to output.
     * @param key 32 bytes, iv 12 bytes, aad may be null
     */
    public static native byte[] aesEncryptGcm(byte[] plaintext, byte[] key, byte[] iv, byte[] aad);

    /**
     * AES-256-GCM decrypt. Ciphertext must include the 16-byte auth tag.
     * @param key 32 bytes, iv 12 bytes, aad may be null
     */
    public static native byte[] aesDecryptGcm(byte[] ciphertext, byte[] key, byte[] iv, byte[] aad);

    // ── AES-256-CBC ───────────────────────────────────────────────────────────

    /**
     * AES-256-CBC encrypt with random IV.
     * IV (16 bytes) is prepended to the returned ciphertext.
     * @param key 32 bytes
     */
    public static native byte[] aesEncryptCbc(byte[] plaintext, byte[] key);

    /**
     * AES-256-CBC decrypt. First 16 bytes of ciphertext must be the IV.
     * @param key 32 bytes
     */
    public static native byte[] aesDecryptCbc(byte[] ciphertext, byte[] key);

    /**
     * AES-256-CBC encrypt with an explicit IV (IV not included in output).
     * @param key 32 bytes, iv 16 bytes
     */
    public static native byte[] aesEncryptCbcWithIv(byte[] plaintext, byte[] key, byte[] iv);

    /**
     * AES-256-CBC decrypt with an explicit IV.
     * @param key 32 bytes, iv 16 bytes
     */
    public static native byte[] aesDecryptCbcWithIv(byte[] ciphertext, byte[] key, byte[] iv);

    // ── AES-256-CTR ───────────────────────────────────────────────────────────

    /**
     * AES-256-CTR encrypt/decrypt (symmetric — same for both directions).
     * @param key 32 bytes, iv 16 bytes
     */
    public static native byte[] aesCtr(byte[] data, byte[] key, byte[] iv);
}
