fn main() {
    let config_path = std::env::var("APPCONFIG_PATH")
        .unwrap_or_else(|_| "configuration/appconfig.toml".to_string());
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new(&config_path);

    rust_log::setup_logger(Some(&config_loader.get_sub_config("logger"))).unwrap();
    rust_log::info!("Rust project starter template.");
}
