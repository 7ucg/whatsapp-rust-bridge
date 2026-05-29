# Usage — Java / Kotlin / Android

The Java bridge wraps the same native library as the C bridge, using JNI. The same `.dll` / `.so` is used — just built with the `--features jni` flag to include the `Java_*` symbol exports.

## Setup

### Desktop JVM (Java / Kotlin)

1. Build the JNI library:
   ```powershell
   .\build-all.ps1 -SkipWasm -SkipC
   ```

2. Copy files to your project:
   ```
   native/java/com/whatsapp/bridge/*.java    → src/main/java/com/whatsapp/bridge/
   native/target/<triple>/release-jni/*.dll  → your library path
   ```

3. Load at startup:
   ```java
   NativeLoader.load();
   ```
   Or manually:
   ```java
   System.loadLibrary("whatsapp_bridge");
   // or absolute path:
   System.load("/path/to/whatsapp_bridge.dll");
   ```

### Android (Gradle)

1. Cross-compile with cargo-ndk:
   ```bash
   export ANDROID_NDK_HOME=/path/to/ndk
   cargo ndk -t arm64-v8a -t x86_64 \
     -o android/app/src/main/jniLibs \
     build --release --features jni \
     --manifest-path native/Cargo.toml
   ```

2. Add Java sources to your module:
   ```
   native/java/com/whatsapp/bridge/ → app/src/main/java/com/whatsapp/bridge/
   ```

3. Gradle picks up `jniLibs/` automatically. Load in `Application.onCreate()`:
   ```java
   NativeLoader.load();
   ```

---

## Crypto

### Hashing

```java
byte[] hash = Crypto.sha256(data);           // 32 bytes
byte[] dig  = Crypto.md5(data);              // 16 bytes
byte[] mac  = Crypto.hmacSha256(data, key);  // 32 bytes
```

### HKDF

```java
byte[] okm = Crypto.hkdf(
    ikm,
    salt,    // null or empty for no salt
    info,    // null or empty for empty info
    80       // desired output length
);
```

### AES-256-GCM

```java
// key: 32 bytes, iv: 12 bytes, aad: any length (or null)
byte[] ct = Crypto.aesEncryptGcm(plaintext, key, iv, aad);
// auth tag (16 bytes) is appended

byte[] pt = Crypto.aesDecryptGcm(ct, key, iv, aad);
```

### AES-256-CBC

```java
// Random IV — IV (16 bytes) prepended to output
byte[] ct = Crypto.aesEncryptCbc(plaintext, key);   // key: 32 bytes
byte[] pt = Crypto.aesDecryptCbc(ct, key);

// Explicit IV
byte[] ct2 = Crypto.aesEncryptCbcWithIv(plaintext, key, iv);  // iv: 16 bytes
byte[] pt2 = Crypto.aesDecryptCbcWithIv(ct2, key, iv);
```

### AES-256-CTR

```java
// Symmetric — same call for encrypt and decrypt
byte[] out = Crypto.aesCtr(data, key, iv);  // key: 32 bytes, iv: 16 bytes
```

---

## Curve25519

```java
// Generate key pair — returns 65 bytes: [33 pubKey | 32 privKey]
byte[] kp   = Curve.generateKeyPair();
byte[] pub  = Curve.pubKeyFrom(kp);   // 33 bytes (0x05 prefix)
byte[] priv = Curve.privKeyFrom(kp);  // 32 bytes

// DH shared secret
byte[] shared = Curve.calculateAgreement(serverPub, priv);  // 32 bytes

// Sign
byte[] sig = Curve.calculateSignature(priv, message);  // 64 bytes

// Verify
boolean valid = Curve.verifySignature(pub, message, sig);

// Derive public key from private key
byte[] derivedPub = Curve.getPublicFromPrivate(priv);  // 33 bytes
```

---

## Key Helper

```java
// Registration ID
int regId = KeyHelper.generateRegistrationId();  // 0–16383

// Pre-key — returns 65 bytes: [33 pubKey | 32 privKey]
byte[] pk    = KeyHelper.generatePreKey(42);
byte[] pkPub = KeyHelper.pubKeyFrom(pk);
byte[] pkPriv = KeyHelper.privKeyFrom(pk);

// Signed pre-key — returns 129 bytes: [33 pubKey | 32 privKey | 64 signature]
byte[] spk    = KeyHelper.generateSignedPreKey(identityPrivKey, 1);
byte[] spkPub = KeyHelper.pubKeyFrom(spk);
byte[] spkPriv = KeyHelper.privKeyFrom(spk);
byte[] spkSig = KeyHelper.signatureFrom(spk);

// Verify the signed pre-key against the identity public key
boolean ok = Curve.verifySignature(identityPub, spkPub, spkSig);
```

---

## Binary nodes

```java
// Encode: JSON string -> WhatsApp wire bytes
String json = "{\"tag\":\"iq\",\"attrs\":{\"id\":\"1\",\"type\":\"get\"}}";
byte[] wire = Binary.encodeNode(json);

// Decode: wire bytes -> JSON bytes
byte[] jsonBytes = Binary.decodeNode(wire);

// Convenience: decode to String
String jsonOut = Binary.decodeNodeAsString(wire);
System.out.println(jsonOut);
// {"tag":"iq","attrs":{"id":"1","type":"get"}}
```

### JSON schema

```json
{
  "tag":   "iq",
  "attrs": { "id": "1", "type": "get" },
  "content": "text"
           | [ { "tag": "...", ... } ]
           | { "b64": "<base64 bytes>" }
}
```

`content` is optional. Binary payloads must be wrapped in `{ "b64": "..." }`.

---

## Kotlin

The Java classes work directly in Kotlin with no wrappers needed:

```kotlin
import com.whatsapp.bridge.*

NativeLoader.load()

// Crypto
val hash = Crypto.sha256(data)
val ct   = Crypto.aesEncryptGcm(plaintext, key, iv, aad)

// Curve25519
val kp    = Curve.generateKeyPair()
val pub   = Curve.pubKeyFrom(kp)
val priv  = Curve.privKeyFrom(kp)
val sig   = Curve.calculateSignature(priv, message)
val valid = Curve.verifySignature(pub, message, sig)

// Binary
val wire    = Binary.encodeNode("""{"tag":"iq","attrs":{"id":"1"}}""")
val decoded = Binary.decodeNodeAsString(wire)
```

---

## Error handling

All methods throw `java.lang.RuntimeException` on failure (bad key length, decryption failure, etc.).  
Wrap calls in try/catch where needed:

```java
try {
    byte[] pt = Crypto.aesDecryptGcm(ct, key, iv, aad);
} catch (RuntimeException e) {
    // e.g. "AES-GCM decryption failed (bad auth tag?)"
    Log.e("Bridge", "Decrypt failed", e);
}
```

---

## Android: ProGuard / R8

Add to `proguard-rules.pro`:

```proguard
-keep class com.whatsapp.bridge.** { *; }
```

---

## NativeLoader internals

`NativeLoader.load()` tries `System.loadLibrary("whatsapp_bridge")` first (works if the library is on `java.library.path` or bundled in `jniLibs/`).

If that fails, it falls back to extracting the library from the JAR resources at path:
```
/native/<os>-<arch>/libwhatsapp_bridge.so   (Linux)
/native/<os>-<arch>/whatsapp_bridge.dll     (Windows)
/native/<os>-<arch>/libwhatsapp_bridge.dylib (macOS)
```

To bundle the library in a JAR, add it to `src/main/resources/native/<os>-<arch>/`.
