import {readFileSync} from "fs";
import {join} from "path";

export const base64Wasm = () => {
  const bytes = readFileSync(join(__dirname, "../pkg/whatsapp_rust_bridge_bg.wasm"))

  const base64 = bytes.toString('base64');

  return base64;
};
