use tenchi_mcp::config::Config;

#[test]
fn test_config_parsing() {
    let toml_str = r#"
[global]
ollama_url = "http://localhost:11434"
default_timeout = 300

[[models]]
name = "test-model"
role = "Coder"
description = "Test desc"
priority = 1
system_prompt = "Test prompt"
[models.options]
temperature = 0.5
"#;
    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(config.global.ollama_url, "http://localhost:11434");
    assert_eq!(config.models[0].options.as_ref().unwrap().get("temperature").unwrap().as_f64().unwrap(), 0.5);
}
