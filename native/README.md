# whatsapp-native-bridge

Native C / Java (JNI) bridge for `whatsapp-rust-bridge`.  
Compiled from the same Rust core as the WASM build — no `wasm_bindgen`, pure C ABI.

## Outputs

| Build | Output |
|---|---|
| C (no JNI) | `target/<triple>/release/whatsapp_bridge.{dll,so,dylib}` |
| JNI | `target/<triple>/release-jni/whatsapp_bridge.{dll,so,dylib}` |
| C header | `include/whatsapp_bridge.h` |
| Java sources | `java/com/whatsapp/bridge/*.java` |

## Build

```powershell
# From repo root — recommended
.\build-all.ps1 -SkipWasm          # C + JNI
.\build-all.ps1 -SkipWasm -SkipJni # C only
.\build-all.ps1 -SkipWasm -SkipC   # JNI only

# From this directory — explicit target required
cargo build --release --target x86_64-pc-windows-msvc             # C
cargo build --release --target x86_64-pc-windows-msvc --features jni  # JNI
```

See [../docs/building.md](../docs/building.md) for cross-compilation instructions.

## Usage

- **C / C++** — [../docs/usage-c.md](../docs/usage-c.md)
- **Java / Kotlin / Android** — [../docs/usage-java.md](../docs/usage-java.md)
- **Full API reference** — [../docs/api-reference.md](../docs/api-reference.md)

## Structure

```
native/
├── Cargo.toml          crate manifest (cdylib + staticlib, optional jni feature)
├── .cargo/config.toml  native target config (overrides root wasm32 default)
├── cbindgen.toml       cbindgen config for header generation
├── include/
│   └── whatsapp_bridge.h   C/C++ header (all wa_* functions + error codes)
├── java/com/whatsapp/bridge/
│   ├── Crypto.java     AES-GCM/CBC/CTR, HMAC, HKDF, SHA-256, MD5
│   ├── Curve.java      Curve25519 key gen, DH, sign, verify
│   ├── KeyHelper.java  pre-keys, registration ID
│   ├── Binary.java     WhatsApp binary protocol encode/decode
│   └── NativeLoader.java   System.loadLibrary + JAR-resource fallback
├── src/
│   ├── lib.rs
│   ├── ffi/            C-ABI exports
│   │   ├── mod.rs
│   │   ├── crypto.rs
│   │   ├── curve.rs
│   │   ├── key_helper.rs
│   │   └── binary.rs
│   └── jni_bridge/     JNI exports (compiled only with --features jni)
│       ├── mod.rs
│       ├── crypto.rs
│       ├── curve.rs
│       ├── key_helper.rs
│       └── binary.rs
└── tests/
    └── integration.rs  integration tests
```

## Tests

```powershell
cargo test --target x86_64-pc-windows-msvc
```
