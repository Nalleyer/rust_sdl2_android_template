use std::env;

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let target_os = get_os_from_triple(target.as_str()).unwrap();
    if target_os.contains("android") {
        println!("cargo:rustc-flags=-l SDL2_image");
    }
}

fn get_os_from_triple(triple: &str) -> Option<&str> {
    triple.splitn(3, "-").nth(2)
}
