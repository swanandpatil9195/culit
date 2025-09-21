fn main() {
    println!("cargo:rustc-check-cfg=cfg(has_c_string)");
    if version_check::is_min_version("1.79.0").unwrap_or(false) {
        println!("cargo:rustc-cfg=has_c_string");
    }
}
