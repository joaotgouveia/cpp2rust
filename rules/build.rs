// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};

fn main() {
    let crate_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let modules_rs = PathBuf::from(env::var("OUT_DIR").unwrap()).join("modules.rs");

    // Collect all tgt_*.rs files
    let mut files = Vec::new();
    visit(&crate_root, &mut |p| {
        if let Some(name) = p.file_name().and_then(|s| s.to_str())
            && name.starts_with("tgt_")
            && name.ends_with(".rs")
        {
            files.push(p.to_path_buf());
        }
    });
    files.sort();

    // Rebuild when any target file or this build script changes
    for f in &files {
        println!("cargo:rerun-if-changed={}", f.display());
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    println!("cargo:rerun-if-env-changed=DEP_RUSTLS_FFI_INCLUDE");
    let inc = env::var("DEP_RUSTLS_FFI_INCLUDE")
        .expect("rustls-ffi did not export its include dir (DEP_RUSTLS_FFI_INCLUDE)");
    let src = Path::new(&inc).join("rustls.h");
    let dst = crate_root.join("rustls").join("rustls.h");
    let content = fs::read(&src)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", src.display()));
    write_if_changed(&dst, content.as_slice());

    println!("cargo:rerun-if-env-changed=DEP_BROTLI_INCLUDE");
    let brotli_inc = env::var("DEP_BROTLI_INCLUDE")
        .expect("brotli-sys did not export its include dir (DEP_BROTLI_INCLUDE)");
    let brotli_src_dir = Path::new(&brotli_inc).join("brotli");
    let brotli_dst_dir = crate_root.join("brotli").join("brotli");
    fs::create_dir_all(&brotli_dst_dir).unwrap();
    for name in ["decode.h", "encode.h", "port.h", "types.h"] {
        let src = brotli_src_dir.join(name);
        let dst = brotli_dst_dir.join(name);
        let content = fs::read_to_string(&src)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", src.display()))
            .replace("#include <brotli/port.h>", "#include \"port.h\"")
            .replace("#include <brotli/types.h>", "#include \"types.h\"");
        write_if_changed(&dst, content.as_bytes());
    }

    // Generate modules that include absolute paths
    let mut buf = String::new();
    for f in files {
        let abs = f.canonicalize().unwrap_or(f.clone());

        // Stable, valid Rust module name from path
        let mut module_name = abs
            .strip_prefix(&crate_root)
            .unwrap_or(&abs)
            .to_string_lossy()
            .replace('\\', "/")
            .trim_start_matches("./")
            .replace(['/', '.', '-'], "_");

        if let Some(stripped) = module_name.strip_suffix("_rs") {
            module_name = stripped.to_string();
        }
        if !module_name
            .chars()
            .next()
            .map(|c| c.is_ascii_alphabetic() || c == '_')
            .unwrap_or(false)
        {
            module_name = format!("m_{}", module_name);
        }

        // Generate to OUT_DIR
        buf.push_str(&format!(
            "#[path = r#\"{}\"#]\npub mod {};\n",
            abs.display(),
            module_name
        ));
    }

    fs::create_dir_all(modules_rs.parent().unwrap()).unwrap();
    let mut f = fs::File::create(&modules_rs).unwrap();
    f.write_all(buf.as_bytes()).unwrap();
}

fn visit(dir: &Path, f: &mut impl FnMut(&Path)) {
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();

            // Skip noisy/build dirs
            if p.is_dir() {
                let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if matches!(name, "target" | ".git") {
                    continue;
                }
                visit(&p, f);
            } else {
                f(&p);
            }
        }
    }
}

// Keeps the header's timestamp when the content is unchanged, so the rule
// index, which depends on it, is not rebuilt after every target edit.
fn write_if_changed(dst: &Path, content: &[u8]) {
    if fs::read(dst).is_ok_and(|old| old == content) {
        return;
    }
    if let Some(dir) = dst.parent() {
        fs::create_dir_all(dir).unwrap();
    }
    fs::write(dst, content).unwrap_or_else(|e| panic!("failed to write {}: {e}", dst.display()));
}
