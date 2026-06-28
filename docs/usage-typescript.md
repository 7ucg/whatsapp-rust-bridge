# Usage — TypeScript / Node.js

The TypeScript bridge runs on top of WebAssembly and works in Node.js, Bun, and modern browsers.  
The WASM module is embedded into `dist/index.js` at build time — no separate `.wasm` file needed at runtime.

## Installation

```bash
npm install whatsapp-rust-bridge-baron
# or
bun add whatsapp-rust-bridge-baron
```

Or from a local build:

```ts
const bridge = require("./dist/index.js");
// or
import * as bridge from "./dist/index.js";
```

No `initSync` call needed — the module initialises automatically on import.

---

## Crypto

### SHA-256 / MD5

```ts
import { sha256, md5 } from "whatsapp-rust-bridge-baron";

const hash = sha256(new Uint8Array([1, 2, 3]));  // Uint8Array (32 bytes)
const dig  = md5(new Uint8Array([1, 2, 3]));     // Uint8Array (16 bytes)
```

### HMAC-SHA256

```ts
import { hmacSign } from "whatsapp-rust-bridge-baron";

const mac = hmacSign(data, key);  // Uint8Array (32 bytes)
```

### HKDF

```ts
import { hkdf } from "whatsapp-rust-bridge-baron";

const okm = hkdf(ikm, 80, { salt: saltBytes, info: "WhatsApp Out" });
// info can be a string or Uint8Array; salt is optional
```

### AES-256-GCM

```ts
import { aesEncryptGCM, aesDecryptGCM } from "whatsapp-rust-bridge-baron";

// Encrypt — auth tag (16 bytes) is appended to ciphertext
const ct = aesEncryptGCM(plaintext, key32, iv12, aad);

// Decrypt — expects auth tag appended
const pt = aesDecryptGCM(ct, key32, iv12, aad);
```

### AES-256-CBC

```ts
import { aesEncrypt, aesDecrypt, aesEncrypWithIV, aesDecryptWithIV } from "whatsapp-rust-bridge-baron";

// Random IV — IV (16 bytes) prepended to output
const ct = aesEncrypt(plaintext, key32);
const pt = aesDecrypt(ct, key32);

// Explicit IV
const ct2 = aesEncrypWithIV(plaintext, key32, iv16);
const pt2 = aesDecryptWithIV(ct2, key32, iv16);
```

### AES-256-CTR

```ts
import { aesEncryptCTR, aesDecryptCTR } from "whatsapp-rust-bridge-baron";

const ct = aesEncryptCTR(plaintext, key32, iv16);
const pt = aesDecryptCTR(ct, key32, iv16);
// CTR is symmetric — encrypt and decrypt are the same operation
```

---

## Curve25519

```ts
import {
  generateKeyPair, calculateAgreement,
  calculateSignature, verifySignature,
  getPublicFromPrivateKey,
} from "whatsapp-rust-bridge-baron";

// Generate key pair
const { pubKey, privKey } = generateKeyPair();
// pubKey  → Uint8Array (33 bytes, 0x05 prefix)
// privKey → Uint8Array (32 bytes)

// Diffie-Hellman shared secret
const shared = calculateAgreement(serverPubKey, privKey);  // 32 bytes

// Sign / verify
const sig   = calculateSignature(privKey, message);        // 64 bytes
const valid = verifySignature(pubKey, message, sig);       // boolean

// Derive public key from private key
const pub = getPublicFromPrivateKey(privKey);              // 33 bytes
```

---

## Key Helper (Signal protocol)

```ts
import {
  generateIdentityKeyPair,
  generateRegistrationId,
  generatePreKey,
  generateSignedPreKey,
  _serializeIdentityKeyPair,
} from "whatsapp-rust-bridge-baron";

const identityKeyPair = generateIdentityKeyPair();
// { pubKey: Uint8Array, privKey: Uint8Array }

const registrationId = generateRegistrationId();  // number (0–16383)

const preKey = generatePreKey(42);
// { keyId: 42, keyPair: { pubKey, privKey } }

const signedPreKey = generateSignedPreKey(identityKeyPair, 1);
// { keyId: 1, keyPair: { pubKey, privKey }, signature: Uint8Array }
```

---

## Binary Protocol

```ts
import { encodeNode, decodeNode } from "whatsapp-rust-bridge-baron";
import type { BinaryNode } from "whatsapp-rust-bridge-baron";

// Encode
const node: BinaryNode = {
  tag: "iq",
  attrs: { id: "1", type: "get", xmlns: "w:p" },
  content: [
    { tag: "ping", attrs: {} }
  ],
};
const wireBytes = encodeNode(node);  // Uint8Array

// Decode
const decoded = decodeNode(wireBytes);
console.log(decoded.tag);    // "iq"
console.log(decoded.attrs);  // { id: "1", type: "get", xmlns: "w:p" }

// Content can be BinaryNode[], string, or Uint8Array
const children = decoded.content;  // BinaryNode[]

// Full object (for JSON serialisation)
const obj = decoded.toJSON();
```

---

## Noise Session (WebSocket handshake)

```ts
import { NoiseSession, getWAConnHeader } from "whatsapp-rust-bridge-baron";

const header  = getWAConnHeader();  // 4-byte WA_CONN_HEADER
const session = new NoiseSession(serverPublicKey, noiseHeader, routingInfo);

// Handshake phase
const certPayload = session.processHandshakeInit(
  serverEphemeral,
  serverStaticEncrypted,
  serverPayloadEncrypted,
  clientPrivateKey,
);

const encryptedKey = session.processHandshakeFinish(
  noisePublicKey,
  noisePrivateKey,
  serverEphemeral,
);

session.finishInit();
console.log(session.isFinished);  // true

// Encode / decode frames
const frame     = session.encodeFrame(node);        // Uint8Array
const rawFrame  = session.encodeFrameRaw(bytes);    // Uint8Array
const decoded   = session.decodeFrame(incomingData); // Array
```

---

## Session / Signal Protocol

```ts
import {
  SessionBuilder, SessionCipher, ProtocolAddress,
  GroupCipher, GroupSessionBuilder, SenderKeyName,
} from "whatsapp-rust-bridge-baron";
import type { SignalStorage } from "whatsapp-rust-bridge-baron";

// Implement SignalStorage (async-compatible)
const storage: SignalStorage = {
  loadSession: async (address) => { /* return Uint8Array | null */ },
  storeSession: async (address, record) => { /* ... */ },
  getOurIdentity: async () => identityKeyPair,
  getOurRegistrationId: async () => registrationId,
  isTrustedIdentity: async (name, key, dir) => true,
  loadPreKey: async (id) => { /* return KeyPair | null */ },
  removePreKey: async (id) => { /* ... */ },
  loadSignedPreKey: async (id) => { /* return SignedPreKey | null */ },
  loadSenderKey: async (keyId) => { /* return Uint8Array | null */ },
  storeSenderKey: async (keyId, record) => { /* ... */ },
};

const address = new ProtocolAddress("1234567890@s.whatsapp.net", 0);
const builder = new SessionBuilder(storage, address);

await builder.processPreKeyBundle({
  registrationId: 12345,
  identityKey: remoteIdentityKey,
  preKey: { keyId: 1, publicKey: preKeyPub },
  signedPreKey: { keyId: 1, publicKey: signedPub, signature: sig },
});

const cipher = new SessionCipher(storage, address);
const encrypted = await cipher.encrypt(plaintext);
const decrypted = await cipher.decryptWhisperMessage(ciphertext);
```

---

## App State Sync

```ts
import {
  expandAppStateKeys,
  generateIndexMac, generateContentMac,
  generateSnapshotMac, generatePatchMac,
  LTHashAntiTampering, LTHashState,
} from "whatsapp-rust-bridge-baron";

const keys = expandAppStateKeys(keyData);
// keys.indexKey, keys.patchMacKey, keys.snapshotMacKey,
// keys.valueEncryptionKey, keys.valueMacKey

const indexMac   = generateIndexMac(indexBytes, keys.indexKey);
const contentMac = generateContentMac(operation, data, keyId, keys.valueMacKey);
const snapMac    = generateSnapshotMac(ltHash, version, name, keys.snapshotMacKey);
const patchMac   = generatePatchMac(snapshotMac, valueMacs, version, name, keys.patchMacKey);

// LTHash anti-tamper
const lt = new LTHashAntiTampering();
const newHash = lt.subtractThenAdd(baseHash, toSubtract, toAdd);
```

---

## Feature detection

```ts
import { getEnabledFeatures } from "whatsapp-rust-bridge-baron";

const { audio, image, sticker } = getEnabledFeatures();
if (image) {
  // safe to call image functions
}
```

---

## TypeScript types

All types are exported from `pkg/whatsapp_rust_bridge.d.ts`:

```ts
import type {
  BinaryNode, KeyPair, SignedPreKey, PreKey,
  HkdfInfo, EnabledFeatures, SignalStorage,
  PreKeyBundleInput,
} from "whatsapp-rust-bridge-baron";
```

---

## VoIP / Calls

Pure media-plane primitives for WhatsApp calls (no networking — you own the socket).
All instances are stateful; create one per stream/call and reuse it.

### MLow audio codec

```ts
import { MlowEncoder, MlowDecoder } from "whatsapp-rust-bridge-baron";

const enc = new MlowEncoder();
const dec = new MlowDecoder();

// 60ms mic frame: exactly 960 f32 samples (16 kHz mono, range -1.0..=1.0)
const payload = enc.encode(micFrame);     // Float32Array -> Uint8Array
const pcm     = dec.decode(payload);      // Uint8Array -> Float32Array (PLC on loss)
// dec.setRedundancy(n); enc.reset(); dec.reset();
```

### E2E SRTP media pipeline

```ts
import { MediaPipeline } from "whatsapp-rust-bridge-baron";

// throws if callKey is too short to derive E2E keys
const pipe = MediaPipeline.create(callKey, selfLid, peerLid, ssrc, 960, 4);

const packet  = pipe.protectAudio(payload);      // codec bytes -> SRTP packet
const decoded = pipe.unprotectAudio(packet);     // SRTP packet -> bytes | undefined
pipe.rekeyRecv(callKey, answeringPeerLid);       // after the peer answers
```

### CallEngine (sans-io signaling + media driver)

Feed inputs, then drain `pollOutput()` until it returns `0` (TIMEOUT = drained),
taking each output's payload with the matching getter. Times are monotonic ms.

```ts
import { CallEngine } from "whatsapp-rust-bridge-baron";

const eng = CallEngine.create(JSON.stringify({
  call_id, direction: "incoming", self_lid, peer_lid,
  call_key: [...callKey], ssrc, samples_per_packet: 960,
  relay_token: [...relayToken], relay_ip, relay_port,
  integrity_key: [...integrityKey], warp_mi_tag_len: 4,
  enable_media: true, enable_sframe: true,
}));

eng.start(now);
eng.handleRelayPacket(now, packet);   // or handleMicFrame(now, Int16Array) / handleTimeout(now)

for (let kind = eng.pollOutput(); kind !== 0; kind = eng.pollOutput()) {
  if (kind === 1) sendToRelay(eng.takeTransmit());        // TRANSMIT
  else if (kind === 2) playout(eng.takePlayout());        // PLAYOUT (Int16Array)
  else if (kind === 3) {                                   // EVENT
    switch (eng.eventKind()) {
      case 0: /* RelayAllocated */ break;
      case 1: decodeForeign(eng.takeForeignAudio()); break;
      case 2: /* RelayAllocateFailed */ console.warn(eng.eventCode()); break;
      case 3: /* RelayAllocateTimedOut */ break;
    }
  }
}
const armAt = eng.pollTimeout();   // ms deadline, or -1 = no timer
// eng.callId() / eng.direction() (0=out,1=in) / eng.isAllocated() / eng.isTerminated()
```
