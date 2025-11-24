use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Get the OS-appropriate directory for storing fsqlctl data
/// This follows XDG standards on Linux and equivalent on other platforms
pub fn get_config_dir() -> PathBuf {
    if let Some(config_dir) = dirs::config_dir() {
        // On Linux: ~/.config/fsqlctl (XDG standard)
        // On macOS: ~/Library/Application Support/fsqlctl
        // On Windows: %APPDATA%\fsqlctl
        config_dir.join("fsqlctl")
    } else {
        // Fallback for other operating systems
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".fsqlctl")
    }
}

/// Configuration structure for fsqlctl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Mapping of hostnames to their corresponding API keys/tokens
    #[serde(rename = "api-keys")]
    pub api_keys: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_keys: HashMap::new(),
        }
    }
}

impl Config {
    /// Load configuration from the config file
    /// Creates a default config if the file doesn't exist
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = get_config_dir().join("config.toml");

        if !config_path.exists() {
            // Return default config if file doesn't exist
            return Ok(Self::default());
        }

        let config_content = fs::read_to_string(&config_path)?;
        let config = toml::from_str::<Config>(&config_content)?;
        Ok(config)
    }

    /// Save the current configuration to the config file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = get_config_dir();
        let config_path = config_dir.join("config.toml");

        // Ensure the config directory exists
        fs::create_dir_all(&config_dir)?;

        let config_content = toml::to_string_pretty(self)?;
        fs::write(&config_path, config_content)?;

        Ok(())
    }

    /// Get the stored API key/token for a specific host
    pub fn get_token(&self, host: &str) -> Option<&String> {
        self.api_keys.get(host)
    }

    /// Set (or update) the API key/token for a specific host
    pub fn set_token(&mut self, host: &str, token: &str) {
        self.api_keys.insert(host.to_string(), token.to_string());
    }

    /// Get the path to the config file
    pub fn get_config_path() -> PathBuf {
        get_config_dir().join("config.toml")
    }

    /// Get a user-friendly description of where the config file is located
    pub fn get_config_location() -> String {
        Self::get_config_path().display().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir_function() {
        let dir = get_config_dir();
        assert!(dir.to_string_lossy().contains("fsqlctl"));
    }

    #[test]
    fn test_config_creation() {
        let config = Config::default();
        assert!(config.api_keys.is_empty());
    }

    #[test]
    fn test_token_operations() {
        let mut config = Config::default();

        // Test setting and getting tokens
        config.set_token("api.query.ai", "test-token-123");
        assert_eq!(
            config.get_token("api.query.ai"),
            Some(&"test-token-123".to_string())
        );

        // Test nonexistent host
        assert_eq!(config.get_token("nonexistent.com"), None);

        // Test multiple hosts
        config.set_token("another.host", "different-token");
        assert_eq!(
            config.get_token("another.host"),
            Some(&"different-token".to_string())
        );
        assert_eq!(
            config.get_token("api.query.ai"),
            Some(&"test-token-123".to_string())
        );
    }

    #[test]
    fn test_serialization() {
        let mut config = Config::default();
        config.set_token("api.query.ai", "token-123");
        config.set_token("api.example.com", "token-456");

        // Test serializing to TOML
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("[api-keys]"));
        assert!(toml_str.contains("api.query.ai"));
        assert!(toml_str.contains("token-123"));

        // Test deserializing from TOML
        let deserialized: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(
            deserialized.get_token("api.query.ai"),
            Some(&"token-123".to_string())
        );
        assert_eq!(
            deserialized.get_token("api.example.com"),
            Some(&"token-456".to_string())
        );
    }
}
