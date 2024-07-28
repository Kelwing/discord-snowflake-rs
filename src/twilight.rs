use twilight_model::id::Id;
use crate::Snowflake;

impl<T> From<Id<T>> for Snowflake {
    fn from(id: Id<T>) -> Self {
        Self(id.get())
    }
}

impl<T> From<Snowflake> for Id<T> {
    fn from(snowflake: Snowflake) -> Self {
        Self::new(snowflake.0)
    }
}