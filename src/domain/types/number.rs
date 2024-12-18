use sqlx::{Database, Type, Postgres, encode::{Encode, IsNull}, postgres::PgArgumentBuffer};
use std::error::Error as StdError;
use std::fmt::Display;


type Result<T> = std::result::Result<T, Box<dyn StdError + Send + Sync + 'static>>;


pub trait Number<DB: Database = Postgres>: Copy + Display {
    fn type_info(&self) -> Option<DB::TypeInfo> {None}
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        let string = self.to_string();
        let string = string.as_str();
        <&str as Encode<Postgres>>::encode_by_ref(&string, buf)
    }
}

impl Number for u128 {
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        Some(<&str as Type<Postgres>>::type_info())
    }
}

impl Number for i128{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        Some(<&str as Type<Postgres>>::type_info())
    }
}

impl Number for u64{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        <i64 as Number>::type_info(&0)
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        let value = (*self as i64).wrapping_add(i64::MIN);
        <i64 as Number>::encode_by_ref(&value, buf)
    }
}

impl Number for i64{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        Some(<i64 as Type<Postgres>>::type_info())
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        <i64 as Encode<Postgres>>::encode_by_ref(&self, buf)
    }
}

impl Number for f64{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        Some(<f64 as Type<Postgres>>::type_info())
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        <f64 as Encode<Postgres>>::encode_by_ref(&self, buf)
    }
}

impl Number for u32{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        <i32 as Number>::type_info(&0)
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        let value = (*self as i32).wrapping_add(i32::MIN);
        <i32 as Number>::encode_by_ref(&value, buf)
    }
}

impl Number for i32{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        Some(<i32 as Type<Postgres>>::type_info())
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        <i32 as Encode<Postgres>>::encode_by_ref(&self, buf)
    }
}

impl Number for f32{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        Some(<f32 as Type<Postgres>>::type_info())
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        <f32 as Encode<Postgres>>::encode_by_ref(&self, buf)
    }
}

impl Number for u16{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        <i16 as Number>::type_info(&0)
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        let value = (*self as i16).wrapping_add(i16::MIN);
        <i16 as Number>::encode_by_ref(&value, buf)
    }
}

impl Number for i16{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        Some(<i16 as Type<Postgres>>::type_info())
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        <i16 as Encode<Postgres>>::encode_by_ref(&self, buf)
    }
}

impl Number for u8{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        <i16 as Number>::type_info(&0)
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        let value = *self as i16;
        <i16 as Encode<Postgres>>::encode_by_ref(&value, buf)
    }
}
impl Number for i8{
    fn type_info(&self) -> Option<<Postgres as Database>::TypeInfo> {
        <i16 as Number>::type_info(&0)
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull> {
        let value = *self as i16;
        <i16 as Encode<Postgres>>::encode_by_ref(&value, buf)
    }
}