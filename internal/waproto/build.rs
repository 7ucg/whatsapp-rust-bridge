// # Updating the Proto File
//
// When modifying `src/whatsapp.proto`, follow these steps:
//
// 1. Format the proto file (requires `buf` CLI: https://buf.build/docs/installation):
//    ```
//    buf format waproto/src/whatsapp.proto -w
//    ```
//
// 2. Regenerate the Rust code:
//    ```
//    cargo build -p waproto --features generate
//    ```
//
// 3. Fix any breaking changes in the codebase (e.g., `optional` -> `required` field changes)

fn main() -> std::io::Result<()> {
    #[cfg(not(feature = "generate"))]
    {
        println!("cargo:rerun-if-changed=build.rs");
        Ok(())
    }

    #[cfg(feature = "generate")]
    {
        use prost::Message as _;

        println!("cargo:rerun-if-changed=src/whatsapp.proto");
        println!("cargo:warning=Regenerating proto definitions...");

        // Use vendored protoc 3.x which accepts WhatsApp's non-standard enums
        let protoc = protoc_bin_vendored::protoc_bin_path().expect("vendored protoc not found");
        // SAFETY: build scripts are single-threaded
        unsafe { std::env::set_var("PROTOC", protoc); }

        let mut config = prost_build::Config::new();

        // Serialize always; Deserialize only for WASM bridge (halves serde codegen).
        config.type_attribute(".", "#[derive(serde::Serialize)]");
        config.type_attribute(
            ".",
            "#[cfg_attr(feature = \"serde-deserialize\", derive(serde::Deserialize))]",
        );
        // Default missing fields to match protobuf semantics (structs only).
        config.message_attribute(
            ".",
            "#[cfg_attr(feature = \"serde-deserialize\", serde(default))]",
        );

        // Accept snake_case on deserialization for WASM bridge enum variants.
        config.type_attribute(
            ".",
            "#[cfg_attr(feature = \"serde-snake-case\", serde(rename_all(deserialize = \"snake_case\")))]",
        );

        // O(1)-clone Bytes for hot-path crypto structures instead of Vec<u8>.
        config.bytes([
            ".proto.SessionStructure.Chain.ChainKey",
            ".proto.SessionStructure.Chain.MessageKey",
            ".proto.SenderKeyStateStructure.SenderChainKey",
            ".proto.SenderKeyStateStructure.SenderMessageKey",
            ".proto.SenderKeyStateStructure.SenderSigningKey",
        ]);

        // Boxed: large (and mostly absent-on-the-wire) submessages whose inline
        // form makes prost's repeated-field decode memcpy-bound — every element
        // pays push(default) plus Vec-growth copies of the full struct size.
        config.boxed(".proto.HistorySyncMsg.message");
        config.boxed(".proto.WebMessageInfo.message");
        config.boxed(".proto.WebMessageInfo.statusMentionMessageInfo");
        config.boxed(".proto.Message.messageContextInfo");

        // Box the remaining inline message-typed fields so `Message` — a union
        // of ~110 content variants of which exactly one is ever set — stops
        // paying for all of them inline. prost already boxes the variants in
        // recursion cycles; these are the rest. Shrinking the struct makes
        // every clone, decode, and `Arc<Message>` cheaper to move and hold.
        for field in [
            "bcallMessage",
            "callLogMesssage",
            "cancelPaymentRequestMessage",
            "chat",
            "conditionalRevealMessage",
            "declinePaymentRequestMessage",
            "encCommentMessage",
            "encEventResponseMessage",
            "encReactionMessage",
            "groupRootKeyShare",
            "invoiceMessage",
            "keepInChatMessage",
            "paymentInviteMessage",
            "paymentReminderMessage",
            "pinInChatMessage",
            "placeholderMessage",
            "pollAddOptionMessage",
            "pollUpdateMessage",
            "questionResponseMessage",
            "reactionMessage",
            "rootSecretDistributeMessage",
            "scheduledCallCreationMessage",
            "scheduledCallEditMessage",
            "secretEncryptedMessage",
            "statusNotificationMessage",
            "statusQuestionAnswerMessage",
            "statusQuotedMessage",
            "statusStickerInteractionMessage",
            "stickerSyncRmrMessage",
        ] {
            config.boxed(format!(".proto.Message.{field}").as_str());
        }

        // Bytes fields lack serde support; skip them (internal crypto state).
        config.field_attribute(
            ".proto.SessionStructure.Chain.ChainKey.key",
            "#[serde(skip)]",
        );
        config.field_attribute(
            ".proto.SessionStructure.Chain.MessageKey.cipherKey",
            "#[serde(skip)]",
        );
        config.field_attribute(
            ".proto.SessionStructure.Chain.MessageKey.macKey",
            "#[serde(skip)]",
        );
        config.field_attribute(
            ".proto.SessionStructure.Chain.MessageKey.iv",
            "#[serde(skip)]",
        );
        config.field_attribute(
            ".proto.SenderKeyStateStructure.SenderChainKey.seed",
            "#[serde(skip)]",
        );
        config.field_attribute(
            ".proto.SenderKeyStateStructure.SenderMessageKey.seed",
            "#[serde(skip)]",
        );
        config.field_attribute(
            ".proto.SenderKeyStateStructure.SenderSigningKey.public",
            "#[serde(skip)]",
        );
        config.field_attribute(
            ".proto.SenderKeyStateStructure.SenderSigningKey.private",
            "#[serde(skip)]",
        );

        // Emit the FileDescriptorSet so we can generate `tags.rs` from it below.
        let out_dir = std::path::PathBuf::from(
            std::env::var_os("OUT_DIR").expect("OUT_DIR set for build scripts"),
        );
        let fds_path = out_dir.join("whatsapp.desc");
        config.file_descriptor_set_path(&fds_path);

        // Output to src/ so generated code is version-controlled.
        config.out_dir("src/");

        config.compile_protos(&["src/whatsapp.proto"], &["src/"])?;

        // prost-build names the output file after the proto package ("proto.rs").
        // Rename to "whatsapp.rs" so existing include!("whatsapp.rs") keeps working.
        let out = std::path::Path::new("src");
        let generated = out.join("proto.rs");
        let target = out.join("whatsapp.rs");
        if generated.exists() {
            std::fs::rename(&generated, &target)?;
        }

        // Generate `tags.rs` (committed) from the descriptor: one module per
        // message with a `u32` const per field carrying its wire tag, so hand-
        // written partial decoders in `wacore` reference these instead of magic
        // numbers and a schema change fails compilation instead of silently
        // desyncing.
        let fds = prost_types::FileDescriptorSet::decode(std::fs::read(&fds_path)?.as_slice())
            .map_err(std::io::Error::other)?;
        generate_tags(&fds, &out.join("tags.rs"))?;

        Ok(())
    }
}

/// Generate `tags.rs`: one module per message carrying a `u32` const per field
/// with its wire tag, straight from the descriptor.
#[cfg(feature = "generate")]
fn generate_tags(
    fds: &prost_types::FileDescriptorSet,
    out_path: &std::path::Path,
) -> std::io::Result<()> {
    use heck::{ToShoutySnakeCase, ToSnakeCase};
    use prost_types::DescriptorProto;

    /// prost-parity identifier sanitization so module names always match what
    /// prost would generate for the same message.
    fn module_ident(name: &str) -> String {
        let snake = name.to_snake_case();
        match snake.as_str() {
            "as" | "break" | "const" | "continue" | "else" | "enum" | "false" | "fn" | "for"
            | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub"
            | "ref" | "return" | "static" | "struct" | "trait" | "true" | "type" | "unsafe"
            | "use" | "where" | "while" | "dyn" | "abstract" | "become" | "box" | "do"
            | "final" | "macro" | "override" | "priv" | "typeof" | "unsized" | "virtual"
            | "yield" | "async" | "await" | "try" | "gen" => format!("r#{snake}"),
            "_" | "super" | "self" | "crate" | "extern" => format!("{snake}_"),
            other if other.starts_with(|c: char| c.is_numeric()) => format!("_{snake}"),
            _ => snake,
        }
    }

    fn emit_message(out: &mut String, msg: &DescriptorProto, indent: usize) {
        // Synthetic map-entry messages have no hand-decodable surface.
        if msg
            .options
            .as_ref()
            .and_then(|o| o.map_entry)
            .unwrap_or(false)
        {
            return;
        }
        let pad = "    ".repeat(indent);
        out.push_str(&format!("{pad}pub mod {} {{\n", module_ident(msg.name())));
        let mut seen = std::collections::HashSet::new();
        for field in &msg.field {
            let const_name = field.name().to_shouty_snake_case();
            assert!(
                seen.insert(const_name.clone()),
                "tags.rs: const name collision `{const_name}` in message `{}`",
                msg.name()
            );
            out.push_str(&format!(
                "{pad}    pub const {const_name}: u32 = {};\n",
                field.number()
            ));
        }
        for nested in &msg.nested_type {
            emit_message(out, nested, indent + 1);
        }
        out.push_str(&format!("{pad}}}\n"));
    }

    let mut out = String::with_capacity(1 << 20);
    out.push_str(
        "// @generated from whatsapp.proto by waproto's build.rs. Do not edit.\n\
         //\n\
         // Wire tag of every message field, for hand-written partial decoders.\n",
    );
    for file in &fds.file {
        for msg in &file.message_type {
            emit_message(&mut out, msg, 0);
        }
    }
    std::fs::write(out_path, out)
}
