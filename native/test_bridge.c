#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "include/whatsapp_bridge.h"

static void print_hex(const char* label, const uint8_t* buf, size_t len) {
    printf("%s: ", label);
    for (size_t i = 0; i < len; i++) printf("%02x", buf[i]);
    printf("\n");
}

static int test_sha256() {
    const uint8_t input[] = "hello";
    uint8_t out[32];
    size_t out_len = sizeof(out);
    int rc = wa_sha256(input, 5, out, &out_len);
    if (rc != WA_OK || out_len != 32) { printf("FAIL sha256 (rc=%d)\n", rc); return 0; }
    /* known SHA-256("hello") */
    const uint8_t expected[32] = {
        0x2c,0xf2,0x4d,0xba,0x5f,0xb0,0xa3,0x0e,0x26,0xe8,0x3b,0x2a,0xc5,0xb9,0xe2,0x9e,
        0x1b,0x16,0x1e,0x5c,0x1f,0xa7,0x42,0x5e,0x73,0x04,0x33,0x62,0x93,0x8b,0x98,0x24
    };
    if (memcmp(out, expected, 32) != 0) { printf("FAIL sha256 wrong result\n"); return 0; }
    print_hex("sha256(hello)", out, 32);
    return 1;
}

static int test_md5() {
    const uint8_t input[] = "hello";
    uint8_t out[16];
    size_t out_len = sizeof(out);
    int rc = wa_md5(input, 5, out, &out_len);
    if (rc != WA_OK || out_len != 16) { printf("FAIL md5 (rc=%d)\n", rc); return 0; }
    print_hex("md5(hello)", out, 16);
    return 1;
}

static int test_hmac() {
    const uint8_t data[] = "hello";
    const uint8_t key[]  = "secret";
    uint8_t out[32];
    size_t out_len = sizeof(out);
    int rc = wa_hmac_sha256(data, 5, key, 6, out, &out_len);
    if (rc != WA_OK || out_len != 32) { printf("FAIL hmac (rc=%d)\n", rc); return 0; }
    print_hex("hmac-sha256", out, 32);
    return 1;
}

static int test_hkdf() {
    const uint8_t ikm[]  = "input key material";
    const uint8_t info[] = "WhatsApp test";
    uint8_t out[32];
    int rc = wa_hkdf(ikm, 18, NULL, 0, info, 13, out, 32);
    if (rc != WA_OK) { printf("FAIL hkdf (rc=%d)\n", rc); return 0; }
    print_hex("hkdf(32)", out, 32);
    return 1;
}

static int test_aes_gcm() {
    uint8_t key[32] = {0};
    uint8_t iv[12]  = {0};
    const uint8_t pt[] = "WhatsApp test message";
    uint8_t ct[256];
    size_t ct_len = sizeof(ct);

    int rc = wa_aes_encrypt_gcm(pt, sizeof(pt)-1, key, iv, NULL, 0, ct, &ct_len);
    if (rc != WA_OK) { printf("FAIL aes-gcm encrypt (rc=%d)\n", rc); return 0; }
    printf("aes-gcm ct_len=%zu\n", ct_len);

    uint8_t pt2[256];
    size_t pt2_len = sizeof(pt2);
    rc = wa_aes_decrypt_gcm(ct, ct_len, key, iv, NULL, 0, pt2, &pt2_len);
    if (rc != WA_OK) { printf("FAIL aes-gcm decrypt (rc=%d)\n", rc); return 0; }
    if (pt2_len != sizeof(pt)-1 || memcmp(pt, pt2, pt2_len) != 0) {
        printf("FAIL aes-gcm roundtrip mismatch\n"); return 0;
    }
    printf("aes-gcm roundtrip OK\n");
    return 1;
}

static int test_aes_cbc() {
    uint8_t key[32] = {0};
    const uint8_t pt[] = "CBC test payload!";
    uint8_t ct[256];
    size_t ct_len = sizeof(ct);

    int rc = wa_aes_encrypt_cbc(pt, sizeof(pt)-1, key, ct, &ct_len);
    if (rc != WA_OK) { printf("FAIL aes-cbc encrypt (rc=%d)\n", rc); return 0; }
    printf("aes-cbc ct_len=%zu\n", ct_len);

    uint8_t pt2[256];
    size_t pt2_len = sizeof(pt2);
    rc = wa_aes_decrypt_cbc(ct, ct_len, key, pt2, &pt2_len);
    if (rc != WA_OK) { printf("FAIL aes-cbc decrypt (rc=%d)\n", rc); return 0; }
    if (pt2_len != sizeof(pt)-1 || memcmp(pt, pt2, pt2_len) != 0) {
        printf("FAIL aes-cbc roundtrip mismatch\n"); return 0;
    }
    printf("aes-cbc roundtrip OK\n");
    return 1;
}

static int test_curve() {
    uint8_t pub1[33], priv1[32];
    uint8_t pub2[33], priv2[32];

    if (wa_generate_key_pair(pub1, priv1) != WA_OK) { printf("FAIL keygen 1\n"); return 0; }
    if (wa_generate_key_pair(pub2, priv2) != WA_OK) { printf("FAIL keygen 2\n"); return 0; }

    uint8_t shared1[32], shared2[32];
    if (wa_calculate_agreement(pub2, 33, priv1, shared1) != WA_OK) { printf("FAIL dh 1\n"); return 0; }
    if (wa_calculate_agreement(pub1, 33, priv2, shared2) != WA_OK) { printf("FAIL dh 2\n"); return 0; }

    if (memcmp(shared1, shared2, 32) != 0) { printf("FAIL DH shared secret mismatch\n"); return 0; }
    print_hex("dh shared secret", shared1, 32);

    uint8_t sig[64];
    const uint8_t msg[] = "test message";
    if (wa_calculate_signature(priv1, msg, 12, sig) != WA_OK) { printf("FAIL sign\n"); return 0; }

    int valid = wa_verify_signature(pub1, 33, msg, 12, sig);
    if (valid != 1) { printf("FAIL verify (got %d)\n", valid); return 0; }
    printf("curve25519 sign+verify OK\n");

    /* tamper with sig */
    sig[0] ^= 0xFF;
    int invalid = wa_verify_signature(pub1, 33, msg, 12, sig);
    if (invalid != 0) { printf("FAIL tampered sig not rejected\n"); return 0; }
    printf("curve25519 tamper-detection OK\n");

    return 1;
}

static int test_key_helper() {
    uint32_t reg_id = 0;
    if (wa_generate_registration_id(&reg_id) != WA_OK) { printf("FAIL reg_id\n"); return 0; }
    if (reg_id > 16383) { printf("FAIL reg_id out of range: %u\n", reg_id); return 0; }
    printf("registration_id=%u\n", reg_id);

    uint8_t pub[33], priv[32];
    uint32_t kid;
    if (wa_generate_pre_key(42, pub, priv, &kid) != WA_OK) { printf("FAIL pre_key\n"); return 0; }
    if (kid != 42) { printf("FAIL pre_key id mismatch\n"); return 0; }
    printf("pre_key id=%u OK\n", kid);

    /* identity key pair for signed pre key */
    uint8_t id_pub[33], id_priv[32];
    wa_generate_key_pair(id_pub, id_priv);

    uint8_t spk_pub[33], spk_priv[32], spk_sig[64];
    uint32_t spk_id;
    if (wa_generate_signed_pre_key(id_priv, 7, spk_pub, spk_priv, spk_sig, &spk_id) != WA_OK) {
        printf("FAIL signed_pre_key\n"); return 0;
    }
    if (spk_id != 7) { printf("FAIL signed_pre_key id mismatch\n"); return 0; }

    /* verify the signature */
    int ok = wa_verify_signature(id_pub, 33, spk_pub, 33, spk_sig);
    if (ok != 1) { printf("FAIL signed_pre_key signature invalid\n"); return 0; }
    printf("signed_pre_key id=%u signature OK\n", spk_id);

    return 1;
}

static int test_binary_roundtrip() {
    const char* json_in = "{\"tag\":\"presence\",\"attrs\":{\"type\":\"available\"},\"content\":null}";
    /* skip null content — use minimal node */
    const char* json2 = "{\"tag\":\"iq\",\"attrs\":{\"id\":\"123\",\"type\":\"get\"}}";

    uint8_t encoded[4096];
    size_t enc_len = sizeof(encoded);
    int rc = wa_encode_node((const uint8_t*)json2, strlen(json2), encoded, &enc_len);
    if (rc != WA_OK) { printf("FAIL encode_node (rc=%d)\n", rc); return 0; }
    printf("encode_node enc_len=%zu\n", enc_len);

    uint8_t decoded_json[4096];
    size_t dec_len = sizeof(decoded_json);
    rc = wa_decode_node(encoded, enc_len, decoded_json, &dec_len);
    if (rc != WA_OK) { printf("FAIL decode_node (rc=%d)\n", rc); return 0; }
    decoded_json[dec_len] = '\0';
    printf("decode_node: %s\n", decoded_json);

    (void)json_in;
    return 1;
}

int main(void) {
    int passed = 0, total = 0;

    #define RUN(fn) do { total++; if (fn()) { printf("[PASS] " #fn "\n"); passed++; } else printf("[FAIL] " #fn "\n"); } while(0)

    RUN(test_sha256);
    RUN(test_md5);
    RUN(test_hmac);
    RUN(test_hkdf);
    RUN(test_aes_gcm);
    RUN(test_aes_cbc);
    RUN(test_curve);
    RUN(test_key_helper);
    RUN(test_binary_roundtrip);

    printf("\n%d/%d tests passed\n", passed, total);
    return passed == total ? 0 : 1;
}
