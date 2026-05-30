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
The host native target must be installed:

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

Output: `dist-native/c/whatsapp_bridge.dll` (or `.so` / `.dylib`)

### Java / JNI

```powershell
cd native
cargo build --release --target x86_64-pc-windows-msvc --features jni
```

Output: `dist-native/jni/whatsapp_bridge.dll` + `dist-native/jni/java/...`

## Proto regeneration

When `internal/waproto/src/whatsapp.proto` is updated:

```powershell
cargo build -p waproto --features generate
```

- Uses `protoc-bin-vendored` — no separate protoc install needed
- `build.rs` renames `proto.rs` → `whatsapp.rs` automatically
- Proto syntax is `proto2` (WhatsApp enums don't start at 0)
- After regeneration, check `internal/wacore/libsignal/src/` for breaking field type changes

## Cross-compilation targets

| Platform | Target triple |
|---|---|
| Windows x64 | `x86_64-pc-windows-msvc` |
| Windows arm64 | `aarch64-pc-windows-msvc` |
| Linux x64 | `x86_64-unknown-linux-gnu` |
| Linux arm64 | `aarch64-unknown-linux-gnu` |
| macOS x64 | `x86_64-apple-darwin` |
| macOS arm64 | `aarch64-apple-darwin` |
| Android arm64 | `aarch64-linux-android` (needs NDK + cargo-ndk) |
| Android x86_64 | `x86_64-linux-android` |

### Android (cargo-ndk)

```bash
export ANDROID_NDK_HOME=/path/to/ndk
cargo ndk -t arm64-v8a -t x86_64 -o ./android/app/src/main/jniLibs \
  build --release --features jni \
  --manifest-path native/Cargo.toml
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

The project uses [`iai-callgrind`](https://github.com/iai-callgrind/iai-callgrind) for instruction-count benchmarks — **Linux only** (requires Valgrind/Callgrind).

```bash
# Linux only
cargo bench -p wacore-binary
cargo bench -p wacore-libsignal
```

On Windows, run the Bun/Mitata benchmarks instead:

```powershell
npm run bench
```

To run `iai-callgrind` on Windows, install WSL and Valgrind inside it:

```powershell
wsl --install
# then inside WSL:
sudo apt install valgrind
cargo install iai-callgrind-runner
cargo bench -p wacore-binary
```

## Common gotchas

- `wacore-libsignal/Cargo.toml` — `serde`, `sha1`, `sha2`, `waproto` must be in `[dependencies]`, not under `[target.'cfg(wasm32)'.dependencies]`. The native build needs them too.
- `wacore-noise/Cargo.toml` — `wacore-binary`, `wacore-libsignal`, `waproto` are under `[target.'cfg(target_arch = "wasm32")'.dependencies]`. Native `cargo test` will fail — this is expected.
- `waproto/build.rs` — field attribute paths use `.proto.` prefix (not `.whatsapp.`), matching `package proto;`.
- The C and JNI builds write to the same `target/` directory. `build-all.ps1` copies the JNI lib to `dist-native/jni/` to keep them separate.
- `cargo test` (no target) → tries to run WASM binary natively → OS error 193 on Windows. Always specify `--target x86_64-pc-windows-msvc` for native tests.
- `wasm-pack build` requires `link.exe` in PATH. If it fails with a linker error, ensure VS Build Tools are installed and `link.exe` is not shadowed by the Git-for-Windows version at `C:\Program Files\Git\usr\bin\link.exe`.
