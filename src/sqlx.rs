use sqlx::{database::{HasArguments, HasValueRef}, encode::IsNull, Decode, Encode, MySql, Postgres, Sqlite};
use crate::Snowflake;

impl<'q> Encode<'q, MySql> for Snowflake {
    fn encode_by_ref(&self, buf: &mut <MySql as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        self.0.encode_by_ref(buf)
    }
}

impl<'q> Encode<'q, Postgres> for Snowflake {
    fn encode_by_ref(&self, buf: &mut <Postgres as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        let val = self.0 as i64;
        <i64 as Encode<'_, MySql>>::encode_by_ref(&val, buf)
    }
}

impl <'q> Encode<'q, Sqlite> for Snowflake {
    fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        let val = self.0 as i64;
        <i64 as Encode<'_, Sqlite>>::encode_by_ref(&val, buf)
    }
}

impl<'r, 'b> Decode<'r, MySql> for Snowflake {
    fn decode(value: <MySql as HasValueRef>::ValueRef) -> Result<Self, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        Ok(Self(u64::decode(value)?))
    }
}

impl<'r, 'b> Decode<'r, Postgres> for Snowflake {
    fn decode(value: <Postgres as HasValueRef>::ValueRef) -> Result<Self, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        Ok(Self(<i64 as Decode<Postgres>>::decode(value)? as u64))
    }
}

impl <'r, 'b> Decode<'r, Sqlite> for Snowflake {
    fn decode(value: <Sqlite as HasValueRef>::ValueRef) -> Result<Self, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        Ok(Self(<i64 as Decode<Sqlite>>::decode(value)? as u64))
    }
}