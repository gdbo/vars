pub fn setup(level: &str) {
    if std::env::var_os("RUST_LOG").is_none() {
        let env = format!("vars={},tower_http={}", level, level);
        std::env::set_var("RUST_LOG", env);
    }

    tracing_subscriber::fmt::init();
}
