package com.whatsapp.bridge;

/**
 * WhatsApp Signal-protocol key generation backed by Rust via JNI.
 */
public final class KeyHelper {

    private KeyHelper() {}

    /** Generate a random 14-bit registration ID (0..16383). */
    public static native int generateRegistrationId();

    /**
     * Generate a pre-key pair.
     * @param keyId desired key ID
     * @return 65 bytes: [33 pubKey | 32 privKey]
     */
    public static native byte[] generatePreKey(int keyId);

    /**
     * Generate a signed pre-key.
     * @param identityPrivKey 32-byte identity private key
     * @param signedKeyId     desired key ID
     * @return 129 bytes: [33 pubKey | 32 privKey | 64 signature]
     */
    public static native byte[] generateSignedPreKey(byte[] identityPrivKey, int signedKeyId);

    // ── Result split helpers ──────────────────────────────────────────────────

    /** Extract pubKey (0..32) from a generatePreKey() / generateSignedPreKey() result. */
    public static byte[] pubKeyFrom(byte[] result) {
        byte[] pub = new byte[33];
        System.arraycopy(result, 0, pub, 0, 33);
        return pub;
    }

    /** Extract privKey (33..64) from a generatePreKey() / generateSignedPreKey() result. */
    public static byte[] privKeyFrom(byte[] result) {
        byte[] priv = new byte[32];
        System.arraycopy(result, 33, priv, 0, 32);
        return priv;
    }

    /** Extract signature (65..128) from a generateSignedPreKey() result. */
    public static byte[] signatureFrom(byte[] result) {
        if (result.length < 129) throw new IllegalArgumentException("Not a signed pre-key result");
        byte[] sig = new byte[64];
        System.arraycopy(result, 65, sig, 0, 64);
        return sig;
    }
}
