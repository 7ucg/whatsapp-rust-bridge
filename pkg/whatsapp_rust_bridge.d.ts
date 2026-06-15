/* tslint:disable */
/* eslint-disable */

export interface BinaryNode {
    tag: string;
    attrs: { [key: string]: string };
    content?: BinaryNode[] | string | Uint8Array;
}



export interface ILogger {
    level: string;
    trace(obj: object, msg?: string): void;
    debug(obj: object, msg?: string): void;
    info(obj: object, msg?: string): void;
    warn(obj: object, msg?: string): void;
    error(obj: object, msg?: string): void;
}



export interface SignalStorage {
    loadSession(address: string): Uint8Array | null | undefined | Promise<Uint8Array | null | undefined>;
    storeSession(address: string, record: SessionRecord): void | Promise<void>;
    getOurIdentity(): KeyPair | Promise<KeyPair>;
    getOurRegistrationId(): number | Promise<number>;
    isTrustedIdentity(name: string, identityKey: Uint8Array, direction: number): boolean | Promise<boolean>;
    loadPreKey(id: number): KeyPair | null | undefined | Promise<KeyPair | null | undefined>;
    removePreKey(id: number): void | Promise<void>;
    loadSignedPreKey(id: number): SignedPreKey | null | undefined | Promise<SignedPreKey | null | undefined>;
    loadSenderKey(keyId: string): Uint8Array | null | undefined | Promise<Uint8Array | null | undefined>;
    storeSenderKey(keyId: string, record: Uint8Array): void | Promise<void>;
}


/**
 * A WhatsApp JID (Jabber ID) — identifies a user, group, broadcast, etc.
 */
export interface JidInfo {
    user: string;
    server: string;
    agent: number;
    device: number;
}

/**
 * A cryptographic key pair containing public and private keys
 */
export interface KeyPair {
    pubKey: Uint8Array;
    privKey: Uint8Array;
}

/**
 * A decoded app-state mutation — the result of decrypting one record.
 */
export interface DecodedMutation {
    /**
     * The operation type: 0 = SET, 1 = REMOVE
     */
    operation: number;
    /**
     * The index components (JSON array path, e.g. [\"contact\",\"123@s.whatsapp.net\"])
     */
    index: string[];
    /**
     * The index MAC bytes (base64 in JSON, Uint8Array in JS)
     */
    indexMac: Uint8Array;
    /**
     * The value MAC bytes
     */
    valueMac: Uint8Array;
    /**
     * The decrypted action value as protobuf-encoded bytes (SyncActionValue)
     */
    actionBytes: Uint8Array | undefined;
}

/**
 * Enabled features in this build.
 * Use this to check feature availability at runtime before calling feature-gated functions.
 */
export interface EnabledFeatures {
    /**
     * Audio processing support (waveform generation, duration detection)
     */
    audio: boolean;
    /**
     * Image processing support (thumbnails, profile pictures, format conversion)
     */
    image: boolean;
    /**
     * Sticker metadata support (WebP EXIF for WhatsApp stickers)
     */
    sticker: boolean;
}

/**
 * Encoded mutation result returned by `encodeAppStateMutation`.
 */
export interface EncodedMutation {
    /**
     * Protobuf-encoded `SyncdMutation` bytes — add to your patch\'s mutations array.
     */
    mutationBytes: Uint8Array;
    /**
     * The value MAC (32 bytes) — needed for `LTHashState.setValueMac`.
     */
    valueMac: Uint8Array;
    /**
     * The index MAC (from the encoded record) — needed for `LTHashState.setValueMac` key.
     */
    indexMac: Uint8Array;
}

export interface HkdfInfo {
    salt?: Uint8Array | undefined;
    info?: Uint8Array | string | undefined;
}

export interface PreKey {
    keyId: number;
    keyPair: KeyPair;
}

export interface PreKeyBundleInput {
    registrationId: number;
    identityKey: Uint8Array;
    preKey?: PreKeyPublicKey | undefined;
    signedPreKey: SignedPreKeyPublicKey;
}

export interface PreKeyPublicKey {
    keyId: number;
    publicKey: Uint8Array;
}

export interface SignedPreKey {
    keyId: number;
    keyPair: KeyPair;
    signature: Uint8Array;
}

export interface SignedPreKeyPublicKey {
    keyId: number;
    publicKey: Uint8Array;
    signature: Uint8Array;
}


export class ExpandedAppStateKeys {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly indexKey: Uint8Array;
    readonly patchMacKey: Uint8Array;
    readonly snapshotMacKey: Uint8Array;
    readonly valueEncryptionKey: Uint8Array;
    readonly valueMacKey: Uint8Array;
}

export class GroupCipher {
    free(): void;
    [Symbol.dispose](): void;
    decrypt(ciphertext: Uint8Array): Promise<Uint8Array>;
    encrypt(plaintext: Uint8Array): Promise<Uint8Array>;
    constructor(storage: SignalStorage, group_id: string, sender: ProtocolAddress);
}

export class GroupSessionBuilder {
    free(): void;
    [Symbol.dispose](): void;
    create(sender_key_name: SenderKeyName): Promise<SenderKeyDistributionMessage>;
    constructor(storage: SignalStorage);
    process(sender_key_name: SenderKeyName, skdm: SenderKeyDistributionMessage): Promise<void>;
}

export class InternalBinaryNode {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    toJSON(): any;
    attrs: { [key: string]: string };
    get content(): BinaryNode[] | string | Uint8Array | undefined;
    set content(value: BinaryNode[] | string | Uint8Array);
    readonly tag: string;
}

export class LTHashAntiTampering {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    subtractThenAdd(base: Uint8Array, subtract: Uint8Array[], add: Uint8Array[]): Uint8Array;
}

export class LTHashState {
    free(): void;
    [Symbol.dispose](): void;
    clone(): LTHashState;
    deleteValueMac(index_mac_base64: string): boolean;
    getValueMac(index_mac_base64: string): Uint8Array | undefined;
    hasValueMac(index_mac_base64: string): boolean;
    constructor();
    setValueMac(index_mac_base64: string, value_mac: Uint8Array): void;
    hash: Uint8Array;
    version: bigint;
}

/**
 * Noise_IK_25519_AESGCM_SHA256 handshake — faster reconnect using a cached
 * server static key. Falls back to XX automatically if the server rejects.
 *
 * Usage:
 *   const ik = new NoiseIkSession(staticPub32, staticPriv32, serverStaticPub32, payload, prologue);
 *   const clientHello = ik.buildClientHello();
 *   // send clientHello framed over the wire, then:
 *   const result = ik.readServerHello(serverHelloBytes);
 *   if (result.fallback) {
 *     // use result.fallback (NoiseSession) to continue as XX fallback
 *   } else {
 *     // IK succeeded — use result.writeCipher / readCipher
 *   }
 */
export class NoiseIkSession {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Build the IK ClientHello bytes (framed, ready to send).
     */
    buildClientHello(): Uint8Array;
    /**
     * Create a new IK session.
     * @param staticPub      Client's static public key (33 bytes, 0x05 prefix)
     * @param staticPriv     Client's static private key (32 bytes)
     * @param serverStaticPub Server's static public key (32 bytes, no prefix)
     * @param clientPayload  The payload to send 0-RTT (e.g. client hello proto)
     * @param prologue       Noise prologue bytes (WA header)
     */
    constructor(static_pub: Uint8Array, static_priv: Uint8Array, server_static_pub: Uint8Array, client_payload: Uint8Array, prologue: Uint8Array);
    /**
     * Process the server's response.
     * Returns a JS object: `{ success: true, writeCipher, readCipher }` on IK success,
     * or `{ success: false, fallbackSession: NoiseSession }` when the server requests XX fallback.
     */
    readServerHello(response_bytes: Uint8Array, routing_info?: Uint8Array | null): any;
}

/**
 * NoiseSession implements the Noise_XX_25519_AESGCM_SHA256 protocol pattern
 * with combined binary encoding/decoding operations for reduced WASM boundary crossings.
 */
export class NoiseSession {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Updates the session hash with the given data (no-op after handshake).
     */
    authenticate(data: Uint8Array): void;
    clearBuffer(): void;
    decodeFrame(new_data: Uint8Array): Array<any>;
    decrypt(ciphertext: Uint8Array): Uint8Array;
    encodeFrame(node: EncodingNode): Uint8Array;
    encodeFrameRaw(data: Uint8Array): Uint8Array;
    encrypt(plaintext: Uint8Array): Uint8Array;
    finishInit(): void;
    getHash(): Uint8Array;
    mixIntoKey(data: Uint8Array): void;
    constructor(public_key: Uint8Array, noise_header: Uint8Array, routing_info?: Uint8Array | null);
    processHandshakeFinish(noise_public_key: Uint8Array, noise_private_key: Uint8Array, server_ephemeral: Uint8Array): Uint8Array;
    processHandshakeInit(server_ephemeral: Uint8Array, server_static_encrypted: Uint8Array, server_payload_encrypted: Uint8Array, private_key: Uint8Array): Uint8Array;
    readonly bufferedBytes: number;
    readonly isFinished: boolean;
}

/**
 * Noise XXfallback session — used when an IK attempt is rejected by the server.
 * Reuses the ephemeral already on the wire to avoid an extra round-trip.
 */
export class NoiseXxFallbackSession {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Build the client finish message (send this over the wire).
     */
    buildClientFinish(): Uint8Array;
    /**
     * Finalize the handshake — returns a ready `NoiseSession`.
     */
    finish(): NoiseSession;
}

export class ProtocolAddress {
    free(): void;
    [Symbol.dispose](): void;
    static from(encoded: string): ProtocolAddress;
    is(other: ProtocolAddress): boolean;
    constructor(id: string, device_id: number);
    toString(): string;
    readonly deviceId: number;
    readonly id: string;
}

export class SenderKeyDistributionMessage {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static deserialize(serialized: Uint8Array): SenderKeyDistributionMessage;
    serialize(): Uint8Array;
}

export class SenderKeyName {
    free(): void;
    [Symbol.dispose](): void;
    constructor(group_id: string, sender: ProtocolAddress);
    toString(): string;
}

export class SenderKeyRecord {
    free(): void;
    [Symbol.dispose](): void;
    static deserialize(serialized: Uint8Array): SenderKeyRecord;
    isEmpty(): boolean;
    constructor();
    serialize(): Uint8Array;
}

export class SessionBuilder {
    free(): void;
    [Symbol.dispose](): void;
    initOutgoing(bundle_input: PreKeyBundleInput): Promise<void>;
    constructor(storage: SignalStorage, remote_address: ProtocolAddress);
    processPreKeyBundle(bundle_input: PreKeyBundleInput): Promise<void>;
}

export class SessionCipher {
    free(): void;
    [Symbol.dispose](): void;
    decryptPreKeyWhisperMessage(ciphertext: Uint8Array): Promise<Uint8Array>;
    decryptWhisperMessage(ciphertext: Uint8Array): Promise<Uint8Array>;
    encrypt(plaintext: Uint8Array): Promise<{ type: number; body: Uint8Array }>;
    hasOpenSession(): Promise<boolean>;
    constructor(storage: SignalStorage, remote_address: ProtocolAddress);
}

export class SessionRecord {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static deserialize(val: any): SessionRecord;
    haveOpenSession(): boolean;
    serialize(): Uint8Array;
}

export function _serializeIdentityKeyPair(key_pair: KeyPair): Uint8Array;

/**
 * Decrypts AES-256-CBC; IV is the first 16 bytes of `buffer`.
 */
export function aesDecrypt(buffer: Uint8Array, key: Uint8Array): Uint8Array;

export function aesDecryptCTR(ciphertext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array;

/**
 * Decrypts AES-256-GCM; expects auth tag (16 bytes) appended to ciphertext.
 */
export function aesDecryptGCM(ciphertext_with_tag: Uint8Array, key: Uint8Array, iv: Uint8Array, additional_data: Uint8Array): Uint8Array;

/**
 * Decrypts AES-256-CBC with an explicit IV.
 */
export function aesDecryptWithIV(buffer: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array;

/**
 * Encrypts AES-256-CBC with a given IV (no IV prefix in output).
 */
export function aesEncrypWithIV(buffer: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array;

/**
 * Encrypts AES-256-CBC with a random IV; IV is prepended to output.
 */
export function aesEncrypt(buffer: Uint8Array, key: Uint8Array): Uint8Array;

export function aesEncryptCTR(plaintext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array;

/**
 * Encrypts with AES-256-GCM; auth tag (16 bytes) is appended to ciphertext.
 */
export function aesEncryptGCM(plaintext: Uint8Array, key: Uint8Array, iv: Uint8Array, additional_data: Uint8Array): Uint8Array;

/**
 * Returns true if both JIDs refer to the same user (ignoring device).
 */
export function areSameUser(a: string, b: string): boolean;

export function calculateAgreement(public_key_bytes: Uint8Array, private_key_bytes: Uint8Array): Uint8Array;

export function calculateSignature(private_key_bytes: Uint8Array, message: Uint8Array): Uint8Array;

/**
 * Extract all unique key IDs from a list of patches (and optionally a snapshot).
 *
 * @param snapshotBytes  Optional protobuf-encoded `SyncdSnapshot` bytes (pass empty Uint8Array to skip)
 * @param patchesBytes   Array of protobuf-encoded `SyncdPatch` bytes
 * @returns              Array of key-ID byte arrays that need to be fetched
 */
export function collectAppStateKeyIds(snapshot_bytes: Uint8Array, patches_bytes: Uint8Array[]): Array<any>;

/**
 * Decrypt and decode a single app-state record.
 *
 * @param recordBytes   Protobuf-encoded `SyncdRecord` bytes
 * @param keys          Expanded app-state keys (from `expandAppStateKeys`)
 * @param keyId         The key ID bytes (used for MAC validation)
 * @param operation     0 = SET, 1 = REMOVE
 * @param validateMacs  Whether to verify MACs (set false to skip for speed)
 */
export function decodeAppStateRecord(record_bytes: Uint8Array, keys: ExpandedAppStateKeys, key_id: Uint8Array, operation: number, validate_macs: boolean): DecodedMutation;

export function decodeNode(data: Uint8Array): InternalBinaryNode;

/**
 * Encode and encrypt a mutation into a `SyncdMutation` (ready to include in a patch).
 *
 * @param operation     0 = SET, 1 = REMOVE
 * @param indexBytes    The index as JSON bytes, e.g. `["contact","123@s.whatsapp.net"]`
 * @param actionBytes   Protobuf-encoded `SyncActionValue` bytes
 * @param keys          Expanded app-state keys (from `expandAppStateKeys`)
 * @param keyId         The key ID bytes
 * @param iv            16-byte IV for AES-CBC encryption (use random bytes)
 * @param version       Per-action schema version stamped into the mutation
 *                      (mirrors WA Web; e.g. label_edit/label_jid = 3, 0 otherwise)
 */
export function encodeAppStateMutation(operation: number, index_bytes: Uint8Array, action_bytes: Uint8Array, keys: ExpandedAppStateKeys, key_id: Uint8Array, iv: Uint8Array, version: number): EncodedMutation;

/**
 * Encode a JidInfo back to its canonical string.
 */
export function encodeJid(info: JidInfo): string;

export function encodeNode(node_val: EncodingNode): Uint8Array;

export function expandAppStateKeys(key_data: Uint8Array): ExpandedAppStateKeys;

export function generateContentMac(operation: number, data: Uint8Array, key_id: Uint8Array, key: Uint8Array): Uint8Array;

export function generateIdentityKeyPair(): KeyPair;

export function generateIndexMac(index_bytes: Uint8Array, key: Uint8Array): Uint8Array;

export function generateKeyPair(): KeyPair;

export function generatePatchMac(snapshot_mac: Uint8Array, value_macs: Uint8Array[], version: bigint, name: string, key: Uint8Array): Uint8Array;

export function generatePreKey(key_id: number): PreKey;

export function generateRegistrationId(): number;

export function generateSignedPreKey(identity_key_pair: KeyPair, signed_key_id: number): SignedPreKey;

export function generateSnapshotMac(lt_hash: Uint8Array, version: bigint, name: string, key: Uint8Array): Uint8Array;

/**
 * Returns which optional features are enabled in this build.
 * Use this to conditionally call feature-gated functions.
 */
export function getEnabledFeatures(): EnabledFeatures;

/**
 * Extracts the sender's identity key from a PreKeySignalMessage for identity-change detection.
 * Returns `undefined` if parsing fails or the message is not a valid PreKeyMessage.
 */
export function getPreKeyMessageIdentityKey(ciphertext: Uint8Array): Uint8Array | undefined;

export function getPublicFromPrivateKey(private_key_bytes: Uint8Array): Uint8Array;

/**
 * Returns the WhatsApp connection header (WA_CONN_HEADER).
 * This is the 4-byte header sent at the start of a WebSocket connection.
 */
export function getWAConnHeader(): Uint8Array;

export function hasLogger(): boolean;

export function hkdf(buffer: Uint8Array, expanded_length: number, info: HkdfInfo): Uint8Array;

export function hmacSign(buffer: Uint8Array, key: Uint8Array): Uint8Array;

/**
 * Returns true if the JID belongs to a multi-device (AD) session (device > 0).
 */
export function isADJid(jid_str: string): boolean;

/**
 * Returns true if the JID is a WhatsApp bot.
 */
export function isBotJid(jid_str: string): boolean;

/**
 * Returns true if the JID is a broadcast list (not status).
 */
export function isBroadcastListJid(jid_str: string): boolean;

/**
 * Returns true if the JID is a group (g.us).
 */
export function isGroupJid(jid_str: string): boolean;

/**
 * Returns true if the JID is a hosted/Cloud API device.
 */
export function isHostedJid(jid_str: string): boolean;

/**
 * Returns true if the JID is a LID-based user (lid server).
 */
export function isLidJid(jid_str: string): boolean;

/**
 * Returns true if the JID is a Meta Messenger bridged contact.
 */
export function isMessengerJid(jid_str: string): boolean;

/**
 * Returns true if the JID is a newsletter (channel).
 */
export function isNewsletterJid(jid_str: string): boolean;

/**
 * Returns true if the JID is the status broadcast ("status@broadcast").
 */
export function isStatusBroadcastJid(jid_str: string): boolean;

/**
 * Returns true if the JID is a regular user (s.whatsapp.net).
 */
export function isUserJid(jid_str: string): boolean;

/**
 * Extract the device ID from a JID string (0 = primary device).
 */
export function jidGetDevice(jid_str: string): number | undefined;

/**
 * Extract the server domain from a JID string.
 */
export function jidGetServer(jid_str: string): string | undefined;

/**
 * Extract the user part (phone / group-id) from a JID string.
 */
export function jidGetUser(jid_str: string): string | undefined;

/**
 * Create a group JID: "groupId@g.us"
 */
export function jidGroup(group_id: string): string;

/**
 * Create a LID JID: "lid@lid"
 */
export function jidLid(lid: string): string;

/**
 * Create a newsletter (channel) JID: "id@newsletter"
 */
export function jidNewsletter(id: string): string;

/**
 * Normalize a JID to its primary user form (device = 0, agent = 0).
 * "123@s.whatsapp.net:5" → "123@s.whatsapp.net"
 */
export function jidNormalizedUser(jid_str: string): string | undefined;

export function jidServerBot(): string;

export function jidServerBroadcast(): string;

export function jidServerGroup(): string;

export function jidServerHosted(): string;

export function jidServerLid(): string;

export function jidServerMessenger(): string;

export function jidServerNewsletter(): string;

export function jidServerUser(): string;

/**
 * Returns the status broadcast JID: "status@broadcast"
 */
export function jidStatusBroadcast(): string;

/**
 * Create a user JID: "phone@s.whatsapp.net"
 */
export function jidUser(phone: string): string;

/**
 * Returns the base user part stripping any ":device" suffix.
 * "123:4@s.whatsapp.net" → "123"
 */
export function jidUserBase(jid_str: string): string | undefined;

/**
 * Create a user JID with a specific device ID.
 */
export function jidUserDevice(phone: string, device: number): string;

/**
 * Change the device ID on an existing JID string.
 */
export function jidWithDevice(jid_str: string, device: number): string | undefined;

export function logMessage(level: string, message: string): void;

export function md5(buffer: Uint8Array): Uint8Array;

/**
 * Parse a JID string into its components.
 * Accepts: "user@server", "user@server:device", "user.agent:device@server"
 */
export function parseJid(jid_str: string): JidInfo | undefined;

export function setLogger(logger: ILogger): void;

export function sha256(buffer: Uint8Array): Uint8Array;

export function updateLogger(logger: ILogger): void;

export function verifySignature(public_key_bytes: Uint8Array, message: Uint8Array, signature: Uint8Array): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_expandedappstatekeys_free: (a: number, b: number) => void;
    readonly __wbg_groupcipher_free: (a: number, b: number) => void;
    readonly __wbg_groupsessionbuilder_free: (a: number, b: number) => void;
    readonly __wbg_internalbinarynode_free: (a: number, b: number) => void;
    readonly __wbg_lthashantitampering_free: (a: number, b: number) => void;
    readonly __wbg_lthashstate_free: (a: number, b: number) => void;
    readonly __wbg_noiseiksession_free: (a: number, b: number) => void;
    readonly __wbg_noisesession_free: (a: number, b: number) => void;
    readonly __wbg_noisexxfallbacksession_free: (a: number, b: number) => void;
    readonly __wbg_protocoladdress_free: (a: number, b: number) => void;
    readonly __wbg_senderkeydistributionmessage_free: (a: number, b: number) => void;
    readonly __wbg_senderkeyname_free: (a: number, b: number) => void;
    readonly __wbg_senderkeyrecord_free: (a: number, b: number) => void;
    readonly __wbg_sessionbuilder_free: (a: number, b: number) => void;
    readonly __wbg_sessionrecord_free: (a: number, b: number) => void;
    readonly _serializeIdentityKeyPair: (a: number) => number;
    readonly aesDecrypt: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly aesDecryptCTR: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly aesDecryptGCM: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
    readonly aesDecryptWithIV: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly aesEncrypWithIV: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly aesEncrypt: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly aesEncryptGCM: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
    readonly areSameUser: (a: number, b: number, c: number, d: number) => number;
    readonly calculateAgreement: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly calculateSignature: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly collectAppStateKeyIds: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly decodeAppStateRecord: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly decodeNode: (a: number, b: number, c: number) => void;
    readonly encodeAppStateMutation: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => void;
    readonly encodeJid: (a: number, b: number) => void;
    readonly encodeNode: (a: number, b: number) => void;
    readonly expandAppStateKeys: (a: number, b: number) => number;
    readonly expandedappstatekeys_indexKey: (a: number) => number;
    readonly expandedappstatekeys_patchMacKey: (a: number) => number;
    readonly expandedappstatekeys_snapshotMacKey: (a: number) => number;
    readonly expandedappstatekeys_valueEncryptionKey: (a: number) => number;
    readonly expandedappstatekeys_valueMacKey: (a: number) => number;
    readonly generateContentMac: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly generateIdentityKeyPair: () => number;
    readonly generateIndexMac: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly generatePatchMac: (a: number, b: number, c: number, d: number, e: number, f: bigint, g: number, h: number, i: number, j: number) => void;
    readonly generatePreKey: (a: number) => number;
    readonly generateRegistrationId: () => number;
    readonly generateSignedPreKey: (a: number, b: number, c: number) => void;
    readonly generateSnapshotMac: (a: number, b: number, c: number, d: bigint, e: number, f: number, g: number, h: number) => void;
    readonly getEnabledFeatures: () => number;
    readonly getPreKeyMessageIdentityKey: (a: number, b: number) => number;
    readonly getPublicFromPrivateKey: (a: number, b: number, c: number) => void;
    readonly getWAConnHeader: () => number;
    readonly groupcipher_decrypt: (a: number, b: number, c: number) => number;
    readonly groupcipher_encrypt: (a: number, b: number, c: number) => number;
    readonly groupcipher_new: (a: number, b: number, c: number, d: number) => number;
    readonly groupsessionbuilder_create: (a: number, b: number) => number;
    readonly groupsessionbuilder_new: (a: number) => number;
    readonly groupsessionbuilder_process: (a: number, b: number, c: number) => number;
    readonly hasLogger: () => number;
    readonly hkdf: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly hmacSign: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly internalbinarynode_attrs: (a: number) => number;
    readonly internalbinarynode_content: (a: number) => number;
    readonly internalbinarynode_set_attrs: (a: number, b: number) => void;
    readonly internalbinarynode_set_content: (a: number, b: number) => void;
    readonly internalbinarynode_tag: (a: number, b: number) => void;
    readonly internalbinarynode_toJSON: (a: number) => number;
    readonly isADJid: (a: number, b: number) => number;
    readonly isBotJid: (a: number, b: number) => number;
    readonly isBroadcastListJid: (a: number, b: number) => number;
    readonly isGroupJid: (a: number, b: number) => number;
    readonly isHostedJid: (a: number, b: number) => number;
    readonly isLidJid: (a: number, b: number) => number;
    readonly isMessengerJid: (a: number, b: number) => number;
    readonly isNewsletterJid: (a: number, b: number) => number;
    readonly isStatusBroadcastJid: (a: number, b: number) => number;
    readonly isUserJid: (a: number, b: number) => number;
    readonly jidGetDevice: (a: number, b: number) => number;
    readonly jidGetServer: (a: number, b: number, c: number) => void;
    readonly jidGetUser: (a: number, b: number, c: number) => void;
    readonly jidGroup: (a: number, b: number, c: number) => void;
    readonly jidLid: (a: number, b: number, c: number) => void;
    readonly jidNewsletter: (a: number, b: number, c: number) => void;
    readonly jidNormalizedUser: (a: number, b: number, c: number) => void;
    readonly jidServerBot: (a: number) => void;
    readonly jidServerBroadcast: (a: number) => void;
    readonly jidServerGroup: (a: number) => void;
    readonly jidServerHosted: (a: number) => void;
    readonly jidServerLid: (a: number) => void;
    readonly jidServerMessenger: (a: number) => void;
    readonly jidServerNewsletter: (a: number) => void;
    readonly jidServerUser: (a: number) => void;
    readonly jidStatusBroadcast: (a: number) => void;
    readonly jidUser: (a: number, b: number, c: number) => void;
    readonly jidUserBase: (a: number, b: number, c: number) => void;
    readonly jidUserDevice: (a: number, b: number, c: number, d: number) => void;
    readonly jidWithDevice: (a: number, b: number, c: number, d: number) => void;
    readonly logMessage: (a: number, b: number, c: number, d: number) => void;
    readonly lthashantitampering_new: () => number;
    readonly lthashantitampering_subtractThenAdd: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly lthashstate_clone: (a: number) => number;
    readonly lthashstate_deleteValueMac: (a: number, b: number, c: number) => number;
    readonly lthashstate_getValueMac: (a: number, b: number, c: number) => number;
    readonly lthashstate_hasValueMac: (a: number, b: number, c: number) => number;
    readonly lthashstate_hash: (a: number) => number;
    readonly lthashstate_new: () => number;
    readonly lthashstate_setValueMac: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly lthashstate_set_hash: (a: number, b: number, c: number) => void;
    readonly lthashstate_set_version: (a: number, b: bigint) => void;
    readonly lthashstate_version: (a: number) => bigint;
    readonly md5: (a: number, b: number) => number;
    readonly noiseiksession_buildClientHello: (a: number, b: number) => void;
    readonly noiseiksession_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
    readonly noiseiksession_readServerHello: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly noisesession_authenticate: (a: number, b: number, c: number) => void;
    readonly noisesession_bufferedBytes: (a: number) => number;
    readonly noisesession_clearBuffer: (a: number) => void;
    readonly noisesession_decodeFrame: (a: number, b: number, c: number, d: number) => void;
    readonly noisesession_decrypt: (a: number, b: number, c: number, d: number) => void;
    readonly noisesession_encodeFrame: (a: number, b: number, c: number) => void;
    readonly noisesession_encodeFrameRaw: (a: number, b: number, c: number, d: number) => void;
    readonly noisesession_encrypt: (a: number, b: number, c: number, d: number) => void;
    readonly noisesession_finishInit: (a: number, b: number) => void;
    readonly noisesession_getHash: (a: number) => number;
    readonly noisesession_isFinished: (a: number) => number;
    readonly noisesession_mixIntoKey: (a: number, b: number, c: number, d: number) => void;
    readonly noisesession_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly noisesession_processHandshakeFinish: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly noisesession_processHandshakeInit: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => void;
    readonly noisexxfallbacksession_buildClientFinish: (a: number, b: number) => void;
    readonly noisexxfallbacksession_finish: (a: number, b: number) => void;
    readonly parseJid: (a: number, b: number) => number;
    readonly protocoladdress_deviceId: (a: number) => number;
    readonly protocoladdress_from: (a: number, b: number) => void;
    readonly protocoladdress_id: (a: number, b: number) => void;
    readonly protocoladdress_is: (a: number, b: number) => number;
    readonly protocoladdress_new: (a: number, b: number, c: number) => void;
    readonly protocoladdress_toString: (a: number, b: number) => void;
    readonly senderkeydistributionmessage_deserialize: (a: number, b: number, c: number) => void;
    readonly senderkeydistributionmessage_serialize: (a: number) => number;
    readonly senderkeyname_new: (a: number, b: number, c: number) => number;
    readonly senderkeyname_toString: (a: number, b: number) => void;
    readonly senderkeyrecord_deserialize: (a: number, b: number, c: number) => void;
    readonly senderkeyrecord_isEmpty: (a: number) => number;
    readonly senderkeyrecord_new: () => number;
    readonly senderkeyrecord_serialize: (a: number, b: number) => void;
    readonly sessionbuilder_initOutgoing: (a: number, b: number) => number;
    readonly sessionbuilder_new: (a: number, b: number) => number;
    readonly sessionbuilder_processPreKeyBundle: (a: number, b: number) => number;
    readonly sessioncipher_decryptPreKeyWhisperMessage: (a: number, b: number, c: number) => number;
    readonly sessioncipher_decryptWhisperMessage: (a: number, b: number, c: number) => number;
    readonly sessioncipher_encrypt: (a: number, b: number, c: number) => number;
    readonly sessioncipher_hasOpenSession: (a: number) => number;
    readonly sessionrecord_deserialize: (a: number, b: number) => void;
    readonly sessionrecord_haveOpenSession: (a: number) => number;
    readonly sessionrecord_serialize: (a: number) => number;
    readonly setLogger: (a: number, b: number) => void;
    readonly sha256: (a: number, b: number) => number;
    readonly verifySignature: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly sessioncipher_new: (a: number, b: number) => number;
    readonly aesEncryptCTR: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly generateKeyPair: () => number;
    readonly updateLogger: (a: number) => void;
    readonly __wbg_sessioncipher_free: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_895: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_1710: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_897: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
