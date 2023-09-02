fn main() {
    if !cfg!(windows) {
        panic!("Unsupported system, only supports Windows.");
    }

    if !cfg!(target_arch = "x86_64") && !cfg!(target_arch = "x86") && !cfg!(target_arch = "aarch64")
    {
        panic!("Unsupported target architecture, only supports x86_64, i686, and aarch64.");
    }

    let builder = |hde: &str| {
        let path =
            std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("minhook/src");

        cc::Build::new()
            .file(path.join("buffer.c"))
            .file(path.join("hook.c"))
            .file(path.join("trampoline.c"))
            .file(path.join(hde))
            .compile("minhook");

        println!("cargo:rerun-if-changed=minhook/src");
    };

    if cfg!(target_arch = "x86_64") || cfg!(target_arch = "aarch64") {
        return builder("hde/hde64.c");
    }

    if cfg!(target_arch = "x86") {
        return builder("hde/hde32.c");
    }
}

// fn main() {
//     let toolchain = std::env::var("RUSTUP_TOOLCHAIN").expect("Failed to obtain RUSTUP_TOOLCHAIN");

//     let parts: Vec<&str> = toolchain.split('-').collect();

//     let (_channel, arch, _vendor, os, _plaform) =
//         (parts[0], parts[1], parts[2], parts[3], parts[4]);

//     if os != "windows" {
//         panic!("Unsupported system. Windows only");
//     }

//     let closure = |hde: &str| {
//         let path =
//             std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("minhook/src");

//         cc::Build::new()
//             .file(path.join("buffer.c"))
//             .file(path.join("hook.c"))
//             .file(path.join("trampoline.c"))
//             .file(path.join(hde))
//             .compile("minhook");

//         println!("cargo:rerun-if-changed=minhook/src");
//     };

//     if arch == "x86_64" {
//         return closure("hde/hde64.c");
//     }

//     if arch == "i686" {
//         return closure("hde/hde32.c");
//     };

//     panic!("Only supports toolchain x86_64 and i686")
// }
