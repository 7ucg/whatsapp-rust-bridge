// This module contains the auto-generated protobuf definitions.
// The code is generated from `whatsapp.proto` and checked into version control.
// To regenerate, run `cargo build -p waproto --features generate`.
// See `build.rs` for the full proto compilation config.

#![allow(clippy::large_enum_variant)]
pub mod whatsapp {
    #[rustfmt::skip]
    include!("whatsapp.rs");
}

/// Wire tags of every message field in `whatsapp.proto`, generated alongside the
/// prost code (see `build.rs`). Hand-written partial decoders reference these
/// consts instead of magic numbers, so schema changes surface as compile errors
/// rather than silent wire-format drift.
pub mod tags {
    #[rustfmt::skip]
    include!("tags.rs");
}

/// Pinned, non-generic codec entry points for the hottest protobuf roots.
///
/// prost's `Message` methods are generic, so rustc instantiates them in every
/// crate that calls them; routing calls through these `#[inline(never)]`
/// functions pins a single instantiation in this crate instead of shipping a
/// full encode/decode tree per calling crate.
pub mod codec {
    use crate::whatsapp;
    use prost::Message as _;

    #[inline(never)]
    pub fn message_encoded_len(msg: &whatsapp::Message) -> usize {
        msg.encoded_len()
    }

    /// Append the encoded message to `out`. Infallible into a `Vec`.
    #[inline(never)]
    pub fn message_encode_into(msg: &whatsapp::Message, out: &mut Vec<u8>) {
        msg.encode(out).expect("encode into Vec is infallible");
    }

    #[inline(never)]
    pub fn message_to_vec(msg: &whatsapp::Message) -> Vec<u8> {
        msg.encode_to_vec()
    }

    #[inline(never)]
    pub fn message_decode(mut bytes: &[u8]) -> Result<whatsapp::Message, prost::DecodeError> {
        whatsapp::Message::decode(&mut bytes)
    }

    #[inline(never)]
    pub fn web_message_info_decode(
        mut bytes: &[u8],
    ) -> Result<whatsapp::WebMessageInfo, prost::DecodeError> {
        whatsapp::WebMessageInfo::decode(&mut bytes)
    }

    #[inline(never)]
    pub fn history_sync_decode(
        mut bytes: &[u8],
    ) -> Result<whatsapp::HistorySync, prost::DecodeError> {
        whatsapp::HistorySync::decode(&mut bytes)
    }

    #[inline(never)]
    pub fn message_context_info_encoded_len(mci: &whatsapp::MessageContextInfo) -> usize {
        mci.encoded_len()
    }

    /// Append the encoded `MessageContextInfo` to `out`. Infallible into a `Vec`.
    #[inline(never)]
    pub fn message_context_info_encode_into(mci: &whatsapp::MessageContextInfo, out: &mut Vec<u8>) {
        mci.encode(out).expect("encode into Vec is infallible");
    }

    #[inline(never)]
    pub fn message_context_info_to_vec(mci: &whatsapp::MessageContextInfo) -> Vec<u8> {
        mci.encode_to_vec()
    }

    /// Merge wire bytes into an existing `MessageContextInfo` (prost merge
    /// semantics: later-set fields win).
    #[inline(never)]
    pub fn message_context_info_merge(
        mci: &mut whatsapp::MessageContextInfo,
        bytes: &[u8],
    ) -> Result<(), prost::DecodeError> {
        let mut cursor = bytes;
        mci.merge(&mut &mut cursor)
    }
}
