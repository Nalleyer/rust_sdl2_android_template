use std::env;

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let triple = target.as_str();
    let target_os = get_os_from_triple(triple).unwrap();
    if target_os.contains("android") {
        println!("cargo:rustc-flags=-l SDL2_image");
    }
    if let Some(lib_dir) = get_lib_dir_name_from_triple(triple) {
         println!("cargo:rustc-link-search=android-project/app/build/intermediates/stripped_native_libs/debug/out/lib/{}", lib_dir);
    }
}

fn get_os_from_triple(triple: &str) -> Option<&str> {
    triple.splitn(3, "-").nth(2)
}

fn get_lib_dir_name_from_triple(triple: &str) -> Option<&'static str> {
    match triple {
        "aarch64-linux-android" => Some("arm64-v8a"),
        "armv7-linux-androideabi" => Some("armeabi-v7a"),
        "i686-linux-android" => Some("x86"),
        _ => None,
    }
}
