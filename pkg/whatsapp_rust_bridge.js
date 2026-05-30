/* @ts-self-types="./whatsapp_rust_bridge.d.ts" */

export class ExpandedAppStateKeys {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ExpandedAppStateKeys.prototype);
        obj.__wbg_ptr = ptr;
        ExpandedAppStateKeysFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ExpandedAppStateKeysFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_expandedappstatekeys_free(ptr, 0);
    }
    /**
     * @returns {Uint8Array}
     */
    get indexKey() {
        const ret = wasm.expandedappstatekeys_indexKey(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @returns {Uint8Array}
     */
    get patchMacKey() {
        const ret = wasm.expandedappstatekeys_patchMacKey(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @returns {Uint8Array}
     */
    get snapshotMacKey() {
        const ret = wasm.expandedappstatekeys_snapshotMacKey(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @returns {Uint8Array}
     */
    get valueEncryptionKey() {
        const ret = wasm.expandedappstatekeys_valueEncryptionKey(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @returns {Uint8Array}
     */
    get valueMacKey() {
        const ret = wasm.expandedappstatekeys_valueMacKey(this.__wbg_ptr);
        return takeObject(ret);
    }
}
if (Symbol.dispose) ExpandedAppStateKeys.prototype[Symbol.dispose] = ExpandedAppStateKeys.prototype.free;

export class GroupCipher {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GroupCipherFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_groupcipher_free(ptr, 0);
    }
    /**
     * @param {Uint8Array} ciphertext
     * @returns {Promise<Uint8Array>}
     */
    decrypt(ciphertext) {
        const ptr0 = passArray8ToWasm0(ciphertext, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.groupcipher_decrypt(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
     * @param {Uint8Array} plaintext
     * @returns {Promise<Uint8Array>}
     */
    encrypt(plaintext) {
        const ptr0 = passArray8ToWasm0(plaintext, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.groupcipher_encrypt(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
     * @param {SignalStorage} storage
     * @param {string} group_id
     * @param {ProtocolAddress} sender
     */
    constructor(storage, group_id, sender) {
        const ptr0 = passStringToWasm0(group_id, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(sender, ProtocolAddress);
        const ret = wasm.groupcipher_new(addHeapObject(storage), ptr0, len0, sender.__wbg_ptr);
        this.__wbg_ptr = ret >>> 0;
        GroupCipherFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) GroupCipher.prototype[Symbol.dispose] = GroupCipher.prototype.free;

export class GroupSessionBuilder {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GroupSessionBuilderFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_groupsessionbuilder_free(ptr, 0);
    }
    /**
     * @param {SenderKeyName} sender_key_name
     * @returns {Promise<SenderKeyDistributionMessage>}
     */
    create(sender_key_name) {
        _assertClass(sender_key_name, SenderKeyName);
        const ret = wasm.groupsessionbuilder_create(this.__wbg_ptr, sender_key_name.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @param {SignalStorage} storage
     */
    constructor(storage) {
        const ret = wasm.groupsessionbuilder_new(addHeapObject(storage));
        this.__wbg_ptr = ret >>> 0;
        GroupSessionBuilderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {SenderKeyName} sender_key_name
     * @param {SenderKeyDistributionMessage} skdm
     * @returns {Promise<void>}
     */
    process(sender_key_name, skdm) {
        _assertClass(sender_key_name, SenderKeyName);
        _assertClass(skdm, SenderKeyDistributionMessage);
        const ret = wasm.groupsessionbuilder_process(this.__wbg_ptr, sender_key_name.__wbg_ptr, skdm.__wbg_ptr);
        return takeObject(ret);
    }
}
if (Symbol.dispose) GroupSessionBuilder.prototype[Symbol.dispose] = GroupSessionBuilder.prototype.free;

export class InternalBinaryNode {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InternalBinaryNode.prototype);
        obj.__wbg_ptr = ptr;
        InternalBinaryNodeFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InternalBinaryNodeFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_internalbinarynode_free(ptr, 0);
    }
    /**
     * @returns {{ [key: string]: string }}
     */
    get attrs() {
        const ret = wasm.internalbinarynode_attrs(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @returns {BinaryNode[] | string | Uint8Array | undefined}
     */
    get content() {
        const ret = wasm.internalbinarynode_content(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @param {{ [key: string]: string }} new_attrs
     */
    set attrs(new_attrs) {
        wasm.internalbinarynode_set_attrs(this.__wbg_ptr, addHeapObject(new_attrs));
    }
    /**
     * @param {BinaryNode[] | string | Uint8Array} new_content
     */
    set content(new_content) {
        wasm.internalbinarynode_set_content(this.__wbg_ptr, addHeapObject(new_content));
    }
    /**
     * @returns {string}
     */
    get tag() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.internalbinarynode_tag(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {any}
     */
    toJSON() {
        const ret = wasm.internalbinarynode_toJSON(this.__wbg_ptr);
        return takeObject(ret);
    }
}
if (Symbol.dispose) InternalBinaryNode.prototype[Symbol.dispose] = InternalBinaryNode.prototype.free;

export class LTHashAntiTampering {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LTHashAntiTamperingFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lthashantitampering_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.lthashantitampering_new();
        this.__wbg_ptr = ret >>> 0;
        LTHashAntiTamperingFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Uint8Array} base
     * @param {Uint8Array[]} subtract
     * @param {Uint8Array[]} add
     * @returns {Uint8Array}
     */
    subtractThenAdd(base, subtract, add) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(base, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArrayJsValueToWasm0(subtract, wasm.__wbindgen_export);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passArrayJsValueToWasm0(add, wasm.__wbindgen_export);
            const len2 = WASM_VECTOR_LEN;
            wasm.lthashantitampering_subtractThenAdd(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
if (Symbol.dispose) LTHashAntiTampering.prototype[Symbol.dispose] = LTHashAntiTampering.prototype.free;

export class LTHashState {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LTHashState.prototype);
        obj.__wbg_ptr = ptr;
        LTHashStateFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LTHashStateFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lthashstate_free(ptr, 0);
    }
    /**
     * @returns {LTHashState}
     */
    clone() {
        const ret = wasm.lthashstate_clone(this.__wbg_ptr);
        return LTHashState.__wrap(ret);
    }
    /**
     * @param {string} index_mac_base64
     * @returns {boolean}
     */
    deleteValueMac(index_mac_base64) {
        const ptr0 = passStringToWasm0(index_mac_base64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.lthashstate_deleteValueMac(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * @param {string} index_mac_base64
     * @returns {Uint8Array | undefined}
     */
    getValueMac(index_mac_base64) {
        const ptr0 = passStringToWasm0(index_mac_base64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.lthashstate_getValueMac(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
     * @param {string} index_mac_base64
     * @returns {boolean}
     */
    hasValueMac(index_mac_base64) {
        const ptr0 = passStringToWasm0(index_mac_base64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.lthashstate_hasValueMac(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * @returns {Uint8Array}
     */
    get hash() {
        const ret = wasm.lthashstate_hash(this.__wbg_ptr);
        return takeObject(ret);
    }
    constructor() {
        const ret = wasm.lthashstate_new();
        this.__wbg_ptr = ret >>> 0;
        LTHashStateFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {string} index_mac_base64
     * @param {Uint8Array} value_mac
     */
    setValueMac(index_mac_base64, value_mac) {
        const ptr0 = passStringToWasm0(index_mac_base64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(value_mac, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.lthashstate_setValueMac(this.__wbg_ptr, ptr0, len0, ptr1, len1);
    }
    /**
     * @param {Uint8Array} hash
     */
    set hash(hash) {
        const ptr0 = passArray8ToWasm0(hash, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.lthashstate_set_hash(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {bigint} version
     */
    set version(version) {
        wasm.lthashstate_set_version(this.__wbg_ptr, version);
    }
    /**
     * @returns {bigint}
     */
    get version() {
        const ret = wasm.lthashstate_version(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
}
if (Symbol.dispose) LTHashState.prototype[Symbol.dispose] = LTHashState.prototype.free;

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
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NoiseIkSessionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_noiseiksession_free(ptr, 0);
    }
    /**
     * Build the IK ClientHello bytes (framed, ready to send).
     * @returns {Uint8Array}
     */
    buildClientHello() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.noiseiksession_buildClientHello(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new IK session.
     * @param staticPub      Client's static public key (33 bytes, 0x05 prefix)
     * @param staticPriv     Client's static private key (32 bytes)
     * @param serverStaticPub Server's static public key (32 bytes, no prefix)
     * @param clientPayload  The payload to send 0-RTT (e.g. client hello proto)
     * @param prologue       Noise prologue bytes (WA header)
     * @param {Uint8Array} static_pub
     * @param {Uint8Array} static_priv
     * @param {Uint8Array} server_static_pub
     * @param {Uint8Array} client_payload
     * @param {Uint8Array} prologue
     */
    constructor(static_pub, static_priv, server_static_pub, client_payload, prologue) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(static_pub, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(static_priv, wasm.__wbindgen_export);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passArray8ToWasm0(server_static_pub, wasm.__wbindgen_export);
            const len2 = WASM_VECTOR_LEN;
            const ptr3 = passArray8ToWasm0(client_payload, wasm.__wbindgen_export);
            const len3 = WASM_VECTOR_LEN;
            const ptr4 = passArray8ToWasm0(prologue, wasm.__wbindgen_export);
            const len4 = WASM_VECTOR_LEN;
            wasm.noiseiksession_new(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            NoiseIkSessionFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Process the server's response.
     * Returns a JS object: `{ success: true, writeCipher, readCipher }` on IK success,
     * or `{ success: false, fallbackSession: NoiseSession }` when the server requests XX fallback.
     * @param {Uint8Array} response_bytes
     * @param {Uint8Array | null} [routing_info]
     * @returns {any}
     */
    readServerHello(response_bytes, routing_info) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(response_bytes, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            var ptr1 = isLikeNone(routing_info) ? 0 : passArray8ToWasm0(routing_info, wasm.__wbindgen_export);
            var len1 = WASM_VECTOR_LEN;
            wasm.noiseiksession_readServerHello(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
if (Symbol.dispose) NoiseIkSession.prototype[Symbol.dispose] = NoiseIkSession.prototype.free;

/**
 * NoiseSession implements the Noise_XX_25519_AESGCM_SHA256 protocol pattern
 * with combined binary encoding/decoding operations for reduced WASM boundary crossings.
 */
export class NoiseSession {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(NoiseSession.prototype);
        obj.__wbg_ptr = ptr;
        NoiseSessionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NoiseSessionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_noisesession_free(ptr, 0);
    }
    /**
     * Updates the session hash with the given data (no-op after handshake).
     * @param {Uint8Array} data
     */
    authenticate(data) {
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.noisesession_authenticate(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number}
     */
    get bufferedBytes() {
        const ret = wasm.noisesession_bufferedBytes(this.__wbg_ptr);
        return ret >>> 0;
    }
    clearBuffer() {
        wasm.noisesession_clearBuffer(this.__wbg_ptr);
    }
    /**
     * @param {Uint8Array} new_data
     * @returns {Array<any>}
     */
    decodeFrame(new_data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(new_data, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.noisesession_decodeFrame(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {Uint8Array} ciphertext
     * @returns {Uint8Array}
     */
    decrypt(ciphertext) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(ciphertext, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.noisesession_decrypt(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {EncodingNode} node
     * @returns {Uint8Array}
     */
    encodeFrame(node) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.noisesession_encodeFrame(retptr, this.__wbg_ptr, addHeapObject(node));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {Uint8Array} data
     * @returns {Uint8Array}
     */
    encodeFrameRaw(data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.noisesession_encodeFrameRaw(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {Uint8Array} plaintext
     * @returns {Uint8Array}
     */
    encrypt(plaintext) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(plaintext, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.noisesession_encrypt(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    finishInit() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.noisesession_finishInit(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {Uint8Array}
     */
    getHash() {
        const ret = wasm.noisesession_getHash(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @returns {boolean}
     */
    get isFinished() {
        const ret = wasm.noisesession_isFinished(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {Uint8Array} data
     */
    mixIntoKey(data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.noisesession_mixIntoKey(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {Uint8Array} public_key
     * @param {Uint8Array} noise_header
     * @param {Uint8Array | null} [routing_info]
     */
    constructor(public_key, noise_header, routing_info) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(public_key, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(noise_header, wasm.__wbindgen_export);
            const len1 = WASM_VECTOR_LEN;
            var ptr2 = isLikeNone(routing_info) ? 0 : passArray8ToWasm0(routing_info, wasm.__wbindgen_export);
            var len2 = WASM_VECTOR_LEN;
            wasm.noisesession_new(retptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            NoiseSessionFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {Uint8Array} noise_public_key
     * @param {Uint8Array} noise_private_key
     * @param {Uint8Array} server_ephemeral
     * @returns {Uint8Array}
     */
    processHandshakeFinish(noise_public_key, noise_private_key, server_ephemeral) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(noise_public_key, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(noise_private_key, wasm.__wbindgen_export);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passArray8ToWasm0(server_ephemeral, wasm.__wbindgen_export);
            const len2 = WASM_VECTOR_LEN;
            wasm.noisesession_processHandshakeFinish(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {Uint8Array} server_ephemeral
     * @param {Uint8Array} server_static_encrypted
     * @param {Uint8Array} server_payload_encrypted
     * @param {Uint8Array} private_key
     * @returns {Uint8Array}
     */
    processHandshakeInit(server_ephemeral, server_static_encrypted, server_payload_encrypted, private_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(server_ephemeral, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(server_static_encrypted, wasm.__wbindgen_export);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passArray8ToWasm0(server_payload_encrypted, wasm.__wbindgen_export);
            const len2 = WASM_VECTOR_LEN;
            const ptr3 = passArray8ToWasm0(private_key, wasm.__wbindgen_export);
            const len3 = WASM_VECTOR_LEN;
            wasm.noisesession_processHandshakeInit(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
if (Symbol.dispose) NoiseSession.prototype[Symbol.dispose] = NoiseSession.prototype.free;

/**
 * Noise XXfallback session — used when an IK attempt is rejected by the server.
 * Reuses the ephemeral already on the wire to avoid an extra round-trip.
 */
export class NoiseXxFallbackSession {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(NoiseXxFallbackSession.prototype);
        obj.__wbg_ptr = ptr;
        NoiseXxFallbackSessionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NoiseXxFallbackSessionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_noisexxfallbacksession_free(ptr, 0);
    }
    /**
     * Build the client finish message (send this over the wire).
     * @returns {Uint8Array}
     */
    buildClientFinish() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.noisexxfallbacksession_buildClientFinish(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Finalize the handshake — returns a ready `NoiseSession`.
     * @returns {NoiseSession}
     */
    finish() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.noisexxfallbacksession_finish(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return NoiseSession.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
if (Symbol.dispose) NoiseXxFallbackSession.prototype[Symbol.dispose] = NoiseXxFallbackSession.prototype.free;

export class ProtocolAddress {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ProtocolAddress.prototype);
        obj.__wbg_ptr = ptr;
        ProtocolAddressFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ProtocolAddressFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_protocoladdress_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get deviceId() {
        const ret = wasm.protocoladdress_deviceId(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {string} encoded
     * @returns {ProtocolAddress}
     */
    static from(encoded) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.protocoladdress_from(retptr, addHeapObject(encoded));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return ProtocolAddress.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    get id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.protocoladdress_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {ProtocolAddress} other
     * @returns {boolean}
     */
    is(other) {
        _assertClass(other, ProtocolAddress);
        const ret = wasm.protocoladdress_is(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {string} id
     * @param {number} device_id
     */
    constructor(id, device_id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.protocoladdress_new(retptr, addHeapObject(id), addHeapObject(device_id));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            ProtocolAddressFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.protocoladdress_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) ProtocolAddress.prototype[Symbol.dispose] = ProtocolAddress.prototype.free;

export class SenderKeyDistributionMessage {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SenderKeyDistributionMessage.prototype);
        obj.__wbg_ptr = ptr;
        SenderKeyDistributionMessageFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SenderKeyDistributionMessageFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_senderkeydistributionmessage_free(ptr, 0);
    }
    /**
     * @param {Uint8Array} serialized
     * @returns {SenderKeyDistributionMessage}
     */
    static deserialize(serialized) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(serialized, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.senderkeydistributionmessage_deserialize(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return SenderKeyDistributionMessage.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {Uint8Array}
     */
    serialize() {
        const ret = wasm.senderkeydistributionmessage_serialize(this.__wbg_ptr);
        return takeObject(ret);
    }
}
if (Symbol.dispose) SenderKeyDistributionMessage.prototype[Symbol.dispose] = SenderKeyDistributionMessage.prototype.free;

export class SenderKeyName {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SenderKeyNameFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_senderkeyname_free(ptr, 0);
    }
    /**
     * @param {string} group_id
     * @param {ProtocolAddress} sender
     */
    constructor(group_id, sender) {
        const ptr0 = passStringToWasm0(group_id, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(sender, ProtocolAddress);
        const ret = wasm.senderkeyname_new(ptr0, len0, sender.__wbg_ptr);
        this.__wbg_ptr = ret >>> 0;
        SenderKeyNameFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {string}
     */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.senderkeyname_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) SenderKeyName.prototype[Symbol.dispose] = SenderKeyName.prototype.free;

export class SenderKeyRecord {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SenderKeyRecord.prototype);
        obj.__wbg_ptr = ptr;
        SenderKeyRecordFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SenderKeyRecordFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_senderkeyrecord_free(ptr, 0);
    }
    /**
     * @param {Uint8Array} serialized
     * @returns {SenderKeyRecord}
     */
    static deserialize(serialized) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(serialized, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.senderkeyrecord_deserialize(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return SenderKeyRecord.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {boolean}
     */
    isEmpty() {
        const ret = wasm.senderkeyrecord_isEmpty(this.__wbg_ptr);
        return ret !== 0;
    }
    constructor() {
        const ret = wasm.senderkeyrecord_new();
        this.__wbg_ptr = ret >>> 0;
        SenderKeyRecordFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Uint8Array}
     */
    serialize() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.senderkeyrecord_serialize(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
if (Symbol.dispose) SenderKeyRecord.prototype[Symbol.dispose] = SenderKeyRecord.prototype.free;

export class SessionBuilder {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SessionBuilderFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sessionbuilder_free(ptr, 0);
    }
    /**
     * @param {PreKeyBundleInput} bundle_input
     * @returns {Promise<void>}
     */
    initOutgoing(bundle_input) {
        const ret = wasm.sessionbuilder_initOutgoing(this.__wbg_ptr, addHeapObject(bundle_input));
        return takeObject(ret);
    }
    /**
     * @param {SignalStorage} storage
     * @param {ProtocolAddress} remote_address
     */
    constructor(storage, remote_address) {
        _assertClass(remote_address, ProtocolAddress);
        const ret = wasm.sessionbuilder_new(addHeapObject(storage), remote_address.__wbg_ptr);
        this.__wbg_ptr = ret >>> 0;
        SessionBuilderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {PreKeyBundleInput} bundle_input
     * @returns {Promise<void>}
     */
    processPreKeyBundle(bundle_input) {
        const ret = wasm.sessionbuilder_processPreKeyBundle(this.__wbg_ptr, addHeapObject(bundle_input));
        return takeObject(ret);
    }
}
if (Symbol.dispose) SessionBuilder.prototype[Symbol.dispose] = SessionBuilder.prototype.free;

export class SessionCipher {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SessionCipherFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sessioncipher_free(ptr, 0);
    }
    /**
     * @param {Uint8Array} ciphertext
     * @returns {Promise<Uint8Array>}
     */
    decryptPreKeyWhisperMessage(ciphertext) {
        const ptr0 = passArray8ToWasm0(ciphertext, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sessioncipher_decryptPreKeyWhisperMessage(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
     * @param {Uint8Array} ciphertext
     * @returns {Promise<Uint8Array>}
     */
    decryptWhisperMessage(ciphertext) {
        const ptr0 = passArray8ToWasm0(ciphertext, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sessioncipher_decryptWhisperMessage(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
     * @param {Uint8Array} plaintext
     * @returns {Promise<{ type: number; body: Uint8Array }>}
     */
    encrypt(plaintext) {
        const ptr0 = passArray8ToWasm0(plaintext, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sessioncipher_encrypt(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
     * @returns {Promise<boolean>}
     */
    hasOpenSession() {
        const ret = wasm.sessioncipher_hasOpenSession(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @param {SignalStorage} storage
     * @param {ProtocolAddress} remote_address
     */
    constructor(storage, remote_address) {
        _assertClass(remote_address, ProtocolAddress);
        const ret = wasm.sessionbuilder_new(addHeapObject(storage), remote_address.__wbg_ptr);
        this.__wbg_ptr = ret >>> 0;
        SessionCipherFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) SessionCipher.prototype[Symbol.dispose] = SessionCipher.prototype.free;

export class SessionRecord {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SessionRecord.prototype);
        obj.__wbg_ptr = ptr;
        SessionRecordFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SessionRecordFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sessionrecord_free(ptr, 0);
    }
    /**
     * @param {any} val
     * @returns {SessionRecord}
     */
    static deserialize(val) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sessionrecord_deserialize(retptr, addHeapObject(val));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return SessionRecord.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {boolean}
     */
    haveOpenSession() {
        const ret = wasm.sessionrecord_haveOpenSession(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {Uint8Array}
     */
    serialize() {
        const ret = wasm.sessionrecord_serialize(this.__wbg_ptr);
        return takeObject(ret);
    }
}
if (Symbol.dispose) SessionRecord.prototype[Symbol.dispose] = SessionRecord.prototype.free;

/**
 * @param {KeyPair} key_pair
 * @returns {Uint8Array}
 */
export function _serializeIdentityKeyPair(key_pair) {
    const ret = wasm._serializeIdentityKeyPair(addHeapObject(key_pair));
    return takeObject(ret);
}

/**
 * Decrypts AES-256-CBC; IV is the first 16 bytes of `buffer`.
 * @param {Uint8Array} buffer
 * @param {Uint8Array} key
 * @returns {Uint8Array}
 */
export function aesDecrypt(buffer, key) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.aesDecrypt(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {Uint8Array} ciphertext
 * @param {Uint8Array} key
 * @param {Uint8Array} iv
 * @returns {Uint8Array}
 */
export function aesDecryptCTR(ciphertext, key, iv) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(ciphertext, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(iv, wasm.__wbindgen_export);
        const len2 = WASM_VECTOR_LEN;
        wasm.aesDecryptCTR(retptr, ptr0, len0, ptr1, len1, ptr2, len2);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Decrypts AES-256-GCM; expects auth tag (16 bytes) appended to ciphertext.
 * @param {Uint8Array} ciphertext_with_tag
 * @param {Uint8Array} key
 * @param {Uint8Array} iv
 * @param {Uint8Array} additional_data
 * @returns {Uint8Array}
 */
export function aesDecryptGCM(ciphertext_with_tag, key, iv, additional_data) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(ciphertext_with_tag, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(iv, wasm.__wbindgen_export);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passArray8ToWasm0(additional_data, wasm.__wbindgen_export);
        const len3 = WASM_VECTOR_LEN;
        wasm.aesDecryptGCM(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Decrypts AES-256-CBC with an explicit IV.
 * @param {Uint8Array} buffer
 * @param {Uint8Array} key
 * @param {Uint8Array} iv
 * @returns {Uint8Array}
 */
export function aesDecryptWithIV(buffer, key, iv) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(iv, wasm.__wbindgen_export);
        const len2 = WASM_VECTOR_LEN;
        wasm.aesDecryptWithIV(retptr, ptr0, len0, ptr1, len1, ptr2, len2);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Encrypts AES-256-CBC with a given IV (no IV prefix in output).
 * @param {Uint8Array} buffer
 * @param {Uint8Array} key
 * @param {Uint8Array} iv
 * @returns {Uint8Array}
 */
export function aesEncrypWithIV(buffer, key, iv) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(iv, wasm.__wbindgen_export);
        const len2 = WASM_VECTOR_LEN;
        wasm.aesEncrypWithIV(retptr, ptr0, len0, ptr1, len1, ptr2, len2);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Encrypts AES-256-CBC with a random IV; IV is prepended to output.
 * @param {Uint8Array} buffer
 * @param {Uint8Array} key
 * @returns {Uint8Array}
 */
export function aesEncrypt(buffer, key) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.aesEncrypt(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {Uint8Array} plaintext
 * @param {Uint8Array} key
 * @param {Uint8Array} iv
 * @returns {Uint8Array}
 */
export function aesEncryptCTR(plaintext, key, iv) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(plaintext, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(iv, wasm.__wbindgen_export);
        const len2 = WASM_VECTOR_LEN;
        wasm.aesDecryptCTR(retptr, ptr0, len0, ptr1, len1, ptr2, len2);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Encrypts with AES-256-GCM; auth tag (16 bytes) is appended to ciphertext.
 * @param {Uint8Array} plaintext
 * @param {Uint8Array} key
 * @param {Uint8Array} iv
 * @param {Uint8Array} additional_data
 * @returns {Uint8Array}
 */
export function aesEncryptGCM(plaintext, key, iv, additional_data) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(plaintext, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(iv, wasm.__wbindgen_export);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passArray8ToWasm0(additional_data, wasm.__wbindgen_export);
        const len3 = WASM_VECTOR_LEN;
        wasm.aesEncryptGCM(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Returns true if both JIDs refer to the same user (ignoring device).
 * @param {string} a
 * @param {string} b
 * @returns {boolean}
 */
export function areSameUser(a, b) {
    const ptr0 = passStringToWasm0(a, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(b, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.areSameUser(ptr0, len0, ptr1, len1);
    return ret !== 0;
}

/**
 * @param {Uint8Array} public_key_bytes
 * @param {Uint8Array} private_key_bytes
 * @returns {Uint8Array}
 */
export function calculateAgreement(public_key_bytes, private_key_bytes) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(public_key_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(private_key_bytes, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.calculateAgreement(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {Uint8Array} private_key_bytes
 * @param {Uint8Array} message
 * @returns {Uint8Array}
 */
export function calculateSignature(private_key_bytes, message) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(private_key_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(message, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.calculateSignature(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Extract all unique key IDs from a list of patches (and optionally a snapshot).
 *
 * @param snapshotBytes  Optional protobuf-encoded `SyncdSnapshot` bytes (pass empty Uint8Array to skip)
 * @param patchesBytes   Array of protobuf-encoded `SyncdPatch` bytes
 * @returns              Array of key-ID byte arrays that need to be fetched
 * @param {Uint8Array} snapshot_bytes
 * @param {Uint8Array[]} patches_bytes
 * @returns {Array<any>}
 */
export function collectAppStateKeyIds(snapshot_bytes, patches_bytes) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(snapshot_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayJsValueToWasm0(patches_bytes, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.collectAppStateKeyIds(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Decrypt and decode a single app-state record.
 *
 * @param recordBytes   Protobuf-encoded `SyncdRecord` bytes
 * @param keys          Expanded app-state keys (from `expandAppStateKeys`)
 * @param keyId         The key ID bytes (used for MAC validation)
 * @param operation     0 = SET, 1 = REMOVE
 * @param validateMacs  Whether to verify MACs (set false to skip for speed)
 * @param {Uint8Array} record_bytes
 * @param {ExpandedAppStateKeys} keys
 * @param {Uint8Array} key_id
 * @param {number} operation
 * @param {boolean} validate_macs
 * @returns {DecodedMutation}
 */
export function decodeAppStateRecord(record_bytes, keys, key_id, operation, validate_macs) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(record_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(keys, ExpandedAppStateKeys);
        const ptr1 = passArray8ToWasm0(key_id, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.decodeAppStateRecord(retptr, ptr0, len0, keys.__wbg_ptr, ptr1, len1, operation, validate_macs);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {Uint8Array} data
 * @returns {InternalBinaryNode}
 */
export function decodeNode(data) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.decodeNode(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return InternalBinaryNode.__wrap(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Encode and encrypt a mutation into a `SyncdMutation` (ready to include in a patch).
 *
 * @param operation     0 = SET, 1 = REMOVE
 * @param indexBytes    The index as JSON bytes, e.g. `["contact","123@s.whatsapp.net"]`
 * @param actionBytes   Protobuf-encoded `SyncActionValue` bytes
 * @param keys          Expanded app-state keys (from `expandAppStateKeys`)
 * @param keyId         The key ID bytes
 * @param iv            16-byte IV for AES-CBC encryption (use random bytes)
 * @param {number} operation
 * @param {Uint8Array} index_bytes
 * @param {Uint8Array} action_bytes
 * @param {ExpandedAppStateKeys} keys
 * @param {Uint8Array} key_id
 * @param {Uint8Array} iv
 * @returns {EncodedMutation}
 */
export function encodeAppStateMutation(operation, index_bytes, action_bytes, keys, key_id, iv) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(index_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(action_bytes, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        _assertClass(keys, ExpandedAppStateKeys);
        const ptr2 = passArray8ToWasm0(key_id, wasm.__wbindgen_export);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passArray8ToWasm0(iv, wasm.__wbindgen_export);
        const len3 = WASM_VECTOR_LEN;
        wasm.encodeAppStateMutation(retptr, operation, ptr0, len0, ptr1, len1, keys.__wbg_ptr, ptr2, len2, ptr3, len3);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Encode a JidInfo back to its canonical string.
 * @param {JidInfo} info
 * @returns {string}
 */
export function encodeJid(info) {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.encodeJid(retptr, addHeapObject(info));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {EncodingNode} node_val
 * @returns {Uint8Array}
 */
export function encodeNode(node_val) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.encodeNode(retptr, addHeapObject(node_val));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {Uint8Array} key_data
 * @returns {ExpandedAppStateKeys}
 */
export function expandAppStateKeys(key_data) {
    const ptr0 = passArray8ToWasm0(key_data, wasm.__wbindgen_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.expandAppStateKeys(ptr0, len0);
    return ExpandedAppStateKeys.__wrap(ret);
}

/**
 * @param {number} operation
 * @param {Uint8Array} data
 * @param {Uint8Array} key_id
 * @param {Uint8Array} key
 * @returns {Uint8Array}
 */
export function generateContentMac(operation, data, key_id, key) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key_id, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len2 = WASM_VECTOR_LEN;
        wasm.generateContentMac(retptr, operation, ptr0, len0, ptr1, len1, ptr2, len2);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @returns {KeyPair}
 */
export function generateIdentityKeyPair() {
    const ret = wasm.generateIdentityKeyPair();
    return takeObject(ret);
}

/**
 * @param {Uint8Array} index_bytes
 * @param {Uint8Array} key
 * @returns {Uint8Array}
 */
export function generateIndexMac(index_bytes, key) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(index_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.generateIndexMac(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @returns {KeyPair}
 */
export function generateKeyPair() {
    const ret = wasm.generateIdentityKeyPair();
    return takeObject(ret);
}

/**
 * @param {Uint8Array} snapshot_mac
 * @param {Uint8Array[]} value_macs
 * @param {bigint} version
 * @param {string} name
 * @param {Uint8Array} key
 * @returns {Uint8Array}
 */
export function generatePatchMac(snapshot_mac, value_macs, version, name, key) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(snapshot_mac, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayJsValueToWasm0(value_macs, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len3 = WASM_VECTOR_LEN;
        wasm.generatePatchMac(retptr, ptr0, len0, ptr1, len1, version, ptr2, len2, ptr3, len3);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {number} key_id
 * @returns {PreKey}
 */
export function generatePreKey(key_id) {
    const ret = wasm.generatePreKey(key_id);
    return takeObject(ret);
}

/**
 * @returns {number}
 */
export function generateRegistrationId() {
    const ret = wasm.generateRegistrationId();
    return ret >>> 0;
}

/**
 * @param {KeyPair} identity_key_pair
 * @param {number} signed_key_id
 * @returns {SignedPreKey}
 */
export function generateSignedPreKey(identity_key_pair, signed_key_id) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.generateSignedPreKey(retptr, addHeapObject(identity_key_pair), signed_key_id);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {Uint8Array} lt_hash
 * @param {bigint} version
 * @param {string} name
 * @param {Uint8Array} key
 * @returns {Uint8Array}
 */
export function generateSnapshotMac(lt_hash, version, name, key) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(lt_hash, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len2 = WASM_VECTOR_LEN;
        wasm.generateSnapshotMac(retptr, ptr0, len0, version, ptr1, len1, ptr2, len2);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Returns which optional features are enabled in this build.
 * Use this to conditionally call feature-gated functions.
 * @returns {EnabledFeatures}
 */
export function getEnabledFeatures() {
    const ret = wasm.getEnabledFeatures();
    return takeObject(ret);
}

/**
 * Extracts the sender's identity key from a PreKeySignalMessage for identity-change detection.
 * Returns `undefined` if parsing fails or the message is not a valid PreKeyMessage.
 * @param {Uint8Array} ciphertext
 * @returns {Uint8Array | undefined}
 */
export function getPreKeyMessageIdentityKey(ciphertext) {
    const ptr0 = passArray8ToWasm0(ciphertext, wasm.__wbindgen_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.getPreKeyMessageIdentityKey(ptr0, len0);
    return takeObject(ret);
}

/**
 * @param {Uint8Array} private_key_bytes
 * @returns {Uint8Array}
 */
export function getPublicFromPrivateKey(private_key_bytes) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(private_key_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.getPublicFromPrivateKey(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Returns the WhatsApp connection header (WA_CONN_HEADER).
 * This is the 4-byte header sent at the start of a WebSocket connection.
 * @returns {Uint8Array}
 */
export function getWAConnHeader() {
    const ret = wasm.getWAConnHeader();
    return takeObject(ret);
}

/**
 * @returns {boolean}
 */
export function hasLogger() {
    const ret = wasm.hasLogger();
    return ret !== 0;
}

/**
 * @param {Uint8Array} buffer
 * @param {number} expanded_length
 * @param {HkdfInfo} info
 * @returns {Uint8Array}
 */
export function hkdf(buffer, expanded_length, info) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.hkdf(retptr, ptr0, len0, expanded_length, addHeapObject(info));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {Uint8Array} buffer
 * @param {Uint8Array} key
 * @returns {Uint8Array}
 */
export function hmacSign(buffer, key) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(key, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.hmacSign(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Returns true if the JID belongs to a multi-device (AD) session (device > 0).
 * @param {string} jid_str
 * @returns {boolean}
 */
export function isADJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.isADJid(ptr0, len0);
    return ret !== 0;
}

/**
 * Returns true if the JID is a WhatsApp bot.
 * @param {string} jid_str
 * @returns {boolean}
 */
export function isBotJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.isBotJid(ptr0, len0);
    return ret !== 0;
}

/**
 * Returns true if the JID is a broadcast list (not status).
 * @param {string} jid_str
 * @returns {boolean}
 */
export function isBroadcastListJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.isBroadcastListJid(ptr0, len0);
    return ret !== 0;
}

/**
 * Returns true if the JID is a group (g.us).
 * @param {string} jid_str
 * @returns {boolean}
 */
export function isGroupJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.isGroupJid(ptr0, len0);
    return ret !== 0;
}

/**
 * Returns true if the JID is a hosted/Cloud API device.
 * @param {string} jid_str
 * @returns {boolean}
 */
export function isHostedJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.isHostedJid(ptr0, len0);
    return ret !== 0;
}

/**
 * Returns true if the JID is a LID-based user (lid server).
 * @param {string} jid_str
 * @returns {boolean}
 */
export function isLidJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.isLidJid(ptr0, len0);
    return ret !== 0;
}

/**
 * Returns true if the JID is a Meta Messenger bridged contact.
 * @param {string} jid_str
 * @returns {boolean}
 */
export function isMessengerJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.isMessengerJid(ptr0, len0);
    return ret !== 0;
}

/**
 * Returns true if the JID is a newsletter (channel).
 * @param {string} jid_str
 * @returns {boolean}
 */
export function isNewsletterJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.isNewsletterJid(ptr0, len0);
    return ret !== 0;
}

/**
 * Returns true if the JID is the status broadcast ("status@broadcast").
 * @param {string} jid_str
 * @returns {boolean}
 */
export function isStatusBroadcastJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.isStatusBroadcastJid(ptr0, len0);
    return ret !== 0;
}

/**
 * Returns true if the JID is a regular user (s.whatsapp.net).
 * @param {string} jid_str
 * @returns {boolean}
 */
export function isUserJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.isUserJid(ptr0, len0);
    return ret !== 0;
}

/**
 * Extract the device ID from a JID string (0 = primary device).
 * @param {string} jid_str
 * @returns {number | undefined}
 */
export function jidGetDevice(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.jidGetDevice(ptr0, len0);
    return ret === 0xFFFFFF ? undefined : ret;
}

/**
 * Extract the server domain from a JID string.
 * @param {string} jid_str
 * @returns {string | undefined}
 */
export function jidGetServer(jid_str) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.jidGetServer(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        let v2;
        if (r0 !== 0) {
            v2 = getStringFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
        }
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Extract the user part (phone / group-id) from a JID string.
 * @param {string} jid_str
 * @returns {string | undefined}
 */
export function jidGetUser(jid_str) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.jidGetUser(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        let v2;
        if (r0 !== 0) {
            v2 = getStringFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
        }
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Create a group JID: "groupId@g.us"
 * @param {string} group_id
 * @returns {string}
 */
export function jidGroup(group_id) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(group_id, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.jidGroup(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Create a LID JID: "lid@lid"
 * @param {string} lid
 * @returns {string}
 */
export function jidLid(lid) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(lid, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.jidLid(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Create a newsletter (channel) JID: "id@newsletter"
 * @param {string} id
 * @returns {string}
 */
export function jidNewsletter(id) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.jidNewsletter(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Normalize a JID to its primary user form (device = 0, agent = 0).
 * "123@s.whatsapp.net:5" → "123@s.whatsapp.net"
 * @param {string} jid_str
 * @returns {string | undefined}
 */
export function jidNormalizedUser(jid_str) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.jidNormalizedUser(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        let v2;
        if (r0 !== 0) {
            v2 = getStringFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
        }
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @returns {string}
 */
export function jidServerBot() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.jidServerBot(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {string}
 */
export function jidServerBroadcast() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.jidServerBroadcast(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {string}
 */
export function jidServerGroup() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.jidServerGroup(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {string}
 */
export function jidServerHosted() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.jidServerHosted(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {string}
 */
export function jidServerLid() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.jidServerLid(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {string}
 */
export function jidServerMessenger() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.jidServerMessenger(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {string}
 */
export function jidServerNewsletter() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.jidServerNewsletter(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {string}
 */
export function jidServerUser() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.jidServerUser(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Returns the status broadcast JID: "status@broadcast"
 * @returns {string}
 */
export function jidStatusBroadcast() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.jidStatusBroadcast(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Create a user JID: "phone@s.whatsapp.net"
 * @param {string} phone
 * @returns {string}
 */
export function jidUser(phone) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(phone, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.jidUser(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Returns the base user part stripping any ":device" suffix.
 * "123:4@s.whatsapp.net" → "123"
 * @param {string} jid_str
 * @returns {string | undefined}
 */
export function jidUserBase(jid_str) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.jidUserBase(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        let v2;
        if (r0 !== 0) {
            v2 = getStringFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
        }
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Create a user JID with a specific device ID.
 * @param {string} phone
 * @param {number} device
 * @returns {string}
 */
export function jidUserDevice(phone, device) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(phone, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.jidUserDevice(retptr, ptr0, len0, device);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Change the device ID on an existing JID string.
 * @param {string} jid_str
 * @param {number} device
 * @returns {string | undefined}
 */
export function jidWithDevice(jid_str, device) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.jidWithDevice(retptr, ptr0, len0, device);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        let v2;
        if (r0 !== 0) {
            v2 = getStringFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
        }
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {string} level
 * @param {string} message
 */
export function logMessage(level, message) {
    const ptr0 = passStringToWasm0(level, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(message, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    wasm.logMessage(ptr0, len0, ptr1, len1);
}

/**
 * @param {Uint8Array} buffer
 * @returns {Uint8Array}
 */
export function md5(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.md5(ptr0, len0);
    return takeObject(ret);
}

/**
 * Parse a JID string into its components.
 * Accepts: "user@server", "user@server:device", "user.agent:device@server"
 * @param {string} jid_str
 * @returns {JidInfo | undefined}
 */
export function parseJid(jid_str) {
    const ptr0 = passStringToWasm0(jid_str, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.parseJid(ptr0, len0);
    return takeObject(ret);
}

/**
 * @param {ILogger} logger
 */
export function setLogger(logger) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.setLogger(retptr, addHeapObject(logger));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {Uint8Array} buffer
 * @returns {Uint8Array}
 */
export function sha256(buffer) {
    const ptr0 = passArray8ToWasm0(buffer, wasm.__wbindgen_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.sha256(ptr0, len0);
    return takeObject(ret);
}

/**
 * @param {ILogger} logger
 */
export function updateLogger(logger) {
    wasm.updateLogger(addHeapObject(logger));
}

/**
 * @param {Uint8Array} public_key_bytes
 * @param {Uint8Array} message
 * @param {Uint8Array} signature
 * @returns {boolean}
 */
export function verifySignature(public_key_bytes, message, signature) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(public_key_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(message, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(signature, wasm.__wbindgen_export);
        const len2 = WASM_VECTOR_LEN;
        wasm.verifySignature(retptr, ptr0, len0, ptr1, len1, ptr2, len2);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return r0 !== 0;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_8c4e43fe74559d73: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        },
        __wbg_Number_04624de7d0e8332d: function(arg0) {
            const ret = Number(getObject(arg0));
            return ret;
        },
        __wbg_String_8f0eb39a4a4c2f66: function(arg0, arg1) {
            const ret = String(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_bigint_get_as_i64_8fcf4ce7f1ca72a2: function(arg0, arg1) {
            const v = getObject(arg1);
            const ret = typeof(v) === 'bigint' ? v : undefined;
            getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_boolean_get_bbbb1c18aa2f5e25: function(arg0) {
            const v = getObject(arg0);
            const ret = typeof(v) === 'boolean' ? v : undefined;
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg___wbindgen_debug_string_0bc8482c6e3508ae: function(arg0, arg1) {
            const ret = debugString(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_in_47fa6863be6f2f25: function(arg0, arg1) {
            const ret = getObject(arg0) in getObject(arg1);
            return ret;
        },
        __wbg___wbindgen_is_bigint_31b12575b56f32fc: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'bigint';
            return ret;
        },
        __wbg___wbindgen_is_function_0095a73b8b156f76: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_null_ac34f5003991759a: function(arg0) {
            const ret = getObject(arg0) === null;
            return ret;
        },
        __wbg___wbindgen_is_object_5ae8e5880f2c1fbd: function(arg0) {
            const val = getObject(arg0);
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_undefined_9e4d92534c42d778: function(arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_eq_11888390b0186270: function(arg0, arg1) {
            const ret = getObject(arg0) === getObject(arg1);
            return ret;
        },
        __wbg___wbindgen_jsval_loose_eq_9dd77d8cd6671811: function(arg0, arg1) {
            const ret = getObject(arg0) == getObject(arg1);
            return ret;
        },
        __wbg___wbindgen_number_get_8ff4255516ccad3e: function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_string_get_72fb696202c56729: function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_d9b87ff7982e3b21: function(arg0) {
            getObject(arg0)._wbg_cb_unref();
        },
        __wbg_attrs_7e03bce687b13600: function(arg0) {
            const ret = getObject(arg0).attrs;
            return addHeapObject(ret);
        },
        __wbg_call_389efe28435a9388: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_call_4708e0c13bdc8e95: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_content_5adfc5ce260fc60f: function(arg0) {
            const ret = getObject(arg0).content;
            return addHeapObject(ret);
        },
        __wbg_debug_d9ceba20b3b4bc31: function(arg0, arg1, arg2, arg3) {
            getObject(arg0).debug(getObject(arg1), arg2 === 0 ? undefined : getStringFromWasm0(arg2, arg3));
        },
        __wbg_done_57b39ecd9addfe81: function(arg0) {
            const ret = getObject(arg0).done;
            return ret;
        },
        __wbg_entries_58c7934c745daac7: function(arg0) {
            const ret = Object.entries(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_error_38f2d2b44d5fbcdd: function(arg0, arg1, arg2, arg3) {
            getObject(arg0).error(getObject(arg1), arg2 === 0 ? undefined : getStringFromWasm0(arg2, arg3));
        },
        __wbg_from_bddd64e7d5ff6941: function(arg0) {
            const ret = Array.from(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_getOurIdentity_ff06142cf4d771fd: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).getOurIdentity();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_getOurRegistrationId_9476cabd6615435c: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).getOurRegistrationId();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_getRandomValues_1c61fac11405ffdc: function() { return handleError(function (arg0, arg1) {
            globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_getRandomValues_2a91986308c74a93: function() { return handleError(function (arg0, arg1) {
            globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_get_9b94d73e6221f75c: function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return addHeapObject(ret);
        },
        __wbg_get_b3ed3ad4be2bc8ac: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_get_with_ref_key_1dc361bd10053bfe: function(arg0, arg1) {
            const ret = getObject(arg0)[getObject(arg1)];
            return addHeapObject(ret);
        },
        __wbg_has_d4e53238966c12b6: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.has(getObject(arg0), getObject(arg1));
            return ret;
        }, arguments); },
        __wbg_info_32d02c37919ba287: function(arg0, arg1, arg2, arg3) {
            getObject(arg0).info(getObject(arg1), arg2 === 0 ? undefined : getStringFromWasm0(arg2, arg3));
        },
        __wbg_instanceof_ArrayBuffer_c367199e2fa2aa04: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Map_53af74335dec57f4: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Map;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Object_1c6af87502b733ed: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Object;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Promise_0094681e3519d6ec: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Promise;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Uint8Array_9b9075935c74707c: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_internalbinarynode_new: function(arg0) {
            const ret = InternalBinaryNode.__wrap(arg0);
            return addHeapObject(ret);
        },
        __wbg_isArray_d314bb98fcf08331: function(arg0) {
            const ret = Array.isArray(getObject(arg0));
            return ret;
        },
        __wbg_isSafeInteger_bfbc7332a9768d2a: function(arg0) {
            const ret = Number.isSafeInteger(getObject(arg0));
            return ret;
        },
        __wbg_isTrustedIdentity_9227fe7bc856b2cd: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            const ret = getObject(arg0).isTrustedIdentity(getStringFromWasm0(arg1, arg2), getObject(arg3), arg4 >>> 0);
            return addHeapObject(ret);
        }, arguments); },
        __wbg_iterator_6ff6560ca1568e55: function() {
            const ret = Symbol.iterator;
            return addHeapObject(ret);
        },
        __wbg_keys_b50a709a76add04e: function(arg0) {
            const ret = Object.keys(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_length_32ed9a279acd054c: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_length_35a7bace40f36eac: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_level_d79ea42b38dc874b: function(arg0, arg1) {
            const ret = getObject(arg1).level;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_loadPreKey_a2b93ab92b5a1c6c: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).loadPreKey(arg1 >>> 0);
            return addHeapObject(ret);
        }, arguments); },
        __wbg_loadSenderKey_0ff2e83503d12b46: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).loadSenderKey(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_loadSession_72cdb264c350dddb: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).loadSession(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_loadSignedPreKey_01cbb620428303f5: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).loadSignedPreKey(arg1 >>> 0);
            return addHeapObject(ret);
        }, arguments); },
        __wbg_new_361308b2356cecd0: function() {
            const ret = new Object();
            return addHeapObject(ret);
        },
        __wbg_new_3eb36ae241fe6f44: function() {
            const ret = new Array();
            return addHeapObject(ret);
        },
        __wbg_new_77c95b2cd0e1885c: function(arg0, arg1) {
            const ret = new TypeError(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        },
        __wbg_new_b5d9e2fb389fef91: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return __wasm_bindgen_func_elem_1679(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return addHeapObject(ret);
            } finally {
                state0.a = state0.b = 0;
            }
        },
        __wbg_new_dd2b680c8bf6ae29: function(arg0) {
            const ret = new Uint8Array(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_new_from_slice_a3d2629dc1826784: function(arg0, arg1) {
            const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
            return addHeapObject(ret);
        },
        __wbg_new_no_args_1c7c842f08d00ebb: function(arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        },
        __wbg_new_with_length_1763c527b2923202: function(arg0) {
            const ret = new Array(arg0 >>> 0);
            return addHeapObject(ret);
        },
        __wbg_new_with_length_a2c39cbe88fd8ff1: function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return addHeapObject(ret);
        },
        __wbg_next_3482f54c49e8af19: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).next();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_next_418f80d8f5303233: function(arg0) {
            const ret = getObject(arg0).next;
            return addHeapObject(ret);
        },
        __wbg_noisesession_new: function(arg0) {
            const ret = NoiseSession.__wrap(arg0);
            return addHeapObject(ret);
        },
        __wbg_noisexxfallbacksession_new: function(arg0) {
            const ret = NoiseXxFallbackSession.__wrap(arg0);
            return addHeapObject(ret);
        },
        __wbg_parse_708461a1feddfb38: function() { return handleError(function (arg0, arg1) {
            const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_prototypesetcall_bdcdcc5842e4d77d: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), getObject(arg2));
        },
        __wbg_push_8ffdcb2063340ba5: function(arg0, arg1) {
            const ret = getObject(arg0).push(getObject(arg1));
            return ret;
        },
        __wbg_queueMicrotask_0aa0a927f78f5d98: function(arg0) {
            const ret = getObject(arg0).queueMicrotask;
            return addHeapObject(ret);
        },
        __wbg_queueMicrotask_5bb536982f78a56f: function(arg0) {
            queueMicrotask(getObject(arg0));
        },
        __wbg_removePreKey_98811fdd48c073fb: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).removePreKey(arg1 >>> 0);
            return addHeapObject(ret);
        }, arguments); },
        __wbg_resolve_002c4b7d9d8f6b64: function(arg0) {
            const ret = Promise.resolve(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_senderkeydistributionmessage_new: function(arg0) {
            const ret = SenderKeyDistributionMessage.__wrap(arg0);
            return addHeapObject(ret);
        },
        __wbg_sessionrecord_new: function(arg0) {
            const ret = SessionRecord.__wrap(arg0);
            return addHeapObject(ret);
        },
        __wbg_set_3f1d0b984ed272ed: function(arg0, arg1, arg2) {
            getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
        },
        __wbg_set_6cb8631f80447a67: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
            return ret;
        }, arguments); },
        __wbg_set_cc56eefd2dd91957: function(arg0, arg1, arg2) {
            getObject(arg0).set(getArrayU8FromWasm0(arg1, arg2));
        },
        __wbg_set_f43e577aea94465b: function(arg0, arg1, arg2) {
            getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
        },
        __wbg_static_accessor_GLOBAL_12837167ad935116: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_e628e89ab3b1c95f: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_SELF_a621d3dfbb60d0ce: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_WINDOW_f8727f0cf888e0bd: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_storeSenderKey_f75b687ecb81167b: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = getObject(arg0).storeSenderKey(getStringFromWasm0(arg1, arg2), getObject(arg3));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_storeSessionRaw_62bcfde67041fd72: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = getObject(arg0).storeSessionRaw(getStringFromWasm0(arg1, arg2), getObject(arg3));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_storeSession_85743e89b8885477: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = getObject(arg0).storeSession(getStringFromWasm0(arg1, arg2), takeObject(arg3));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_tag_f94df748099af08d: function(arg0, arg1) {
            const ret = getObject(arg1).tag;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_then_0d9fe2c7b1857d32: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        },
        __wbg_then_b9e7b3b5f1a9e1b5: function(arg0, arg1) {
            const ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        },
        __wbg_trace_b203136e640da099: function(arg0, arg1, arg2, arg3) {
            getObject(arg0).trace(getObject(arg1), arg2 === 0 ? undefined : getStringFromWasm0(arg2, arg3));
        },
        __wbg_value_0546255b415e96c1: function(arg0) {
            const ret = getObject(arg0).value;
            return addHeapObject(ret);
        },
        __wbg_warn_24e97d9de99c2b0e: function(arg0, arg1, arg2, arg3) {
            getObject(arg0).warn(getObject(arg1), arg2 === 0 ? undefined : getStringFromWasm0(arg2, arg3));
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 198, function: Function { arguments: [Externref], shim_idx: 199, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_892, __wasm_bindgen_func_elem_894);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000002: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000003: function(arg0) {
            // Cast intrinsic for `I64 -> Externref`.
            const ret = arg0;
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
            const ret = getArrayU8FromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000005: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000006: function(arg0) {
            // Cast intrinsic for `U64 -> Externref`.
            const ret = BigInt.asUintN(64, arg0);
            return addHeapObject(ret);
        },
        __wbindgen_object_clone_ref: function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        },
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
    };
    return {
        __proto__: null,
        "./whatsapp_rust_bridge_bg.js": import0,
    };
}

function __wasm_bindgen_func_elem_894(arg0, arg1, arg2) {
    wasm.__wasm_bindgen_func_elem_894(arg0, arg1, addHeapObject(arg2));
}

function __wasm_bindgen_func_elem_1679(arg0, arg1, arg2, arg3) {
    wasm.__wasm_bindgen_func_elem_1679(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

const ExpandedAppStateKeysFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_expandedappstatekeys_free(ptr >>> 0, 1));
const GroupCipherFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_groupcipher_free(ptr >>> 0, 1));
const GroupSessionBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_groupsessionbuilder_free(ptr >>> 0, 1));
const InternalBinaryNodeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_internalbinarynode_free(ptr >>> 0, 1));
const LTHashAntiTamperingFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_lthashantitampering_free(ptr >>> 0, 1));
const LTHashStateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_lthashstate_free(ptr >>> 0, 1));
const NoiseIkSessionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_noiseiksession_free(ptr >>> 0, 1));
const NoiseSessionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_noisesession_free(ptr >>> 0, 1));
const NoiseXxFallbackSessionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_noisexxfallbacksession_free(ptr >>> 0, 1));
const ProtocolAddressFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_protocoladdress_free(ptr >>> 0, 1));
const SenderKeyDistributionMessageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_senderkeydistributionmessage_free(ptr >>> 0, 1));
const SenderKeyNameFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_senderkeyname_free(ptr >>> 0, 1));
const SenderKeyRecordFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_senderkeyrecord_free(ptr >>> 0, 1));
const SessionBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sessionbuilder_free(ptr >>> 0, 1));
const SessionCipherFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sessioncipher_free(ptr >>> 0, 1));
const SessionRecordFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sessionrecord_free(ptr >>> 0, 1));

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export3(addHeapObject(e));
    }
}

let heap = new Array(128).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('whatsapp_rust_bridge_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
