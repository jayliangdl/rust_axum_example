use serde::de::Deserializer;
use serde::Deserialize;

pub fn default_empty_to_string() -> String {
    "".to_string()
}

pub fn deserialize_null_to_empty_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer).map(|x| x.unwrap_or_default())
}