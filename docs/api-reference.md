# API Reference

This page lists every exported symbol across all three bridges side-by-side.

## Hashing

| Function | TypeScript | C | Java |
|---|---|---|---|
| SHA-256 | `sha256(data)` | `wa_sha256(data, len, out, &out_len)` | `Crypto.sha256(data)` |
| MD5 | `md5(data)` | `wa_md5(data, len, out, &out_len)` | `Crypto.md5(data)` |
| HMAC-SHA256 | `hmacSign(data, key)` | `wa_hmac_sha256(data, len, key, klen, out, &out_len)` | `Crypto.hmacSha256(data, key)` |
| HKDF-SHA256 | `hkdf(ikm, length, {salt?, info?})` | `wa_hkdf(ikm, ilen, salt, slen, info, ilen2, out, out_len)` | `Crypto.hkdf(ikm, salt, info, outLen)` |

## AES-256-GCM

| Function | TypeScript | C | Java |
|---|---|---|---|
| Encrypt | `aesEncryptGCM(pt, key32, iv12, aad)` | `wa_aes_encrypt_gcm(pt, plen, key, iv, aad, alen, out, &olen)` | `Crypto.aesEncryptGcm(pt, key, iv, aad)` |
| Decrypt | `aesDecryptGCM(ct, key32, iv12, aad)` | `wa_aes_decrypt_gcm(ct, clen, key, iv, aad, alen, out, &olen)` | `Crypto.aesDecryptGcm(ct, key, iv, aad)` |

Key: 32 bytes. IV: 12 bytes. Auth tag (16 bytes) appended to ciphertext.

## AES-256-CBC

| Function | TypeScript | C | Java |
|---|---|---|---|
| Encrypt (random IV) | `aesEncrypt(pt, key32)` | `wa_aes_encrypt_cbc(pt, len, key, out, &olen)` | `Crypto.aesEncryptCbc(pt, key)` |
| Decrypt (IV prepended) | `aesDecrypt(ct, key32)` | `wa_aes_decrypt_cbc(ct, len, key, out, &olen)` | `Crypto.aesDecryptCbc(ct, key)` |
| Encrypt (explicit IV) | `aesEncrypWithIV(pt, key32, iv16)` | `wa_aes_encrypt_cbc_iv(pt, len, key, iv, out, &olen)` | `Crypto.aesEncryptCbcWithIv(pt, key, iv)` |
| Decrypt (explicit IV) | `aesDecryptWithIV(ct, key32, iv16)` | `wa_aes_decrypt_cbc_iv(ct, len, key, iv, out, &olen)` | `Crypto.aesDecryptCbcWithIv(ct, key, iv)` |

Key: 32 bytes. IV: 16 bytes.

## AES-256-CTR

| Function | TypeScript | C | Java |
|---|---|---|---|
| Encrypt / Decrypt | `aesEncryptCTR(data, key32, iv16)` / `aesDecryptCTR(...)` | `wa_aes_ctr(data, len, key, iv, out, &olen)` | `Crypto.aesCtr(data, key, iv)` |

Key: 32 bytes. IV: 16 bytes. CTR is symmetric.

## Curve25519

| Function | TypeScript | C | Java |
|---|---|---|---|
| Generate key pair | `generateKeyPair()` → `{pubKey, privKey}` | `wa_generate_key_pair(pub33, priv32)` | `Curve.generateKeyPair()` → 65 bytes |
| DH agreement | `calculateAgreement(pub, priv)` | `wa_calculate_agreement(pub, plen, priv, out32)` | `Curve.calculateAgreement(pub, priv)` |
| Sign | `calculateSignature(priv, msg)` | `wa_calculate_signature(priv, msg, mlen, sig64)` | `Curve.calculateSignature(priv, msg)` |
| Verify | `verifySignature(pub, msg, sig)` → bool | `wa_verify_signature(pub, plen, msg, mlen, sig)` → 1/0/neg | `Curve.verifySignature(pub, msg, sig)` → bool |
| Derive pub from priv | `getPublicFromPrivateKey(priv)` | `wa_get_public_from_private(priv, out33)` | `Curve.getPublicFromPrivate(priv)` |

Public keys: 33 bytes (0x05 prefix + 32 bytes). Private keys: 32 bytes. Signatures: 64 bytes.

## Key Helper

| Function | TypeScript | C | Java |
|---|---|---|---|
| Registration ID | `generateRegistrationId()` | `wa_generate_registration_id(&id)` | `KeyHelper.generateRegistrationId()` |
| Identity key pair | `generateIdentityKeyPair()` | — | — |
| Pre-key | `generatePreKey(keyId)` | `wa_generate_pre_key(id, pub, priv, &kid)` | `KeyHelper.generatePreKey(keyId)` → 65 bytes |
| Signed pre-key | `generateSignedPreKey(identityKP, keyId)` | `wa_generate_signed_pre_key(idPriv, id, pub, priv, sig, &kid)` | `KeyHelper.generateSignedPreKey(idPriv, keyId)` → 129 bytes |
| Serialise identity KP | `_serializeIdentityKeyPair(kp)` | — | — |

## JID (TypeScript only)

### Parse / Encode
| Function | Description |
|---|---|
| `parseJid(str)` | → `{ user, server, agent, device }` |
| `encodeJid(info)` | → canonical JID string |

### Constructors
| Function | Description |
|---|---|
| `jidUser(phone)` | `"123@s.whatsapp.net"` |
| `jidUserDevice(phone, device)` | with explicit device ID |
| `jidGroup(id)` | `"id@g.us"` |
| `jidNewsletter(id)` | `"id@newsletter"` |
| `jidLid(lid)` | `"lid@lid"` |
| `jidStatusBroadcast()` | `"status@broadcast"` |

### Type checks
| Function | Returns |
|---|---|
| `isUserJid(str)` | `s.whatsapp.net` |
| `isLidJid(str)` | `lid` server |
| `isGroupJid(str)` | `g.us` |
| `isBroadcastListJid(str)` | broadcast (not status) |
| `isStatusBroadcastJid(str)` | `status@broadcast` |
| `isNewsletterJid(str)` | newsletter/channel |
| `isADJid(str)` | multi-device (device > 0) |
| `isBotJid(str)` | WhatsApp bot |
| `isMessengerJid(str)` | Meta Messenger bridge |
| `isHostedJid(str)` | Cloud API / hosted device |
| `areSameUser(a, b)` | same user ignoring device |

### Accessors
| Function | Description |
|---|---|
| `jidGetUser(str)` | user / phone part |
| `jidGetServer(str)` | server domain |
| `jidGetDevice(str)` | device ID |
| `jidNormalizedUser(str)` | strip device: `"123:5@s.whatsapp.net"` → `"123@s.whatsapp.net"` |
| `jidWithDevice(str, device)` | return JID with new device ID |
| `jidUserBase(str)` | strip `:device` from user part |

### Server constants (functions)
`jidServerUser()` · `jidServerGroup()` · `jidServerBroadcast()` · `jidServerLid()` · `jidServerNewsletter()` · `jidServerMessenger()` · `jidServerBot()` · `jidServerHosted()`

## Binary protocol

| Function | TypeScript | C | Java |
|---|---|---|---|
| Encode | `encodeNode(node)` | `wa_encode_node(json, jlen, out, &olen)` | `Binary.encodeNode(jsonStr)` |
| Decode | `decodeNode(bytes)` | `wa_decode_node(data, dlen, out, &olen)` | `Binary.decodeNode(data)` |

C and Java exchange JSON. TypeScript uses typed `BinaryNode` objects. See [usage-c.md](usage-c.md#binary-nodes) for the JSON schema.

## Noise Session (TypeScript only)

### Noise XX (standard)

| Method | Description |
|---|---|
| `new NoiseSession(pubKey, noiseHeader, routingInfo?)` | Create session |
| `.processHandshakeInit(ephemeral, staticEnc, payloadEnc, privKey)` | Server → client handshake init |
| `.processHandshakeFinish(noisePub, noisePriv, serverEphemeral)` | Client → server handshake finish |
| `.finishInit()` | Finalise — switch to transport ciphers |
| `.encodeFrame(node)` | Encode + encrypt a BinaryNode frame |
| `.encodeFrameRaw(bytes)` | Encode + encrypt raw bytes as a frame |
| `.decodeFrame(data)` | Feed incoming bytes, returns decoded frames |
| `.encrypt(pt)` / `.decrypt(ct)` | Raw encrypt/decrypt |
| `.authenticate(data)` | Mix data into handshake hash |
| `.mixIntoKey(data)` | Mix data into cipher key |
| `.getHash()` | Current handshake hash |
| `.isFinished` | Whether handshake is complete |
| `.bufferedBytes` | Bytes buffered in frame decoder |
| `.clearBuffer()` | Clear frame decoder buffer |

### Noise IK (fast reconnect)

```ts
new NoiseIkSession(staticPub, staticPriv, serverStaticPub, payload, prologue)
.buildClientHello()   → Uint8Array   // send this
.readServerHello(bytes, routingInfo?)
  // → { success: true,  session: NoiseSession }
  // → { success: false, fallback: NoiseXxFallbackSession }
```

### Noise XXfallback (IK rejected)

```ts
// returned automatically from NoiseIkSession.readServerHello
fallback.buildClientFinish()   → Uint8Array   // send this
fallback.finish()              → NoiseSession  // ready to use
```

## Signal Protocol (TypeScript only)

### SessionBuilder

```ts
new SessionBuilder(storage: SignalStorage, address: ProtocolAddress)
.processPreKeyBundle(bundle: PreKeyBundleInput): Promise<void>
.initOutgoing(bundle: PreKeyBundleInput): Promise<void>
```

### SessionCipher

```ts
new SessionCipher(storage: SignalStorage, address: ProtocolAddress)
.encrypt(plaintext: Uint8Array): Promise<{ type: number; body: Uint8Array }>
.decryptWhisperMessage(ct: Uint8Array): Promise<Uint8Array>
.decryptPreKeyWhisperMessage(ct: Uint8Array): Promise<Uint8Array>
.hasOpenSession(): Promise<boolean>
```

### GroupSessionBuilder / GroupCipher

```ts
new GroupSessionBuilder(storage: SignalStorage)
.create(senderKeyName: SenderKeyName): Promise<SenderKeyDistributionMessage>
.process(senderKeyName: SenderKeyName, skdm: SenderKeyDistributionMessage): Promise<void>

new GroupCipher(storage: SignalStorage, groupId: string, sender: ProtocolAddress)
.encrypt(plaintext: Uint8Array): Promise<Uint8Array>
.decrypt(ciphertext: Uint8Array): Promise<Uint8Array>
```

## App State Sync (TypeScript only)

| Function | Description |
|---|---|
| `expandAppStateKeys(keyData)` | Expand 32-byte key → 5 sub-keys (`ExpandedAppStateKeys`) |
| `decodeAppStateRecord(recordBytes, keys, keyId, op, validateMacs)` | Decrypt one record → `DecodedMutation` |
| `encodeAppStateMutation(op, indexBytes, actionBytes, keys, keyId, iv)` | Encrypt mutation → `EncodedMutation` |
| `collectAppStateKeyIds(snapshotBytes, patchesBytes[])` | Extract key IDs needed → `Uint8Array[]` |
| `generateIndexMac(indexBytes, key)` | HMAC for patch index |
| `generateContentMac(op, data, keyId, key)` | HMAC for patch content |
| `generateSnapshotMac(ltHash, version, name, key)` | Snapshot integrity MAC |
| `generatePatchMac(snapMac, valueMacs, version, name, key)` | Patch integrity MAC |
| `new LTHashAntiTampering()` | LTHash tamper detection singleton |
| `.subtractThenAdd(base, subtract[], add[])` | Update LTHash state |
| `new LTHashState()` | Mutable versioned LTHash state |
| `.setValueMac(indexMacB64, valueMac)` | Add/update entry |
| `.getValueMac(indexMacB64)` | Lookup entry |
| `.deleteValueMac(indexMacB64)` | Remove entry |
| `.hasValueMac(indexMacB64)` | Check entry existence |
| `.hash` / `.version` | Current hash bytes / patch version |
| `.clone()` | Deep clone state |

## Misc (TypeScript only)

| Function | Description |
|---|---|
| `getWAConnHeader()` | 4-byte WebSocket connection header (`WA\x04\x00`) |
| `getEnabledFeatures()` | `{ audio, image, sticker }` — runtime feature flags |
| `getPreKeyMessageIdentityKey(ct)` | Extract sender identity key from PreKeySignalMessage |
| `setLogger(logger)` / `updateLogger(logger)` | Set/replace Rust-side logger |
| `hasLogger()` | Whether a logger is currently set |
| `logMessage(level, msg)` | Emit a log entry through the Rust logger |

## C error codes

| Constant | Value | Meaning |
|---|---|---|
| `WA_OK` | 0 | Success |
| `WA_ERR_NULL_POINTER` | -1 | A required pointer argument is NULL |
| `WA_ERR_BAD_KEY_LEN` | -2 | Key has wrong length |
| `WA_ERR_BAD_IV_LEN` | -3 | IV has wrong length |
| `WA_ERR_ENCRYPT_FAIL` | -4 | Encryption failed |
| `WA_ERR_DECRYPT_FAIL` | -5 | Decryption failed (wrong key / bad auth tag) |
| `WA_ERR_OUTPUT_SMALL` | -6 | Output buffer too small; `*out_len` set to required size |
| `WA_ERR_HKDF_FAIL` | -7 | HKDF expansion failed |
| `WA_ERR_RNG_FAIL` | -8 | OS random number generator failed |

## VoIP / Calls (all targets)

Pure media-plane primitives (`src/voip.rs`, `native/.../voip.rs`, `Voip.java`).
Stateful — one instance per stream/call; release native handles (`*Free` / `*_free`).

### MLow audio codec — `MlowEncoder` / `MlowDecoder`

| Op | TS | C | Java |
|---|---|---|---|
| create | `new MlowEncoder()` | `wa_mlow_encoder_new` | `Voip.mlowEncoderNew` |
| encode (f32 frame → bytes) | `enc.encode(Float32Array)` | `wa_mlow_encode` | `Voip.mlowEncode` |
| decode (bytes → f32) | `dec.decode(Uint8Array)` | `wa_mlow_decode` | `Voip.mlowDecode` |
| redundancy / reset / free | `dec.setRedundancy` · `reset()` | `wa_mlow_decoder_set_redundancy` · `*_reset` · `*_free` | `mlowDecoderSetRedundancy` · `*Reset` · `*Free` |

Mic frames are 960 samples (60 ms @ 16 kHz mono). C decode `out_len` is in **float elements**.

### E2E SRTP — `MediaPipeline`

`create(callKey, selfLid, peerLid, ssrc, samplesPerPacket, warpMiTagLen)` → `protectAudio` (bytes→packet), `unprotectAudio` (packet→bytes | none), `rekeyRecv`, free. Create fails (throw / NULL / 0) if the callKey is too short to derive E2E keys.

### CallEngine — sans-io signaling + media driver

`create(configJson)` → `start(now)` → feed `handleRelayPacket` / `handleMicFrame` / `handleTimeout` → drain `pollOutput()` until it returns `0` (TIMEOUT), taking the payload per **output kind** (`1` Transmit, `2` Playout, `3` Event) via `takeTransmit` / `takePlayout` / `eventKind`+`takeForeignAudio`/`eventCode`. Arm timers from `pollTimeout()` (ms, `-1` = none). Plus `rekeyRecv`, `callId`, `direction` (0 out / 1 in), `isAllocated`, `isTerminated`. STUN transaction ids use an OS RNG.

Config JSON (snake_case): `call_id`, `direction` (`"incoming"`/`"outgoing"`), `self_lid`, `peer_lid`, `call_key[]`, `ssrc`, `samples_per_packet`, `relay_token[]`, `relay_ip`, `relay_port`, `integrity_key[]`, `warp_mi_tag_len`, `enable_media`, `enable_sframe`.
