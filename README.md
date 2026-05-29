# whatsapp-rust-bridge

High-performance WhatsApp utilities powered by Rust — available as **WebAssembly** (Node.js / browser), **native C library**, and **Java/JNI** (Android / JVM).

```
┌─────────────────────────────────────────────────────┐
│                   Rust core (src/)                  │
│  crypto · curve25519 · signal · binary · noise      │
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
| Noise handshake (XX pattern) | ✅ |
| Audio (waveform, duration) | ✅ |
| Image (thumbnails, conversion) | ✅ |
| Sticker metadata (WebP EXIF) | ✅ |

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
├── src/              Rust source — WASM build
│   ├── crypto.rs     AES, HMAC, HKDF, SHA, MD5
│   ├── curve.rs      Curve25519
│   ├── binary.rs     WhatsApp binary protocol
│   ├── noise_session.rs  Noise XX handshake
│   ├── session_builder.rs / session_cipher.rs
│   ├── key_helper.rs
│   ├── appstate.rs
│   └── ...
├── native/           Native C + JNI build
│   ├── src/ffi/      C-ABI exports (wa_* functions)
│   ├── src/jni_bridge/  JNI exports
│   ├── java/         Java wrapper classes
│   └── include/      whatsapp_bridge.h
├── ts/               TypeScript entry point
├── dist/             TypeScript build output
├── pkg/              wasm-pack output
├── test/             Bun tests
├── benches/          Mitata benchmarks
├── build-all.ps1     Master build script
└── docs/             Extended documentation
```



## License

MIT
