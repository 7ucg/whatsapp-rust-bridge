# Architecture

## Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                        Rust workspace                            │
│                                                                  │
│  src/          WASM bindings (wasm_bindgen) — public JS/TS API  │
│                crypto, curve, binary, noise, signal, jid, ...   │
│                                                                  │
│  internal/     Protocol crates (self-contained, no git deps)    │
│    wacore/appstate  wacore/binary  wacore/libsignal              │
│    wacore/noise     wacore/derive  waproto                       │
│                                                                  │
│  native/       C-ABI + JNI bridge                               │
└──────────────┬─────────────────┬───────────────┬────────────────┘
               │                 │               │
  wasm32-unknown-unknown   x86_64-windows   aarch64-linux (etc.)
      wasm_bindgen           cdylib            cdylib + jni
               │                 │               │
       ┌───────▼────────┐  ┌─────▼──────┐ ┌─────▼──────────┐
       │  pkg/  dist/   │  │  .dll/.so  │ │  .dll/.so      │
       │  JS + WASM     │  │  + .h      │ │  Java_* symbols│
       │  TypeScript    │  │            │ │  .java wrappers│
       └────────────────┘  └────────────┘ └────────────────┘
```

## Crate layout

### Root package (`src/` + `Cargo.toml`)

Compiled to `wasm32-unknown-unknown` with `wasm_bindgen`. Every public function is annotated with `#[wasm_bindgen]`.

| File | Responsibility |
|---|---|
| `src/lib.rs` | Re-exports, `getWAConnHeader`, `getEnabledFeatures` |
| `src/crypto.rs` | AES-GCM/CBC/CTR, HMAC-SHA256, HKDF, SHA-256, MD5 |
| `src/curve.rs` | Curve25519 keygen, DH, sign, verify |
| `src/jid.rs` | JID parse/encode/construct/inspect (full API) |
| `src/key_helper.rs` | Pre-keys, signed pre-keys, registration ID |
| `src/binary.rs` | WhatsApp binary protocol encode/decode (`InternalBinaryNode`) |
| `src/noise_session.rs` | Noise XX, IK, XXfallback handshake + framing |
| `src/session_builder.rs` | Signal session establishment |
| `src/session_cipher.rs` | Signal message encrypt/decrypt |
| `src/group_cipher.rs` | Group messaging (sender-key) |
| `src/appstate.rs` | App-state sync: expand keys, encode/decode records, LTHash, MACs |
| `src/storage_adapter.rs` | JS-side Signal storage bridge (all traits incl. `sender_key_lock`) |
| `src/protocol_address.rs` | `ProtocolAddress` |
| `src/sender_key_name.rs` | `SenderKeyName` |
| `src/session_record.rs` | `SessionRecord` |
| `src/group_types.rs` | `SenderKeyRecord`, `SenderKeyDistributionMessage` |
| `src/logger.rs` | Bridged logger |
| `src/audio.rs` | Waveform + duration (feature = `audio`) |
| `src/image_utils.rs` | Thumbnails (feature = `image`) |
| `src/sticker_metadata.rs` | WebP EXIF (feature = `sticker`) |

### Internal crates (`internal/`)

Self-contained protocol implementations — no external git dependencies.

| Crate | Responsibility |
|---|---|
| `wacore-binary` | WhatsApp binary protocol, JID types, constants, build-time token maps |
| `wacore-libsignal` | Signal protocol: sessions, pre-keys, group cipher, ratchet, crypto primitives |
| `wacore-noise` | Noise XX / IK / XXfallback handshake, frame encoder/decoder, edge routing |
| `wacore-appstate` | App-state sync: key expansion, LTHash, patch/snapshot encode+decode+validate |
| `wacore-derive` | Internal proc-macro helpers |
| `waproto` | Protobuf definitions (WhatsApp Version 2.3000.1040457520) |

### Native crate (`native/`)

Compiled to `cdylib` for the host platform or any cross-target.

| Directory | Responsibility |
|---|---|
| `native/src/ffi/` | `extern "C"` exports (`wa_*`) — C ABI |
| `native/src/jni_bridge/` | `extern "system"` JNI exports (`Java_*`) — gated by `--features jni` |
| `native/include/` | `whatsapp_bridge.h` — C header |
| `native/java/` | Java wrapper classes + `NativeLoader` |

---

## WASM / JS layer

### Initialization

`dist/index.js` embeds the WASM bytes as base64 at build time via `dist/macro.js`. On `require()` / `import`, `initSync` is called automatically — no async init step needed by consumers.

### Zero-copy binary decoding

`InternalBinaryNode` keeps a reference-counted `Rc<[u8]>` owner. Child nodes are `NodeRef<'static>` views via `unsafe mem::transmute`. Getters are lazily cached in `UnsafeCell` (safe — WASM is single-threaded).

---

## Proto regeneration

```powershell
cargo build -p waproto --features generate
```

- Requires `protoc-bin-vendored` (bundled via Cargo)
- Proto uses `syntax = "proto2"` — WhatsApp enums don't start at 0 (proto3 rejects this)
- `build.rs` renames the generated `proto.rs` → `whatsapp.rs`
- Field attribute paths use `.proto.` prefix (matches `package proto;` in the `.proto` file)

---

## Build pipeline

```
.\build-all.ps1           # all three targets
  -SkipWasm               # C + JNI only
  -SkipC -SkipJni         # WASM/TS only
  -Target <triple>        # cross-compile
  -Debug                  # debug profile for native

wasm-pack build → pkg/    wasm + JS glue
tsc → dist/               TypeScript
cargo build → dist-native/c/    C library
cargo build --features jni → dist-native/jni/   JNI library
```

---

## Dependency graph

```
waproto ◄─────────────────────────────────────────┐
   ▲                                               │
   │                                               │
wacore-binary ──► wacore-appstate                  │
   │                    ▲                          │
   │                    │                          │
   └──► wacore-noise ───┘                          │
              │                                    │
              └──► wacore-libsignal ───────────────┘
```

All internal crates are `wasm32`-only dependencies of `wacore-noise`; the root crate and `wacore-libsignal` are usable on both native and wasm32.
