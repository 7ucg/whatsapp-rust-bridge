use std::str::FromStr;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wacore_binary::jid::{Jid as CoreJid, JidExt, Server};
use wasm_bindgen::prelude::*;

// ── JidInfo struct ────────────────────────────────────────────────────────────

/// A WhatsApp JID (Jabber ID) — identifies a user, group, broadcast, etc.
#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct JidInfo {
    pub user: String,
    pub server: String,
    pub agent: u8,
    pub device: u16,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn server_from_str(s: &str) -> Server {
    Server::try_from(s).unwrap_or(Server::Pn)
}

fn core_to_info(j: CoreJid) -> JidInfo {
    JidInfo {
        user: j.user.to_string(),
        server: j.server.as_str().to_string(),
        agent: j.agent,
        device: j.device,
    }
}

fn parse(s: &str) -> Option<CoreJid> {
    CoreJid::from_str(s).ok()
}

// ── Parse / encode ────────────────────────────────────────────────────────────

/// Parse a JID string into its components.
/// Accepts: "user@server", "user@server:device", "user.agent:device@server"
#[wasm_bindgen(js_name = parseJid)]
pub fn parse_jid(jid_str: &str) -> Option<JidInfo> {
    parse(jid_str).map(core_to_info)
}

/// Encode a JidInfo back to its canonical string.
#[wasm_bindgen(js_name = encodeJid)]
pub fn encode_jid(info: JidInfo) -> String {
    let server = server_from_str(&info.server);
    let mut buf = String::new();
    wacore_binary::jid::push_jid_to_string(&info.user, server, info.agent, info.device, &mut buf);
    buf
}

// ── Constructors ──────────────────────────────────────────────────────────────

/// Create a user JID: "phone@s.whatsapp.net"
#[wasm_bindgen(js_name = jidUser)]
pub fn jid_user(phone: &str) -> String {
    let j = CoreJid::pn(phone);
    let mut buf = String::new();
    wacore_binary::jid::push_jid_to_string(&j.user, j.server, j.agent, j.device, &mut buf);
    buf
}

/// Create a user JID with a specific device ID.
#[wasm_bindgen(js_name = jidUserDevice)]
pub fn jid_user_device(phone: &str, device: u16) -> String {
    let j = CoreJid::pn_device(phone, device);
    let mut buf = String::new();
    wacore_binary::jid::push_jid_to_string(&j.user, j.server, j.agent, j.device, &mut buf);
    buf
}

/// Create a group JID: "groupId@g.us"
#[wasm_bindgen(js_name = jidGroup)]
pub fn jid_group(group_id: &str) -> String {
    let j = CoreJid::group(group_id);
    let mut buf = String::new();
    wacore_binary::jid::push_jid_to_string(&j.user, j.server, j.agent, j.device, &mut buf);
    buf
}

/// Create a newsletter (channel) JID: "id@newsletter"
#[wasm_bindgen(js_name = jidNewsletter)]
pub fn jid_newsletter(id: &str) -> String {
    let j = CoreJid::newsletter(id);
    let mut buf = String::new();
    wacore_binary::jid::push_jid_to_string(&j.user, j.server, j.agent, j.device, &mut buf);
    buf
}

/// Create a LID JID: "lid@lid"
#[wasm_bindgen(js_name = jidLid)]
pub fn jid_lid(lid: &str) -> String {
    let j = CoreJid::lid(lid);
    let mut buf = String::new();
    wacore_binary::jid::push_jid_to_string(&j.user, j.server, j.agent, j.device, &mut buf);
    buf
}

/// Returns the status broadcast JID: "status@broadcast"
#[wasm_bindgen(js_name = jidStatusBroadcast)]
pub fn jid_status_broadcast() -> String {
    "status@broadcast".into()
}

// ── Type checks ───────────────────────────────────────────────────────────────

/// Returns true if the JID is a regular user (s.whatsapp.net).
#[wasm_bindgen(js_name = isUserJid)]
pub fn is_user_jid(jid_str: &str) -> bool {
    parse(jid_str).map(|j| j.server == Server::Pn).unwrap_or(false)
}

/// Returns true if the JID is a LID-based user (lid server).
#[wasm_bindgen(js_name = isLidJid)]
pub fn is_lid_jid(jid_str: &str) -> bool {
    parse(jid_str).map(|j| j.server == Server::Lid).unwrap_or(false)
}

/// Returns true if the JID is a group (g.us).
#[wasm_bindgen(js_name = isGroupJid)]
pub fn is_group_jid(jid_str: &str) -> bool {
    parse(jid_str).map(|j| j.is_group()).unwrap_or(false)
}

/// Returns true if the JID is a broadcast list (not status).
#[wasm_bindgen(js_name = isBroadcastListJid)]
pub fn is_broadcast_list_jid(jid_str: &str) -> bool {
    parse(jid_str).map(|j| j.is_broadcast_list()).unwrap_or(false)
}

/// Returns true if the JID is the status broadcast ("status@broadcast").
#[wasm_bindgen(js_name = isStatusBroadcastJid)]
pub fn is_status_broadcast_jid(jid_str: &str) -> bool {
    parse(jid_str).map(|j| j.is_status_broadcast()).unwrap_or(false)
}

/// Returns true if the JID is a newsletter (channel).
#[wasm_bindgen(js_name = isNewsletterJid)]
pub fn is_newsletter_jid(jid_str: &str) -> bool {
    parse(jid_str).map(|j| j.is_newsletter()).unwrap_or(false)
}

/// Returns true if the JID belongs to a multi-device (AD) session (device > 0).
#[wasm_bindgen(js_name = isADJid)]
pub fn is_ad_jid(jid_str: &str) -> bool {
    parse(jid_str).map(|j| j.is_ad()).unwrap_or(false)
}

/// Returns true if the JID is a WhatsApp bot.
#[wasm_bindgen(js_name = isBotJid)]
pub fn is_bot_jid(jid_str: &str) -> bool {
    parse(jid_str).map(|j| j.is_bot()).unwrap_or(false)
}

/// Returns true if the JID is a Meta Messenger bridged contact.
#[wasm_bindgen(js_name = isMessengerJid)]
pub fn is_messenger_jid(jid_str: &str) -> bool {
    parse(jid_str).map(|j| j.is_messenger()).unwrap_or(false)
}

/// Returns true if the JID is a hosted/Cloud API device.
#[wasm_bindgen(js_name = isHostedJid)]
pub fn is_hosted_jid(jid_str: &str) -> bool {
    parse(jid_str).map(|j| j.is_hosted()).unwrap_or(false)
}

/// Returns true if both JIDs refer to the same user (ignoring device).
#[wasm_bindgen(js_name = areSameUser)]
pub fn are_same_user(a: &str, b: &str) -> bool {
    match (parse(a), parse(b)) {
        (Some(ja), Some(jb)) => ja.is_same_user_as(&jb),
        _ => false,
    }
}

// ── Accessors ─────────────────────────────────────────────────────────────────

/// Extract the user part (phone / group-id) from a JID string.
#[wasm_bindgen(js_name = jidGetUser)]
pub fn jid_get_user(jid_str: &str) -> Option<String> {
    parse(jid_str).map(|j| j.user.to_string())
}

/// Extract the server domain from a JID string.
#[wasm_bindgen(js_name = jidGetServer)]
pub fn jid_get_server(jid_str: &str) -> Option<String> {
    parse(jid_str).map(|j| j.server.as_str().to_string())
}

/// Extract the device ID from a JID string (0 = primary device).
#[wasm_bindgen(js_name = jidGetDevice)]
pub fn jid_get_device(jid_str: &str) -> Option<u16> {
    parse(jid_str).map(|j| j.device)
}

/// Normalize a JID to its primary user form (device = 0, agent = 0).
/// "123@s.whatsapp.net:5" → "123@s.whatsapp.net"
#[wasm_bindgen(js_name = jidNormalizedUser)]
pub fn jid_normalized_user(jid_str: &str) -> Option<String> {
    let j = parse(jid_str)?;
    let mut buf = String::new();
    wacore_binary::jid::push_jid_to_string(&j.user, j.server, 0, 0, &mut buf);
    Some(buf)
}

/// Change the device ID on an existing JID string.
#[wasm_bindgen(js_name = jidWithDevice)]
pub fn jid_with_device(jid_str: &str, device: u16) -> Option<String> {
    let j = parse(jid_str)?;
    let j2 = j.with_device(device);
    let mut buf = String::new();
    wacore_binary::jid::push_jid_to_string(&j2.user, j2.server, j2.agent, j2.device, &mut buf);
    Some(buf)
}

/// Returns the base user part stripping any ":device" suffix.
/// "123:4@s.whatsapp.net" → "123"
#[wasm_bindgen(js_name = jidUserBase)]
pub fn jid_user_base(jid_str: &str) -> Option<String> {
    parse(jid_str).map(|j| j.user_base().to_string())
}

// ── Server constants (functions, not statics — wasm_bindgen limitation) ──────

#[wasm_bindgen(js_name = jidServerUser)]      pub fn jid_server_user()      -> String { "s.whatsapp.net".into() }
#[wasm_bindgen(js_name = jidServerGroup)]     pub fn jid_server_group()     -> String { "g.us".into() }
#[wasm_bindgen(js_name = jidServerBroadcast)] pub fn jid_server_broadcast() -> String { "broadcast".into() }
#[wasm_bindgen(js_name = jidServerLid)]       pub fn jid_server_lid()       -> String { "lid".into() }
#[wasm_bindgen(js_name = jidServerNewsletter)]pub fn jid_server_newsletter()-> String { "newsletter".into() }
#[wasm_bindgen(js_name = jidServerMessenger)] pub fn jid_server_messenger() -> String { "msgr".into() }
#[wasm_bindgen(js_name = jidServerBot)]       pub fn jid_server_bot()       -> String { "bot".into() }
#[wasm_bindgen(js_name = jidServerHosted)]    pub fn jid_server_hosted()    -> String { "hosted".into() }
