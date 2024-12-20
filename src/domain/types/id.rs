use sqlx::{Encode, Decode, Type, Postgres, postgres::{PgValueRef, PgTypeInfo, PgArgumentBuffer}};
use serde::{Serialize, Deserialize, Serializer};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use bson::oid::ObjectId;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Id(ObjectId);


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;


// Implement Type for Id
impl Type<Postgres> for Id {
    fn type_info() -> PgTypeInfo {
        <[u8] as Type<Postgres>>::type_info()
    }
}


// Implement Encode for Id
impl<'q> Encode<'q, Postgres> for Id {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<sqlx::encode::IsNull> {
        buf.extend_from_slice(&self.0.bytes());
        Ok(sqlx::encode::IsNull::No)
    }
}

// Implement Decode for Id
impl<'r> Decode<'r, Postgres> for Id {
    fn decode(value: PgValueRef<'r>) -> Result<Self> {
        let bytes: &[u8] = value.as_bytes()?;
        let byte_array: [u8; 12] = bytes.as_ref().try_into().map_err(|_| "Invalid length")?;
        let object_id = ObjectId::from_bytes(byte_array);
        Ok(Id(object_id))
    }
}


impl Deref for Id {
    type Target = ObjectId;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Id {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl FromStr for Id {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Id(ObjectId::from_str(s)?))
    }
}


impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.serialize_str(&self.0.to_hex())
    }
}