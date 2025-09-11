use lib;

#[tokio::test]
async fn test_global_directories() {
    //
    // If we can initialize paths without them panicking then it's considered passed.
    //

    let _ = lib::GLOBAL_CONFIG_PATH.to_path_buf();
    let _ = lib::GLOBAL_CACHE_PATH.to_path_buf();
}

#[tokio::test]
async fn test_global_config() {
    let config = lib::GlobalConfigs::load().await;

    assert!(config.is_ok());
}
