use sqlx::{ encode::IsNull, Decode, Encode, MySql, Postgres, Sqlite};
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

impl<'r, 'b> Decode<'r, MySql> for Snowflake {
    fn decode(value: <MySql as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self(<u64 as Decode<MySql>>::decode(value)?))
    }
}

impl<'r, 'b> Decode<'r, Postgres> for Snowflake {
    fn decode(value: <Postgres as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self(<i64 as Decode<Postgres>>::decode(value)? as u64))
    }
}

impl <'r, 'b> Decode<'r, Sqlite> for Snowflake {
    fn decode(value: <Sqlite as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self(<i64 as Decode<Sqlite>>::decode(value)? as u64))
    }
}