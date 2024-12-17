use sqlx::{postgres::{PgArgumentBuffer, Postgres, PgTypeInfo}, Encode, encode::IsNull, types::Json, Type};
use std::{any::TypeId, error::Error as StdError};
use serde::{Serialize, Deserialize};
use std::fmt::{Formatter, Display};
use std::collections::HashMap;
use super::Number;

type Result<T> = std::result::Result<T, Box<dyn StdError + Send + Sync + 'static>>;


#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(untagged)]
pub enum Value<N: Number = i64> {
    #[default]
    None,
    Bool(bool),
    Number(N),
    String(String),
    Array(Vec<Value<N>>),
    Map(HashMap<String, Value<N>>)
}


impl<N: Number> Type<Postgres> for Value<N> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("JSONB")
    }

    fn compatible(_ty: &PgTypeInfo) -> bool {
        true
    }
}


impl<'a, N: Number + 'static + Encode<'a, Postgres> + Serialize + Default + PartialEq> Value<N> {
    pub fn is_map(&self) -> bool {
        match self {
            Self::Map(_) => true,
            _ => false,
        }
    }

    pub fn type_id(&self) -> Option<TypeId> {
        match self {
            Self::None => None,
            Self::Bool(_) => Some(TypeId::of::<bool>()),
            Self::Number(_) => Some(TypeId::of::<N>()),
            Self::String(_) => Some(TypeId::of::<String>()),
            Self::Array(_) => Some(TypeId::of::<Vec<Value<N>>>()),
            Self::Map(_) => Some(TypeId::of::<HashMap<String, Value<N>>>())
        }
    }


    fn encode_array(array: &Vec<Value<N>>, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        let mut type_id = None;
        if array.len() == 0 {
            return Ok(IsNull::Yes);
        }

        buf.push(b'{');
        for (index, value) in array.into_iter().enumerate() {
            if index > 0 {
                buf.push(b',');
            }
            if let Some(typ) = value.type_id() {
                match type_id {
                    None => type_id = Some(typ),
                    Some(typ_id) => if typ != typ_id {return Err("putting different data types in the same array".into());}
                }
            }
            let is_null = <Value<N> as Encode<Postgres>>::encode_by_ref(value, buf)?;
            match is_null {
                IsNull::Yes => buf.extend_from_slice(b"NULL"),
                IsNull::No => ()
            }
            
        }
        buf.push(b'}');
        Ok(IsNull::No)
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::None => true,
            Self::Bool(_) => false,
            Self::Number(_) => false,
            Self::String(value) => value.is_empty(),
            Self::Array(array) => array.is_empty(),
            Self::Map(map) => map.is_empty()
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Self::Number(number) => number == &Default::default(),
            _ => false
        }
    }

    pub fn as_option_from_option(option: Option<Self>) -> Option<Self> {
        if let Some(value) = option {
            if value.is_empty() || value.is_zero() {
                None
            }else {
                Some(value)
            }
        }else {
            None
        }
    }
}


impl<'a, N: Number + 'static + Encode<'a, Postgres> + Serialize + Default + PartialEq> Encode<'_, Postgres> for Value<N> {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        match self {
            Value::None => Ok(IsNull::Yes),
            Value::Bool(b) => <bool as Encode<Postgres>>::encode_by_ref(b, buf),
            Value::Number(n) => n.encode_by_ref(buf),
            Value::String(s) => <String as Encode<Postgres>>::encode_by_ref(s, buf),
            Value::Array(array) => {
                Value::encode_array(array, buf)
            },
            Value::Map(map) => {
                let json = Json::from(map);
                <Json<&HashMap<String, Value<N>>> as Encode<Postgres>>::encode_by_ref(&json, buf)
            }
        }
    }

    fn produces(&self) -> Option<<Postgres as sqlx::Database>::TypeInfo> {
        match self {
            Self::None => None,
            Self::Bool(bool) => Some(<bool as Type<Postgres>>::type_info()),
            Self::Number(_) => Some(<i64 as Type<Postgres>>::type_info()),
            Self::String(_) => Some(<String as Type<Postgres>>::type_info()),
            Self::Array(value) => None,//Some(PgTypeInfo::with_name(&format!("{}[]", value.produces().unwrap_or(<Value as Type<Postgres>>::type_info())))),
            Self::Map(_) => Some(<Value as Type<Postgres>>::type_info())
        }
    }
}