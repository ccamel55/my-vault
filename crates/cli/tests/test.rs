// use lib::config;
//
// #[tokio::test]
// async fn config_test() {
//     let config = config::SomeConfig {
//         value_1: "".to_string(),
//         value_2: Some("SHSD".to_string()),
//         value_3: Some(config::SomeOtherConfig {
//             value_1: 69,
//             value_2: 420.69,
//             value_3: false,
//             value_4: vec!["a".to_string(), "b".to_string()],
//         }),
//     };
//
//     let config_deserialized = toml::to_string_pretty(&config);
//
//     assert!(config_deserialized.is_ok());
//
//     let config_serialized = toml::from_str::<config::SomeConfig>(&config_deserialized.unwrap());
//
//     assert!(config_serialized.is_ok());
// }
