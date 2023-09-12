fn main() {
    #[cfg(not(feature = "doc-only"))]
    {
        let library = pkg_config::probe_library("libarchive")
            .expect("unable to locate libarchive on your system. Is it installed?");

        link(&library);

        #[cfg(feature = "generate-bindings")]
        generate_bindings(&library);
    }
}

#[cfg(not(feature = "doc-only"))]
fn link(library: &pkg_config::Library) {
    let mut build = cc::Build::new();

    build.file("csrc/wrapper.c");
    build.includes(&library.include_paths);

    for path in &library.link_paths {
        build.flag(&format!("-L{}", path.display()));
    }

    for name in &library.libs {
        build.flag(&format!("-l{name}"));
    }

    build.compile("archive-sys");
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(library: &pkg_config::Library) {
    use bindgen::MacroTypeVariation;
    use std::fs;

    // Generate bindings using bindgen
    {
        let builder = bindgen::Builder::default()
            .default_macro_constant_type(MacroTypeVariation::Signed)
            .header("csrc/wrapper.h")
            .allowlist_type("archive_.*")
            .allowlist_function("archive_.*")
            .allowlist_var("ARCHIVE_.*")
            .allowlist_var("AE_.*");
        let builder = library.include_paths.iter().fold(builder, |builder, path| {
            builder.clang_arg(format!("-I{}", path.display()))
        });
        let bindings = builder.generate().expect("unable to generate bindings");

        let out_file = "src/ffi_bindgen.rs";

        bindings
            .write_to_file(&out_file)
            .unwrap_or_else(|err| panic!("unable to write bindings to {out_file}: {err}"));
    }

    // Write manually written bindings
    {
        let out_file = "src/ffi_manual.rs";

        let source = generate_manual_bindings(&library);
        fs::write(&out_file, source)
            .unwrap_or_else(|err| panic!("unable to write bindings to {out_file}: {err}"));
    }
}

#[cfg(feature = "generate-bindings")]
fn generate_manual_bindings(library: &pkg_config::Library) -> String {
    use quote::quote;

    let version = detect_version(library);

    let ty = if version >= 3999000 {
        quote! { ::std::os::raw::c_int }
    } else {
        #[cfg(windows)]
        quote! { ::std::os::raw::c_ushort }

        #[cfg(not(windows))]
        quote! { ::libc::mode_t }
    };

    quote! {
        pub type __LA_MODE_T = #ty;

        pub const AE_IFMT: __LA_MODE_T = 0o170000;
        pub const AE_IFREG: __LA_MODE_T = 0o100000;
        pub const AE_IFLNK: __LA_MODE_T = 0o120000;
        pub const AE_IFSOCK: __LA_MODE_T = 0o140000;
        pub const AE_IFCHR: __LA_MODE_T = 0o020000;
        pub const AE_IFBLK: __LA_MODE_T = 0o060000;
        pub const AE_IFDIR: __LA_MODE_T = 0o040000;
        pub const AE_IFIFO: __LA_MODE_T = 0o010000;

    }
    .to_string()
}

#[cfg(feature = "generate-bindings")]
fn detect_version(library: &pkg_config::Library) -> u64 {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
    };

    let header_path = library
        .include_paths
        .iter()
        .map(|path| path.join("archive.h"))
        .find(|path| path.exists())
        .expect("Unable to find archive.h on the system. Is libarchive installed?");

    let reader = BufReader::new(
        File::open(&header_path)
            .unwrap_or_else(|err| panic!("fail to open {}: {err}", header_path.display())),
    );

    let version_text = reader
        .lines()
        .find_map(|line| {
            let line =
                line.unwrap_or_else(|err| panic!("fail to open {}: {err}", header_path.display()));
            let suffix = line.strip_prefix("#define	ARCHIVE_VERSION_NUMBER ")?;
            Some(suffix.to_string())
        })
        .expect("fail to detect the version of libarchive");

    version_text
        .parse()
        .unwrap_or_else(|err| panic!("invalid version number '{version_text}: {err}'"))
}
