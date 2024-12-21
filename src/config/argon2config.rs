use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;
use argon2::{self, Argon2, Algorithm, Version, ParamsBuilder};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Argon2Config {
    pub memory_cost: u32,
    pub time_cost: u32,
    pub parallelism: u32,
    #[serde(serialize_with = "serialize_algorithm", deserialize_with = "deserialize_algorithm")]
    pub algorithm: Algorithm,
    #[serde(serialize_with = "serialize_version", deserialize_with = "deserialize_version")]
    pub version: Version,
    pub pepper: Option<String>,
}


fn serialize_algorithm<S>(algorithm: &Algorithm, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("{:?}", algorithm).to_lowercase())
}

fn deserialize_algorithm<'de, D>(deserializer: D) -> Result<Algorithm, D::Error>
where
    D: Deserializer<'de>,
{
    struct AlgorithmVisitor;

    impl<'de> Visitor<'de> for AlgorithmVisitor {
        type Value = Algorithm;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string representing an Algorithm")
        }

        fn visit_str<E>(self, value: &str) -> Result<Algorithm, E>
        where
            E: de::Error,
        {
            match value.to_lowercase().as_str() {
                "argon2d" => Ok(Algorithm::Argon2d),
                "argon2i" => Ok(Algorithm::Argon2i),
                "argon2id" => Ok(Algorithm::Argon2id),
                _ => Err(de::Error::unknown_variant(value, &["argon2d", "argon2i", "argon2id"])),
            }
        }
    }

    deserializer.deserialize_str(AlgorithmVisitor)
}

fn serialize_version<S>(version: &Version, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("{:?}", version))
}

fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    struct VersionVisitor;

    impl<'de> Visitor<'de> for VersionVisitor {
        type Value = Version;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string representing a Version")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Version, E>
        where
            E: de::Error,
        {
            match value {
                13 => Ok(Version::V0x13),
                10 => Ok(Version::V0x10),
                _ => Err(de::Error::unknown_variant(&value.to_string(), &["13", "10"])),
            }
        }

        fn visit_str<E>(self, value: &str) -> Result<Version, E>
        where
            E: de::Error,
        {
            let cleaned_value = value.to_lowercase().replace("version", "").replace("v", "").replace(" ", "");
            match cleaned_value.as_str() {
                "0x10" | "16" => Ok(Version::V0x10),
                "0x13" | "19" => Ok(Version::V0x13),
                _ => Err(de::Error::unknown_variant(&cleaned_value, &["0x13", "0x10"])),
            }
        }
    }

    deserializer.deserialize_any(VersionVisitor)
}

impl Argon2Config {
    pub fn initialize_argon2<'a>(&'a self) -> Argon2<'a> {
        let params = ParamsBuilder::new()
            .m_cost(self.memory_cost)
            .t_cost(self.time_cost)
            .p_cost(self.parallelism)
            .build()
            .unwrap();
        if let Some(pepper) = &self.pepper {
            Argon2::new_with_secret(pepper.as_bytes(), self.algorithm, self.version, params).unwrap()
        } else {
            Argon2::new(self.algorithm, self.version, params)
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_argon2config() {
        let config = Argon2Config {
            memory_cost: 65536,
            time_cost: 3,
            parallelism: 1,
            algorithm: Algorithm::Argon2id,
            version: Version::V0x13,
            pepper: Some("pepper".to_string()),
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"argon2id\""));
        assert!(json.contains("\"V0x13\""));
    }

    #[test]
    fn test_deserialize_argon2config() {
        let json = r#"
        {
            "memory_cost": 65536,
            "time_cost": 3,
            "parallelism": 1,
            "algorithm": "argon2id",
            "version": "V0x13",
            "pepper": "pepper"
        }
        "#;

        let config: Argon2Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.algorithm, Algorithm::Argon2id);
        assert_eq!(config.version, Version::V0x13);
    }
}