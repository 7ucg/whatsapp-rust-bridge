# whatsapp-rust-bridge

High-performance WhatsApp utilities powered by Rust — available as **WebAssembly** (Node.js / browser), **native C library**, and **Java/JNI** (Android / JVM).

```
┌─────────────────────────────────────────────────────┐
│                   Rust core (src/)                  │
│  crypto · curve25519 · signal · binary · noise · voip│
└──────────┬─────────────────┬───────────────┬────────┘
           │                 │               │
     wasm32 target      cdylib target   cdylib + jni
           │                 │               │
    ┌──────▼──────┐  ┌───────▼──────┐ ┌─────▼──────┐
    │ TypeScript  │  │      C       │ │  Java/JNI  │
    │  (Node.js)  │  │  (.h + .dll) │ │ (.java)    │
    └─────────────┘  └──────────────┘ └────────────┘
```

## Features

| Feature | Status |
|---|---|
| Binary protocol (encode / decode) | ✅ |
| Curve25519 (DH, sign, verify) | ✅ |
| AES-256-GCM / CBC / CTR | ✅ |
| HMAC-SHA256 / SHA-256 / MD5 / HKDF | ✅ |
| LibSignal (session, pre-keys, group cipher) | ✅ |
| App State Sync (LTHash, patch/snapshot MAC) | ✅ |
| Noise handshake (XX + IK + XXfallback) | ✅ |
| JID parse / encode / inspect / construct | ✅ |
| VoIP — MLow audio codec (encode / decode) | ✅ |
| VoIP — E2E SRTP media pipeline (protect / unprotect) | ✅ |
| VoIP — sans-io CallEngine (signaling + media driver) | ✅ |
| Audio (waveform, duration) | ✅ (optional feature) |
| Image (thumbnails, conversion) | ✅ (optional feature) |
| Sticker metadata (WebP EXIF) | ✅ (optional feature) |

## Bridges

| Target | Entry point | Docs |
|---|---|---|
| TypeScript / Node.js | `dist/index.js` | [docs/usage-typescript.md](docs/usage-typescript.md) |
| C / C++ | `native/include/whatsapp_bridge.h` | [docs/usage-c.md](docs/usage-c.md) |
| Java / Kotlin / Android | `native/java/com/whatsapp/bridge/` | [docs/usage-java.md](docs/usage-java.md) |

## Build

```powershell
# All three targets (release)
.\build-all.ps1

# Individual targets
.\build-all.ps1 -SkipC -SkipJni     # WASM + TypeScript only
.\build-all.ps1 -SkipWasm           # C + JNI only
.\build-all.ps1 -SkipWasm -SkipJni  # C only
.\build-all.ps1 -SkipWasm -SkipC    # JNI only

# Cross-compile
.\build-all.ps1 -Target aarch64-unknown-linux-gnu
```

See [docs/building.md](docs/building.md) for full build instructions and cross-compilation targets.

## Quick Start

**TypeScript / Node.js**
```ts
const { generateKeyPair, aesEncryptGCM, sha256 } = require("whatsapp-rust-bridge-baron");

const kp   = generateKeyPair();
const hash = sha256(Buffer.from("hello"));
```

**C**
```c
#include "native/include/whatsapp_bridge.h"

uint8_t pub[33], priv[32];
wa_generate_key_pair(pub, priv);
```

**Java**
```java
NativeLoader.load();
byte[] kp = Curve.generateKeyPair();
```

## Project layout

```
whatsapp-rust-bridge/
├── src/                  Rust source — WASM build (wasm-bindgen)
│   ├── crypto.rs         AES, HMAC, HKDF, SHA-256, MD5
│   ├── curve.rs          Curve25519 keygen, DH, sign, verify
│   ├── binary.rs         WhatsApp binary protocol (encode/decode)
│   ├── jid.rs            JID parse/encode/construct/inspect
│   ├── noise_session.rs  Noise XX, IK, XXfallback handshake + framing
│   ├── session_builder.rs / session_cipher.rs   Signal 1-to-1
│   ├── group_cipher.rs   Signal group messaging
│   ├── key_helper.rs     Pre-key / signed-pre-key / registration ID
│   ├── appstate.rs       App-state sync, LTHash, MACs
│   ├── voip.rs           Calls: MLow codec, E2E SRTP, sans-io CallEngine
│   ├── storage_adapter.rs  JS ↔ Signal storage bridge
│   ├── audio.rs          Waveform + duration (feature = audio)
│   ├── image_utils.rs    Thumbnails (feature = image)
│   └── sticker_metadata.rs  WebP EXIF (feature = sticker)
├── internal/             Self-contained protocol crates
│   ├── wacore            Full WhatsApp core + VoIP engine (vendored; client
│   │                     parts unused, voip exposed via src/voip.rs)
│   ├── wacore/appstate   Key expansion, LTHash, patch encode/decode
│   ├── wacore/binary     Binary protocol, JID types, token maps
│   ├── wacore/libsignal  Signal protocol (sessions, group, ratchet)
│   ├── wacore/noise      Noise XX/IK/XXfallback, frame codec
│   ├── wacore/derive     Proc-macro helpers
│   └── waproto           Protobuf definitions (WA 2.3000.x) + tags + codec
├── native/               Native C + JNI build
│   ├── src/ffi/          C-ABI exports (wa_* functions)
│   ├── src/jni_bridge/   JNI exports (Java_* symbols)
│   ├── java/             Java wrapper classes + NativeLoader
│   └── include/          whatsapp_bridge.h
├── ts/                   TypeScript entry point + macro
├── dist/                 TypeScript compiled output
├── pkg/                  wasm-pack output (WASM + JS glue + .d.ts)
├── test/                 Bun tests
├── build-all.ps1         Master build script (WASM + C + JNI)
└── docs/                 Extended documentation
```



## License

MIT
