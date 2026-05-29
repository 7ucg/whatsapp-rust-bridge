# Building

## Prerequisites

| Tool | Purpose | Install |
|---|---|---|
| Rust (nightly) | Core compiler | `rustup install nightly` |
| wasm-pack | WASM build | `cargo install wasm-pack` |
| Node.js / npm | TypeScript build | nodejs.org |
| cbindgen *(optional)* | Regenerate C header | `cargo install cbindgen` |
| cargo-ndk *(optional)* | Android cross-compile | `cargo install cargo-ndk` |

The `wasm32-unknown-unknown` target is installed automatically by wasm-pack.  
The host native target (`x86_64-pc-windows-msvc`, etc.) must be installed:

```powershell
rustup target add x86_64-pc-windows-msvc   # Windows
rustup target add x86_64-unknown-linux-gnu  # Linux
rustup target add aarch64-apple-darwin      # macOS arm64
```

## Build script

```powershell
# Full release build (WASM + C + JNI)
.\build-all.ps1

# Flags
.\build-all.ps1 -Debug             # C/JNI debug profile
.\build-all.ps1 -SkipWasm          # skip WASM/TS
.\build-all.ps1 -SkipC             # skip C-only build
.\build-all.ps1 -SkipJni           # skip JNI build
.\build-all.ps1 -Target <triple>   # cross-compile
```

## Manual builds

### WASM + TypeScript

```powershell
npm install
npm run build
```

Outputs:
- `pkg/` — wasm-pack output (WASM + JS glue + `.d.ts`)
- `dist/` — TypeScript compiled output

### C native

```powershell
cd native
cargo build --release --target x86_64-pc-windows-msvc
```

Output: `native/target/<triple>/release/whatsapp_bridge.dll` (or `.so` / `.dylib`)

### Java / JNI

```powershell
cd native
cargo build --release --target x86_64-pc-windows-msvc --features jni
```

The JNI build produces the same `.dll`/`.so` but with `Java_com_whatsapp_bridge_*` symbols.  
The build script copies it to `native/target/<triple>/release-jni/` to avoid overwriting the C-only binary.

### Regenerate C header (optional)

```powershell
cbindgen --config native/cbindgen.toml --crate whatsapp-native-bridge --output native/include/whatsapp_bridge.h
```

The header is already committed and kept in sync manually.

## Cross-compilation targets

| Platform | Target triple | Notes |
|---|---|---|
| Windows x64 | `x86_64-pc-windows-msvc` | default on Windows |
| Windows arm64 | `aarch64-pc-windows-msvc` | |
| Linux x64 | `x86_64-unknown-linux-gnu` | |
| Linux arm64 | `aarch64-unknown-linux-gnu` | needs `gcc-aarch64-linux-gnu` |
| macOS x64 | `x86_64-apple-darwin` | |
| macOS arm64 | `aarch64-apple-darwin` | default on Apple Silicon |
| Android arm64 | `aarch64-linux-android` | needs NDK + cargo-ndk |
| Android x86_64 | `x86_64-linux-android` | emulator / x86 device |

### Android example (cargo-ndk)

```bash
# Install NDK via Android Studio or sdkmanager
export ANDROID_NDK_HOME=/path/to/ndk
cargo ndk -t arm64-v8a -t x86_64 -o ./android/jniLibs build --release --features jni --manifest-path native/Cargo.toml
```

## Tests

```powershell
# WASM/TS tests (Bun)
bun test

# Native Rust tests
cd native
cargo test --target x86_64-pc-windows-msvc
```

## Benchmarks

```powershell
npm run bench
```
