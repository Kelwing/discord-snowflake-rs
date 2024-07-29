use sqlx::{encode::IsNull, Decode, Encode, MySql, Postgres, Sqlite, Type, Database};
use crate::Snowflake;

impl<'q> Encode<'q, MySql> for Snowflake {
    fn encode_by_ref(
            &self,
            buf: &mut <MySql as sqlx::Database>::ArgumentBuffer<'q>,
        ) -> Result<IsNull, sqlx::error::BoxDynError> {
        <u64 as Encode<'q, MySql>>::encode_by_ref(&self.0, buf)
    }
}

impl<'q> Encode<'q, Postgres> for Snowflake {
    fn encode_by_ref(
            &self,
            buf: &mut <Postgres as sqlx::Database>::ArgumentBuffer<'q>,
        ) -> Result<IsNull, sqlx::error::BoxDynError> {
        let val = self.0 as i64;
        <i64 as Encode<'q, Postgres>>::encode_by_ref(&val, buf)
    }
}

impl <'q> Encode<'q, Sqlite> for Snowflake {
    fn encode_by_ref(
            &self,
            buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
        ) -> Result<IsNull, sqlx::error::BoxDynError> {
        let val = self.0 as i64;
        <i64 as Encode<'q, Sqlite>>::encode_by_ref(&val, buf)
    }
}

impl<'r> Decode<'r, MySql> for Snowflake {
    fn decode(value: <MySql as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self(<u64 as Decode<MySql>>::decode(value)?))
    }
}

impl<'r> Decode<'r, Postgres> for Snowflake {
    fn decode(value: <Postgres as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self(<i64 as Decode<Postgres>>::decode(value)? as u64))
    }
}

impl <'r> Decode<'r, Sqlite> for Snowflake {
    fn decode(value: <Sqlite as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self(<i64 as Decode<Sqlite>>::decode(value)? as u64))
    }
}

impl Type<MySql> for Snowflake {
    fn type_info() -> <MySql as Database>::TypeInfo {
        <u64 as Type<MySql>>::type_info()
    }
}

impl Type<Postgres> for Snowflake {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        <i64 as Type<Postgres>>::type_info()
    }
}

impl Type<Sqlite> for Snowflake {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
}