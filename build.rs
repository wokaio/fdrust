use std::env;
use std::env::VarError;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;

fn version_is_nightly(version: &str) -> bool {
    version.contains("nightly")
}

fn cfg_rust_version() {
    let rustc = env::var("RUSTC").expect("RUSTC unset");

    let mut child = process::Command::new(rustc)
        .args(&["--version"])
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("spawn rustc");

    let mut rustc_version = String::new();

    child
        .stdout
        .as_mut()
        .expect("stdout")
        .read_to_string(&mut rustc_version)
        .expect("read_to_string");
    assert!(child.wait().expect("wait").success());

    if version_is_nightly(&rustc_version) {
        println!("cargo:rustc-cfg=rustc_nightly");
    }
}

fn cfg_serde() {
    match env::var("CARGO_FEATURE_WITH_SERDE") {
        Ok(_) => {
            println!("cargo:rustc-cfg=serde");
        }
        Err(VarError::NotUnicode(..)) => panic!(),
        Err(VarError::NotPresent) => {}
    }
}

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"))
}

fn version() -> String {
    env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION")
}

fn write_version() {
    let version = version();
    let version_ident = format!(
        "VERSION_{}",
        version.replace(".", "_").replace("-", "_").to_uppercase()
    );

    let mut out_file = File::create(Path::join(&out_dir(), "version.rs")).expect("open");
    writeln!(out_file, "/// fdutil crate version").unwrap();
    writeln!(out_file, "pub const FDUTIL_VERSION: &'static str = \"{}\";", version).unwrap();
    writeln!(out_file, "/// This symbol is used by fdutil").unwrap();
    writeln!(out_file, "#[doc(hidden)]").unwrap();
    writeln!(
        out_file,
        "pub const FDUTIL_VERSION_IDENT: &'static str = \"{}\";",
        version_ident
    )
    .unwrap();
    writeln!(
        out_file,
        "/// This symbol can be referenced to assert that proper version of crate is used"
    )
    .unwrap();
    writeln!(out_file, "pub const {}: () = ();", version_ident).unwrap();
    out_file.flush().unwrap();
}

fn main() {
    cfg_rust_version();
    cfg_serde();
    write_version();
}