extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::Path;

fn main() {
    match pkg_config::find_library("libarchive") {
        Ok(lib) => generate_rust_bindings(&lib),
        Err(msg) => panic!("Unable to locate libarchive, err={:?}", msg),
    }
}

fn generate_rust_bindings(lib: &pkg_config::Library) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string());
    let out = Path::new(&manifest_dir);
    let file = Path::new(&out).join("src").join("ffi.rs");
    let h = lib.include_paths[0].join("comp.h").to_string_lossy().into_owned();
    let mut bindings = bindgen::builder();
    bindings.link("archive");
    bindings.emit_builtins();
    bindings.header(h);
    bindings.generate().unwrap().write_to_file(&file).unwrap();
}
