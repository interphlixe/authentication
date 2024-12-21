use sqlx::{Encode, Decode, Postgres, Type, ValueRef, types::Json};
use sqlx::postgres::{PgTypeInfo, PgValueRef, PgArgumentBuffer};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor, MapAccess};
use serde::ser::SerializeStruct;
use std::str::FromStr;
use lettre::Address;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum EmailAddress {
    New(Address),
    Verified(Address),
}

impl Type<Postgres> for EmailAddress {
    fn type_info() -> PgTypeInfo {
        <Json<EmailAddress> as Type<Postgres>>::type_info()
    }
}

impl<'q> Encode<'q, Postgres> for EmailAddress {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let json = Json::from(self);
        <Json<&EmailAddress> as Encode<Postgres>>::encode_by_ref(&json, buf)
    }
}

impl<'r> Decode<'r, Postgres> for EmailAddress {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let email_address = <Json<EmailAddress> as Decode<Postgres>>::decode(value)?.0;
        Ok(email_address)
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


impl From<EmailAddress> for Address {
    fn from(email: EmailAddress) -> Self {
        match email {
            EmailAddress::New(address) => address,
            EmailAddress::Verified(address) => address
        }
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

    fn test_serialize(email: EmailAddress, expected_json: &str) {
        let json = serde_json::to_string(&email).unwrap();
        assert_eq!(json, expected_json);
    }

    fn test_deserialize(json: &str, expected_email: EmailAddress) {
        let email: EmailAddress = serde_json::from_str(json).unwrap();
        assert_eq!(email, expected_email);
    }

    #[test]
    fn test_email_address_serialization() {
        test_serialize(
            EmailAddress::New("user@domain.com".parse::<Address>().unwrap()),
            r#"{"email":"user@domain.com","verified":false}"#,
        );
        test_serialize(
            EmailAddress::Verified("user@domain.com".parse::<Address>().unwrap()),
            r#"{"email":"user@domain.com","verified":true}"#,
        );
    }

    #[test]
    fn test_email_address_deserialization() {
        test_deserialize(
            r#"{"email":"user@domain.com","verified":false}"#,
            EmailAddress::New("user@domain.com".parse::<Address>().unwrap()),
        );
        test_deserialize(
            r#"{"email":"user@domain.com","verified":true}"#,
            EmailAddress::Verified("user@domain.com".parse::<Address>().unwrap()),
        );
    }
}
