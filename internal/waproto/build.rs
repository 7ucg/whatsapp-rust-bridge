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

        Ok(())
    }
}
