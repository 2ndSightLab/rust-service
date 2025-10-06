use serde::Deserialize;

#[must_use]
pub const fn get_config_file_name() -> &'static str {
    "action.toml"
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActionConfig(pub toml::Value);

impl ActionConfig {
    #[must_use]
    pub fn get<T>(&self, key: &str) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.0.get(key)?.clone().try_into().ok()
    }
}
