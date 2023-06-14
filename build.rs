fn main() {
    // if !cfg!(target_os = "windows") {
    //     panic!("minhook_raw supports only Windows system")
    // }

    let closure = |hde: &str| {
        let path =
            std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("minhook/src");

        cc::Build::new()
            .file(path.join("buffer.c"))
            .file(path.join("hook.c"))
            .file(path.join("trampoline.c"))
            .file(path.join(hde))
            .compile(&std::env::var("CARGO_PKG_NAME").unwrap());

        println!("cargo:rerun-if-changed=minhook/src");

        println!(
            "cargo:rustc-link-search=native={}",
            std::env::var("OUT_DIR").unwrap()
        );
    };

    if cfg!(target_arch = "x86_64") {
        return closure("hde/hde64.c");
    }

    if cfg!(target_arch = "x86") {
        return closure("hde/hde32.c");
    };
}
