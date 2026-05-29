# Usage — C / C++

The C bridge is a native shared library (`whatsapp_bridge.dll` / `libwhatsapp_bridge.so` / `libwhatsapp_bridge.dylib`) with a stable C ABI.

## Setup

1. Build the library (see [building.md](building.md)):
   ```powershell
   .\build-all.ps1 -SkipWasm -SkipJni
   ```

2. Copy the output files to your project:
   ```
   native/include/whatsapp_bridge.h         → include/
   native/target/<triple>/release/*.dll     → lib/
   ```

3. Compile your C code:
   ```bash
   # GCC / Clang (Linux / macOS)
   gcc main.c -I./include -L./lib -lwhatsapp_bridge -o app

   # MSVC (Windows)
   cl main.c /I include whatsapp_bridge.dll.lib /Fe:app.exe
   ```

---

## Error codes

Every function returns `int32_t`. Zero means success; negative values are errors.

```c
#define WA_OK                 0
#define WA_ERR_NULL_POINTER  -1
#define WA_ERR_BAD_KEY_LEN   -2
#define WA_ERR_BAD_IV_LEN    -3
#define WA_ERR_ENCRYPT_FAIL  -4
#define WA_ERR_DECRYPT_FAIL  -5
#define WA_ERR_OUTPUT_SMALL  -6   // out_len was too small; *out_len set to required size
#define WA_ERR_HKDF_FAIL     -7
#define WA_ERR_RNG_FAIL      -8
```

**Two-pass pattern** for `WA_ERR_OUTPUT_SMALL`:

```c
size_t needed = 0;
wa_sha256(data, data_len, NULL, &needed);    // pass NULL to query size
uint8_t* out = malloc(needed);
wa_sha256(data, data_len, out, &needed);
```

---

## Hashing

### SHA-256

```c
uint8_t hash[32];
size_t  hash_len = sizeof(hash);

int rc = wa_sha256(data, data_len, hash, &hash_len);
// hash_len == 32 on success
```

### MD5

```c
uint8_t digest[16];
size_t  digest_len = sizeof(digest);

wa_md5(data, data_len, digest, &digest_len);
```

### HMAC-SHA256

```c
uint8_t mac[32];
size_t  mac_len = sizeof(mac);

wa_hmac_sha256(data, data_len, key, key_len, mac, &mac_len);
```

### HKDF-SHA256

```c
uint8_t okm[80];

// salt and info are optional — pass NULL / 0 to omit
int rc = wa_hkdf(
    ikm,  ikm_len,
    salt, salt_len,   // NULL, 0  for no salt
    info, info_len,   // NULL, 0  for empty info
    okm,  80          // out_buf, desired length
);
```

---

## AES-256-GCM

```c
// --- Encrypt ---
// out_buf must be >= plaintext_len + 16 (tag)
uint8_t ct[1024];
size_t  ct_len = sizeof(ct);

int rc = wa_aes_encrypt_gcm(
    plaintext, pt_len,
    key32,          // 32 bytes
    iv12,           // 12 bytes
    aad, aad_len,   // NULL, 0  for no AAD
    ct, &ct_len
);
// ct_len == pt_len + 16 on success

// --- Decrypt ---
uint8_t pt[1024];
size_t  pt_len = sizeof(pt);

rc = wa_aes_decrypt_gcm(
    ct, ct_len,
    key32, iv12,
    aad, aad_len,
    pt, &pt_len
);
```

---

## AES-256-CBC

```c
// --- Encrypt (random IV, prepended to output) ---
// out_buf must be >= plaintext_len + 32  (16 IV + up to 16 padding)
uint8_t ct[1024];
size_t  ct_len = sizeof(ct);

wa_aes_encrypt_cbc(plaintext, pt_len, key32, ct, &ct_len);
// ct[0..15] = IV, ct[16..] = ciphertext

// --- Decrypt (IV is first 16 bytes of ciphertext) ---
uint8_t pt[1024];
size_t  pt_len = sizeof(pt);

wa_aes_decrypt_cbc(ct, ct_len, key32, pt, &pt_len);

// --- Encrypt with explicit IV ---
wa_aes_encrypt_cbc_iv(plaintext, pt_len, key32, iv16, ct, &ct_len);

// --- Decrypt with explicit IV ---
wa_aes_decrypt_cbc_iv(ct, ct_len, key32, iv16, pt, &pt_len);
```

---

## AES-256-CTR

```c
// Symmetric: same function for encrypt and decrypt
uint8_t out[1024];
size_t  out_len = sizeof(out);

wa_aes_ctr(data, data_len, key32, iv16, out, &out_len);
```

---

## Curve25519

### Key pair

```c
uint8_t pub[33];   // 0x05 prefix + 32 bytes
uint8_t priv[32];

wa_generate_key_pair(pub, priv);
```

### Diffie-Hellman

```c
uint8_t shared[32];

// pub_key: 32 or 33 bytes (0x05 prefix optional)
wa_calculate_agreement(server_pub, 33, my_priv, shared);
```

### Sign / verify

```c
uint8_t sig[64];
wa_calculate_signature(priv, message, message_len, sig);

// Returns 1 = valid, 0 = invalid, negative = error
int valid = wa_verify_signature(pub, 33, message, message_len, sig);
```

### Derive public key

```c
uint8_t pub[33];
wa_get_public_from_private(priv, pub);
```

---

## Key Helper

```c
// Registration ID (0..16383)
uint32_t reg_id;
wa_generate_registration_id(&reg_id);

// Pre-key
uint8_t  pk_pub[33], pk_priv[32];
uint32_t pk_id;
wa_generate_pre_key(42, pk_pub, pk_priv, &pk_id);

// Signed pre-key
uint8_t  spk_pub[33], spk_priv[32], spk_sig[64];
uint32_t spk_id;
wa_generate_signed_pre_key(
    identity_priv,   // 32-byte identity private key
    1,               // desired key ID
    spk_pub, spk_priv, spk_sig, &spk_id
);
// Verify the signature
int ok = wa_verify_signature(identity_pub, 33, spk_pub, 33, spk_sig);
```

---

## Binary nodes

The binary API uses a JSON-based interchange format so callers don't need to understand the internal WhatsApp binary encoding.

**JSON schema:**
```json
{
  "tag":   "iq",
  "attrs": { "id": "1", "type": "get" },
  "content": "text"
           | [ <child nodes> ]
           | { "b64": "<base64-encoded bytes>" }
}
```
`content` is optional.

```c
// --- Encode (JSON -> wire bytes) ---
const char* json = "{\"tag\":\"iq\",\"attrs\":{\"id\":\"1\",\"type\":\"get\"}}";

uint8_t wire[4096];
size_t  wire_len = sizeof(wire);

int rc = wa_encode_node(
    (const uint8_t*)json, strlen(json),
    wire, &wire_len
);

// --- Decode (wire bytes -> JSON) ---
uint8_t json_out[4096];
size_t  json_len = sizeof(json_out);

rc = wa_decode_node(wire, wire_len, json_out, &json_len);
json_out[json_len] = '\0';   // null-terminate
printf("%s\n", json_out);
```

### Binary content (base64 wrapper)

```c
// Input JSON with binary content:
// { "tag": "media", "attrs": {}, "content": { "b64": "SGVsbG8=" } }
```

---

## Thread safety

All functions are **stateless and thread-safe**. No global mutable state. Pass independent buffers per call.

---

## C++ example

```cpp
#include "whatsapp_bridge.h"
#include <vector>
#include <stdexcept>

std::vector<uint8_t> sha256(const uint8_t* data, size_t len) {
    std::vector<uint8_t> out(32);
    size_t out_len = 32;
    if (wa_sha256(data, len, out.data(), &out_len) != WA_OK)
        throw std::runtime_error("sha256 failed");
    return out;
}

std::pair<std::vector<uint8_t>, std::vector<uint8_t>> generate_key_pair() {
    std::vector<uint8_t> pub(33), priv(32);
    wa_generate_key_pair(pub.data(), priv.data());
    return { pub, priv };
}
```
