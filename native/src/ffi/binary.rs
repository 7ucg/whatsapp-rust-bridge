use std::slice;
use wacore_binary::marshal::marshal_ref;
use wacore_binary::node::{AttrsRef, NodeContentRef, NodeRef, NodeStr, ValueRef};
use wacore_binary::util::unpack;

use super::crypto::BridgeError;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum JsonContent {
    Text(String),
    Bytes { b64: String },
    Nodes(Vec<JsonNode>),
}

#[derive(Serialize, Deserialize)]
struct JsonNode {
    tag: String,
    #[serde(default)]
    attrs: std::collections::HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<JsonContent>,
}

fn json_node_to_node_ref(n: &JsonNode) -> NodeRef<'static> {
    let attrs: Vec<(NodeStr<'static>, ValueRef<'static>)> = n
        .attrs
        .iter()
        .map(|(k, v)| {
            (
                NodeStr::from(compact_str::CompactString::from(k.as_str())),
                ValueRef::String(NodeStr::from(compact_str::CompactString::from(v.as_str()))),
            )
        })
        .collect();

    let content = n.content.as_ref().map(|c| match c {
        JsonContent::Text(s) => {
            NodeContentRef::String(NodeStr::from(compact_str::CompactString::from(s.as_str())))
        }
        JsonContent::Bytes { b64 } => {
            use base64::Engine;
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(b64)
                .unwrap_or_default();
            NodeContentRef::Bytes(std::borrow::Cow::Owned(bytes))
        }
        JsonContent::Nodes(children) => {
            let nodes: Vec<NodeRef<'static>> = children.iter().map(json_node_to_node_ref).collect();
            NodeContentRef::Nodes(Box::from(nodes.into_boxed_slice()))
        }
    });

    NodeRef::new(
        NodeStr::from(compact_str::CompactString::from(n.tag.as_str())),
        AttrsRef::from_vec(attrs),
        content,
    )
}

fn node_ref_to_json(node: &NodeRef<'_>) -> JsonNode {
    let attrs = node
        .attrs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let content = node.content.as_deref().map(|c| match c {
        NodeContentRef::String(s) => JsonContent::Text(s.to_string()),
        NodeContentRef::Bytes(b) => {
            use base64::Engine;
            JsonContent::Bytes {
                b64: base64::engine::general_purpose::STANDARD.encode(b.as_ref()),
            }
        }
        NodeContentRef::Nodes(children) => {
            JsonContent::Nodes(children.iter().map(node_ref_to_json).collect())
        }
    });

    JsonNode {
        tag: node.tag.to_string(),
        attrs,
        content,
    }
}

// ── wa_encode_node ────────────────────────────────────────────────────────────

/// Encode a JSON-represented BinaryNode into WhatsApp wire format.
///
/// JSON schema:
///   { "tag": "...", "attrs": { "k": "v" },
///     "content": "text" | [nodes] | { "b64": "<base64 bytes>" } }
///
/// *out_len must be >= needed size. Set to actual size on success.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_encode_node(
    json_in: *const u8,
    json_len: usize,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    if json_in.is_null() || out_buf.is_null() || out_len.is_null() {
        return BridgeError::NullPointer as i32;
    }
    let json_bytes = unsafe { slice::from_raw_parts(json_in, json_len) };
    let json_node: JsonNode = match serde_json::from_slice(json_bytes) {
        Ok(n) => n,
        Err(_) => return BridgeError::DecryptionFailed as i32,
    };
    let node_ref = json_node_to_node_ref(&json_node);
    let encoded = match marshal_ref(&node_ref) {
        Ok(b) => b,
        Err(_) => return BridgeError::EncryptionFailed as i32,
    };
    unsafe { super::crypto::write_out_pub(&encoded, out_buf, out_len) }
}

/// Decode WhatsApp binary-encoded node into JSON bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wa_decode_node(
    data: *const u8,
    data_len: usize,
    out_buf: *mut u8,
    out_len: *mut usize,
) -> i32 {
    if data.is_null() || out_buf.is_null() || out_len.is_null() {
        return BridgeError::NullPointer as i32;
    }
    let bytes = unsafe { slice::from_raw_parts(data, data_len) };
    let unpacked = match unpack(bytes) {
        Ok(b) => b.into_owned(),
        Err(_) => return BridgeError::DecryptionFailed as i32,
    };
    let node_ref = match wacore_binary::marshal::unmarshal_ref(&unpacked) {
        Ok(n) => n,
        Err(_) => return BridgeError::DecryptionFailed as i32,
    };
    let json_node = node_ref_to_json(&node_ref);
    let json_bytes = match serde_json::to_vec(&json_node) {
        Ok(b) => b,
        Err(_) => return BridgeError::EncryptionFailed as i32,
    };
    unsafe { super::crypto::write_out_pub(&json_bytes, out_buf, out_len) }
}
