use sqlx::postgres::{PgTypeInfo, PgValueRef, PgArgumentBuffer};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use sqlx::{Encode, Decode, Postgres, Type, ValueRef};
use serde::de::{self, Visitor, MapAccess};
use serde::ser::SerializeStruct;
use std::str::FromStr;
use lettre::Address;
use std::fmt;

#[derive(Clone, Debug)]
pub enum EmailAddress {
    New(Address),
    Verified(Address),
}

impl Type<Postgres> for EmailAddress {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("jsonb")
    }
}

impl<'q> Encode<'q, Postgres> for EmailAddress {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let json_value = match self {
            EmailAddress::New(address) => serde_json::json!({
                "email": address.to_string(),
                "verified": false
            }),
            EmailAddress::Verified(address) => serde_json::json!({
                "email": address.to_string(),
                "verified": true
            }),
        };
        let json_string = serde_json::to_string(&json_value)?;
        <String as Encode<Postgres>>::encode_by_ref(&json_string, buf)
    }
}

impl<'r> Decode<'r, Postgres> for EmailAddress {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let json = <&str as Decode<Postgres>>::decode(value)?;
        Ok(serde_json::from_str(json)?)
    }
}

impl Serialize for EmailAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("EmailAddress", 2)?;
        match self {
            EmailAddress::New(address) => {
                state.serialize_field("email", &address.to_string())?;
                state.serialize_field("verified", &false)?;
            }
            EmailAddress::Verified(address) => {
                state.serialize_field("email", &address.to_string())?;
                state.serialize_field("verified", &true)?;
            }
        }
        state.end()
    }
}

struct EmailAddressVisitor;

impl<'de> Visitor<'de> for EmailAddressVisitor {
    type Value = EmailAddress;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or a struct EmailAddress")
    }

    fn visit_map<V>(self, mut map: V) -> Result<EmailAddress, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut email = None;
        let mut verified = None;
        while let Some(key) = map.next_key()? {
            match key {
                "email" => {
                    if email.is_some() {
                        return Err(de::Error::duplicate_field("email"));
                    }
                    email = Some(map.next_value()?);
                }
                "verified" => {
                    if verified.is_some() {
                        return Err(de::Error::duplicate_field("verified"));
                    }
                    verified = Some(map.next_value()?);
                }
                _ => {
                    let _: de::IgnoredAny = map.next_value()?;
                }
            }
        }
        let email: String = email.ok_or_else(|| de::Error::missing_field("email"))?;
        let verified: bool = verified.ok_or_else(|| de::Error::missing_field("verified"))?;
        let address = email.parse::<Address>().map_err(de::Error::custom)?;
        Ok(if verified {
            EmailAddress::Verified(address)
        } else {
            EmailAddress::New(address)
        })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error, {
        let address = match v.parse::<Address>() {
            Ok(address) => address,
            Err(err) => return Err(de::Error::custom(err))
        };
        Ok(EmailAddress::New(address))
    }
}

impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(EmailAddressVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_new() {
        let email = EmailAddress::New("user@domain.com".parse::<Address>().unwrap());
        let json = serde_json::to_string(&email).unwrap();
        assert_eq!(json, r#"{"email":"user@domain.com","verified":false}"#);
    }

    #[test]
    fn test_serialize_verified() {
        let email = EmailAddress::Verified("user@domain.com".parse::<Address>().unwrap());
        let json = serde_json::to_string(&email).unwrap();
        assert_eq!(json, r#"{"email":"user@domain.com","verified":true}"#);
    }

    #[test]
    fn test_deserialize_new() {
        let json = r#"{"email":"user@domain.com","verified":false}"#;
        let email: EmailAddress = serde_json::from_str(json).unwrap();
        match email {
            EmailAddress::New(address) => assert_eq!(address.to_string(), "user@domain.com"),
            _ => panic!("Expected EmailAddress::New"),
        }
    }

    #[test]
    fn test_deserialize_verified() {
        let json = r#"{"email":"user@domain.com","verified":true}"#;
        let email: EmailAddress = serde_json::from_str(json).unwrap();
        match email {
            EmailAddress::Verified(address) => assert_eq!(address.to_string(), "user@domain.com"),
            _ => panic!("Expected EmailAddress::Verified"),
        }
    }
}
