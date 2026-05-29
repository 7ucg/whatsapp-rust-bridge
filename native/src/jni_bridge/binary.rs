use jni::JNIEnv;
use jni::objects::JByteArray;
use jni::sys::jbyteArray;
use serde::{Deserialize, Serialize};
use wacore_binary::marshal::marshal_ref;
use wacore_binary::node::{AttrsRef, NodeContentRef, NodeRef, NodeStr, ValueRef};
use wacore_binary::util::unpack;

use super::crypto::{bytes_from_jarray, bytes_to_jarray, throw};

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

fn json_to_node_ref(n: &JsonNode) -> NodeRef<'static> {
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
        JsonContent::Nodes(ch) => {
            let nodes: Vec<NodeRef<'static>> = ch.iter().map(json_to_node_ref).collect();
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
        NodeContentRef::Nodes(ch) => {
            JsonContent::Nodes(ch.iter().map(node_ref_to_json).collect())
        }
    });
    JsonNode { tag: node.tag.to_string(), attrs, content }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Binary_encodeNode(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    json_bytes: JByteArray<'_>,
) -> jbyteArray {
    let input = bytes_from_jarray(&mut env, json_bytes);
    let json_node: JsonNode = match serde_json::from_slice(&input) {
        Ok(n) => n,
        Err(e) => return throw(&mut env, &format!("JSON parse error: {e}")),
    };
    let node_ref = json_to_node_ref(&json_node);
    match marshal_ref(&node_ref) {
        Ok(encoded) => bytes_to_jarray(&mut env, &encoded),
        Err(e) => throw(&mut env, &format!("Encode error: {e}")),
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_whatsapp_bridge_Binary_decodeNode(
    mut env: JNIEnv<'_>,
    _class: jni::objects::JClass<'_>,
    data: JByteArray<'_>,
) -> jbyteArray {
    let input = bytes_from_jarray(&mut env, data);
    let unpacked = match unpack(&input) {
        Ok(b) => b.into_owned(),
        Err(e) => return throw(&mut env, &format!("Unpack error: {e}")),
    };
    let node_ref = match wacore_binary::marshal::unmarshal_ref(&unpacked) {
        Ok(n) => n,
        Err(e) => return throw(&mut env, &format!("Decode error: {e}")),
    };
    let json_node = node_ref_to_json(&node_ref);
    match serde_json::to_vec(&json_node) {
        Ok(json) => bytes_to_jarray(&mut env, &json),
        Err(e) => throw(&mut env, &format!("JSON serialize error: {e}")),
    }
}
