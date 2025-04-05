use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=AX_PLAT_FAMILY");
    let root_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let platform_family = std::env::var("AX_PLAT_FAMILY").unwrap_or("dummy".to_string());

    let config_path = Path::new(&root_path).join(format!(
        "../platforms/axplat-{}/axconfig.toml",
        platform_family
    ));
    if config_path.exists() {
        println!(
            "cargo:rustc-env=AX_PLAT_CONFIG_PATH={}",
            config_path.display()
        );
    } else {
        panic!("Config file not found: {}", config_path.display());
    }
}
