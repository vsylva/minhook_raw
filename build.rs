fn main() {
    if let Some(toolchain) = std::env::var("RUSTUP_TOOLCHAIN").ok() {
        let parts: Vec<&str> = toolchain.split('-').collect();

        let (_channel, target, _device, os, _toolchain_name) =
            (parts[0], parts[1], parts[2], parts[3], parts[4]);
        if os != "windows" {
            panic!("Unsupported system. Windows only");
        }
        let closure = |hde: &str| {
            let path = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("minhook/src");

            cc::Build::new()
                .file(path.join("buffer.c"))
                .file(path.join("hook.c"))
                .file(path.join("trampoline.c"))
                .file(path.join(hde))
                .compile("minhook");

            println!("cargo:rerun-if-changed=minhook/src");

            // println!(
            //     "cargo:rustc-link-search=native={}",
            //     std::env::var("OUT_DIR").unwrap()
            // );
        };

        if target == "x86_64" {
            return closure("hde/hde64.c");
        }

        if target == "i686" {
            return closure("hde/hde32.c");
        };

        panic!("Only supports toolchain x86_64 and i686")
    } else {
        panic!("Failed to obtain RUSTUP_TOOLCHAIN");
    }
}
