use std::{ops::Deref, str::FromStr};
use errors::Result;
pub use errors::Error;
pub use generator::SnowflakeGenerator;


mod errors;
mod generator;
#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "sqlx")]
mod sqlx;
#[cfg(feature = "twilight")]
mod twilight;

pub const DISCORD_EPOCH: u64 = 1420070400000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Snowflake(u64);

impl Snowflake {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(&self) -> u64 {
        self.0
    }

    pub fn timestamp(&self, epoch: u64) -> u64 {
        (self.0 >> 22) + epoch
    }
}

impl FromStr for Snowflake {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s.parse::<u64>()?))
    }
}

impl std::fmt::Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Snowflake {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for Snowflake
where
    T: ?Sized,
    <Snowflake as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl From<u64> for Snowflake {
    fn from(value: u64) -> Self {
        Self(value)
    }
}