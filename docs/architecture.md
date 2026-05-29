# Architecture

## Overview

```
┌──────────────────────────────────────────────────────────────┐
│                     Rust core  (src/)                        │
│                                                              │
│  crypto.rs   curve.rs   binary.rs   noise_session.rs         │
│  key_helper.rs   session_builder/cipher.rs   appstate.rs     │
│  group_cipher.rs   storage_adapter.rs   logger.rs            │
│                                                              │
│  Dependencies (git — jlucaso1/whatsapp-rust)                 │
│    wacore-binary   wacore-libsignal   wacore-noise           │
│    wacore-appstate   waproto                                  │
└──────────────┬─────────────────┬───────────────┬────────────┘
               │                 │               │
  wasm32-unknown-unknown   x86_64-windows   aarch64-linux (etc.)
      wasm_bindgen           cdylib            cdylib + jni
               │                 │               │
       ┌───────▼────────┐  ┌─────▼──────┐ ┌─────▼──────────┐
       │  pkg/  dist/   │  │  .dll/.so  │ │  .dll/.so      │
       │  JS + WASM     │  │  + .h      │ │  Java_* symbols│
       │  TypeScript     │  │            │ │  .java wrappers│
       └────────────────┘  └────────────┘ └────────────────┘
```

## Crate layout

### Root crate (`Cargo.toml`)

Compiled to `wasm32-unknown-unknown` with `wasm_bindgen`. Every public function is annotated with `#[wasm_bindgen]` or `#[wasm_bindgen(js_name = camelCase)]`.

Key source files:

| File | Responsibility |
|---|---|
| `src/lib.rs` | Re-exports, `getWAConnHeader`, `getEnabledFeatures` |
| `src/crypto.rs` | AES-GCM/CBC/CTR, HMAC-SHA256, HKDF, SHA-256, MD5 |
| `src/curve.rs` | Curve25519 keygen, DH, sign, verify |
| `src/key_helper.rs` | Pre-keys, signed pre-keys, registration ID |
| `src/binary.rs` | WhatsApp binary protocol encode/decode (`InternalBinaryNode`) |
| `src/noise_session.rs` | Noise XX handshake + framing |
| `src/session_builder.rs` | Signal session establishment |
| `src/session_cipher.rs` | Signal message encrypt/decrypt |
| `src/group_cipher.rs` | Group messaging (sender-key) |
| `src/appstate.rs` | App-state sync (LTHash, patch/snapshot MACs) |
| `src/storage_adapter.rs` | JS-side Signal storage adapter |
| `src/logger.rs` | Bridged logger |

### Native crate (`native/Cargo.toml`)

Compiled to `cdylib` for the host platform (or a cross-target). No `wasm_bindgen` — pure C ABI.

| Directory | Responsibility |
|---|---|
| `native/src/ffi/` | `#[unsafe(no_mangle)] extern "C"` exports (`wa_*`) |
| `native/src/jni_bridge/` | `extern "system"` JNI exports (`Java_*`) |
| `native/include/` | `whatsapp_bridge.h` — C header |
| `native/java/` | Java wrapper classes + `NativeLoader` |

The JNI build is gated behind the `jni` feature flag in `native/Cargo.toml`.

---

## WASM / JS layer

### Initialization

`dist/index.js` (produced by `ts/index.ts`) embeds the WASM bytes as a base64 string via a build-time macro in `dist/macro.js`. On `require()` / `import`, the WASM is decoded and instantiated synchronously with `initSync`. No async init step is needed by consumers.

```
require("whatsapp-rust-bridge-baron")
  └─ dist/index.js
       ├─ macro.js          base64-encoded .wasm bytes
       └─ pkg/whatsapp_rust_bridge.js   wasm-bindgen glue
            └─ pkg/whatsapp_rust_bridge_bg.wasm
```

### Zero-copy binary decoding

`InternalBinaryNode` (Rust) keeps a reference-counted `Rc<[u8]>` owner of the unpacked wire bytes. Child nodes are `NodeRef<'static>` views into that buffer created via `unsafe mem::transmute`. This avoids cloning the payload on every attribute / content access.

JS getters are lazily cached in `UnsafeCell<Option<...>>` fields — safe because WASM is single-threaded.

---

## Native / C layer

### Memory model

All C functions write into caller-provided output buffers. The convention is:

```c
int32_t wa_foo(
    const uint8_t* in,  size_t in_len,
    uint8_t*       out, size_t* out_len   // [in] capacity, [out] bytes written
);
```

`WA_ERR_OUTPUT_SMALL` is returned when `*out_len < required`. The required size is written into `*out_len` so callers can retry with a correctly-sized buffer.

No heap allocations are returned to the caller. All output lives in the caller's buffer.

### JNI layer

JNI exports mirror the C exports 1:1 but use JNI types (`jbyteArray`, `jint`, etc.). Multi-value returns (key pair, signed pre-key) are packed into a single `byte[]`:

| Return | Layout |
|---|---|
| `generateKeyPair()` | `[33 pubKey \| 32 privKey]` = 65 bytes |
| `generatePreKey()` | `[33 pubKey \| 32 privKey]` = 65 bytes |
| `generateSignedPreKey()` | `[33 pubKey \| 32 privKey \| 64 sig]` = 129 bytes |

Helper methods (`pubKeyFrom`, `privKeyFrom`, `signatureFrom`) extract the fields by offset.

---

## Dependency graph

```
whatsapp-rust-bridge (WASM)
├── wacore-binary      binary protocol encode/decode
├── wacore-libsignal   Signal protocol (curve, sessions)
├── wacore-noise       Noise XX handshake
├── wacore-appstate    app-state sync
├── waproto            protobuf types
├── aes / aes-gcm / cbc / ctr / hkdf / hmac / sha2 / md5
├── rand / curve25519-dalek
├── wasm-bindgen / js-sys / tsify-next
└── serde / serde_bytes / serde_json

whatsapp-native-bridge (native)
├── wacore-binary / wacore-libsignal / wacore-appstate
├── aes / aes-gcm / cbc / ctr / hkdf / hmac / sha2 / md5
├── rand / rand_core / curve25519-dalek
├── base64 / compact_str / serde / serde_json
└── jni  (optional — feature = "jni")
```

---

## Build pipeline

```
wasm-pack build
  └─ cargo build --target wasm32-unknown-unknown --release
       └─ produces pkg/*.wasm + pkg/*.js + pkg/*.d.ts

npm run build:ts
  └─ tsc ts/index.ts → dist/index.js + dist/*.d.ts

cargo build --release --target <host>
  └─ produces native/target/<triple>/release/whatsapp_bridge.{dll,so,dylib}

cargo build --release --target <host> --features jni
  └─ produces the same library with additional Java_* symbols
     build-all.ps1 copies it to release-jni/ to avoid overwriting the C build
```
