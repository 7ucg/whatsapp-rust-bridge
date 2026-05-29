package com.whatsapp.bridge;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.file.Files;

/**
 * Utility to load the whatsapp_bridge native library.
 *
 * Usage (once, at app startup):
 *   NativeLoader.load();
 *
 * The library file must be on java.library.path, OR bundled as a resource at:
 *   /native/<os>-<arch>/libwhatsapp_bridge.so  (Linux)
 *   /native/<os>-<arch>/whatsapp_bridge.dll    (Windows)
 *   /native/<os>-<arch>/libwhatsapp_bridge.dylib (macOS)
 */
public final class NativeLoader {

    private static volatile boolean loaded = false;

    private NativeLoader() {}

    public static synchronized void load() {
        if (loaded) return;
        try {
            System.loadLibrary("whatsapp_bridge");
        } catch (UnsatisfiedLinkError e) {
            loadFromResources();
        }
        loaded = true;
    }

    private static void loadFromResources() {
        String os = System.getProperty("os.name", "").toLowerCase();
        String arch = System.getProperty("os.arch", "").toLowerCase();
        String libName;
        String resourceDir;

        if (os.contains("win")) {
            libName = "whatsapp_bridge.dll";
            resourceDir = "windows-" + normalizeArch(arch);
        } else if (os.contains("mac")) {
            libName = "libwhatsapp_bridge.dylib";
            resourceDir = "macos-" + normalizeArch(arch);
        } else {
            libName = "libwhatsapp_bridge.so";
            resourceDir = "linux-" + normalizeArch(arch);
        }

        String resourcePath = "/native/" + resourceDir + "/" + libName;
        try (InputStream in = NativeLoader.class.getResourceAsStream(resourcePath)) {
            if (in == null) {
                throw new UnsatisfiedLinkError(
                    "Native library not found in resources: " + resourcePath +
                    ". Add it to your JAR or set java.library.path.");
            }
            File tmp = File.createTempFile("whatsapp_bridge_", libName);
            tmp.deleteOnExit();
            try (OutputStream out = Files.newOutputStream(tmp.toPath())) {
                in.transferTo(out);
            }
            System.load(tmp.getAbsolutePath());
        } catch (IOException ex) {
            throw new UnsatisfiedLinkError("Failed to extract native library: " + ex.getMessage());
        }
    }

    private static String normalizeArch(String arch) {
        if (arch.contains("aarch64") || arch.contains("arm64")) return "arm64";
        if (arch.contains("x86_64") || arch.contains("amd64"))  return "x86_64";
        return arch;
    }
}
