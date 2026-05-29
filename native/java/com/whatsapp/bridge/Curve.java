package com.whatsapp.bridge;

/**
 * Curve25519 key operations backed by Rust via JNI.
 */
public final class Curve {

    private Curve() {}

    /**
     * Generate a Curve25519 key pair.
     * @return 65 bytes: [33 bytes pubKey (0x05 prefix) | 32 bytes privKey]
     */
    public static native byte[] generateKeyPair();

    /**
     * Diffie-Hellman shared secret.
     * @param pubKey  32 or 33 bytes (0x05 prefix optional)
     * @param privKey 32 bytes
     * @return 32-byte shared secret
     */
    public static native byte[] calculateAgreement(byte[] pubKey, byte[] privKey);

    /**
     * Sign a message with an Ed25519-flavored Curve25519 private key.
     * @param privKey 32 bytes
     * @return 64-byte signature
     */
    public static native byte[] calculateSignature(byte[] privKey, byte[] message);

    /**
     * Verify a signature.
     * @param pubKey    32 or 33 bytes
     * @param signature 64 bytes
     * @return true if valid
     * @throws RuntimeException on invalid key/signature format
     */
    public static boolean verifySignature(byte[] pubKey, byte[] message, byte[] signature) {
        return nativeVerifySignature(pubKey, message, signature) == 1;
    }

    private static native int nativeVerifySignature(byte[] pubKey, byte[] message, byte[] signature);

    /**
     * Derive the public key from a private key.
     * @param privKey 32 bytes
     * @return 33 bytes (0x05 prefix + 32 bytes)
     */
    public static native byte[] getPublicFromPrivate(byte[] privKey);

    // ── Convenience key-pair split helpers ───────────────────────────────────

    /** Extract the public key (bytes 0-32) from a generateKeyPair() result. */
    public static byte[] pubKeyFrom(byte[] keyPair) {
        if (keyPair.length < 33) throw new IllegalArgumentException("Not a key pair");
        byte[] pub = new byte[33];
        System.arraycopy(keyPair, 0, pub, 0, 33);
        return pub;
    }

    /** Extract the private key (bytes 33-64) from a generateKeyPair() result. */
    public static byte[] privKeyFrom(byte[] keyPair) {
        if (keyPair.length < 65) throw new IllegalArgumentException("Not a key pair");
        byte[] priv = new byte[32];
        System.arraycopy(keyPair, 33, priv, 0, 32);
        return priv;
    }
}
