"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.base64Wasm = void 0;
const fs_1 = require("fs");
const path_1 = require("path");
const base64Wasm = () => {
    const bytes = (0, fs_1.readFileSync)((0, path_1.join)(__dirname, "../pkg/whatsapp_rust_bridge_bg.wasm"));
    const base64 = bytes.toString('base64');
    return base64;
};
exports.base64Wasm = base64Wasm;
