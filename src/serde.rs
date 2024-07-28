use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::Snowflake;

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_serialize() {
        let snowflake = Snowflake(1234567890);
        let serialized = serde_json::to_string(&snowflake).unwrap();

        assert_eq!(serialized, "\"1234567890\"");
    }

    #[test]
    fn test_deserialize() {
        let deserialized: Snowflake = serde_json::from_value(json!("1234567890")).unwrap();

        assert_eq!(deserialized, Snowflake(1234567890));
    }
}