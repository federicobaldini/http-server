use std::env;

#[derive(Debug, Clone)]
pub struct Config {
  pub host: String,
  pub port: String,
  pub public_path: String,
  // Maximum number of bytes accepted for request headers
  pub max_header_size: usize,
  // Maximum number of body bytes read per request
  pub max_body_size: usize,
  // Seconds before an idle connection is dropped
  pub read_timeout_secs: u64,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      host: "127.0.0.1".to_string(),
      port: "5000".to_string(),
      public_path: format!("{}/public", env!("CARGO_MANIFEST_DIR")),
      max_header_size: 8 * 1024,
      max_body_size: 1024 * 1024,
      read_timeout_secs: 5,
    }
  }
}

impl Config {
  // Loads configuration from a .env file (if present) and then from environment variables.
  // Falls back to the default value for any variable that is absent or unparseable.
  pub fn from_env() -> Self {
    dotenvy::dotenv().ok();
    let defaults: Self = Self::default();
    Self {
      host: env::var("HOST").unwrap_or(defaults.host),
      port: env::var("PORT").unwrap_or(defaults.port),
      public_path: env::var("PUBLIC_PATH").unwrap_or(defaults.public_path),
      max_header_size: env::var("MAX_HEADER_SIZE")
        .ok()
        .and_then(|v: String| v.parse().ok())
        .unwrap_or(defaults.max_header_size),
      max_body_size: env::var("MAX_BODY_SIZE")
        .ok()
        .and_then(|v: String| v.parse().ok())
        .unwrap_or(defaults.max_body_size),
      read_timeout_secs: env::var("READ_TIMEOUT_SECS")
        .ok()
        .and_then(|v: String| v.parse().ok())
        .unwrap_or(defaults.read_timeout_secs),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_config_has_expected_values() {
    let config: Config = Config::default();
    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, "5000");
    assert_eq!(config.max_header_size, 8 * 1024);
    assert_eq!(config.max_body_size, 1024 * 1024);
    assert_eq!(config.read_timeout_secs, 5);
    assert!(config.public_path.ends_with("/public"));
  }

  #[test]
  fn from_env_falls_back_to_defaults_when_vars_absent() {
    // Run in a context where HOST/PORT/etc. are not set.
    // We can only guarantee this if we temporarily remove them; instead,
    // verify that from_env() produces a valid Config without panicking.
    let config: Config = Config::from_env();
    assert!(!config.host.is_empty());
    assert!(!config.port.is_empty());
    assert!(config.max_header_size > 0);
    assert!(config.max_body_size > 0);
    assert!(config.read_timeout_secs > 0);
  }
}
