package com.whatsapp.bridge;

/**
 * WhatsApp binary node encode/decode backed by Rust via JNI.
 *
 * JSON schema for BinaryNode:
 * <pre>
 * {
 *   "tag": "...",
 *   "attrs": { "key": "value" },
 *   "content": "text"
 *            | [BinaryNode, ...]
 *            | { "b64": "<base64 bytes>" }
 * }
 * </pre>
 *
 * The "content" field is optional. Binary payloads must be base64-encoded
 * inside the { "b64": "..." } wrapper.
 */
public final class Binary {

    private Binary() {}

    /**
     * Encode a BinaryNode (JSON) into WhatsApp wire-format bytes.
     * @param jsonBytes UTF-8 JSON input matching the schema above
     * @return wire-format byte array
     */
    public static native byte[] encodeNode(byte[] jsonBytes);

    /**
     * Decode WhatsApp wire-format bytes into a BinaryNode (JSON).
     * @param data wire-format bytes
     * @return UTF-8 JSON bytes matching the schema above
     */
    public static native byte[] decodeNode(byte[] data);

    /** Convenience overload: encode from a JSON string. */
    public static byte[] encodeNode(String json) {
        return encodeNode(json.getBytes(java.nio.charset.StandardCharsets.UTF_8));
    }

    /** Convenience overload: decode to a JSON string. */
    public static String decodeNodeAsString(byte[] data) {
        byte[] json = decodeNode(data);
        return new String(json, java.nio.charset.StandardCharsets.UTF_8);
    }
}
