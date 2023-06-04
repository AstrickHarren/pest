use sha2::{Digest, Sha256};
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

fn display_digest(digest: &[u8]) -> String {
    digest.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn main() {
    println!("rerun-if-changed=src/grammar.pest");

    // Yes; build.rs is supposed to treat `src` as read-only; however:
    // We want to publish `grammar.rs` and not `grammar.pest`,
    // so putting it in `src` is the simplest way to do so.
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let grammar_pest_path = manifest_dir.join("src/grammar.pest");
    let grammar_rs_path = manifest_dir.join("src/grammar.rs");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let hash_path = out_dir.join("pest_hash.sha1");

    // HACK: don't try to build pest grammar from pest file
    assert!(
        grammar_rs_path.exists(),
        "package is broken; does not contain grammar.rs"
    );
}
