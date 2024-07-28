use crate::errors::Result;
use crate::Snowflake;
use std::time::{SystemTime, UNIX_EPOCH};

/// A generator for Discord-compatible Snowflake IDs.
pub struct SnowflakeGenerator {
    epoch: u64,
    worker_id: u64,
    process_id: u64,
    sequence: u64,
}

impl SnowflakeGenerator {
    /// Create a new Snowflake generator.
    /// 
    /// # Arguments
    /// 
    /// * `epoch` - The epoch to use for the generator.
    /// * `worker_id` - The worker ID to use for the generator.
    /// * `process_id` - The process ID to use for the generator.
    /// 
    /// # Returns
    /// 
    /// A new Snowflake generator.
    pub fn new(epoch: u64, worker_id: u64, process_id: u64) -> Self {
        SnowflakeGenerator {
            epoch,
            worker_id,
            process_id,
            sequence: 0,
        }
    }

    /// Generate a new Snowflake.
    /// 
    /// # Returns
    /// 
    /// A new Snowflake.
    /// 
    /// # Errors
    /// 
    /// This method will return an error if the current timestamp cannot be retrieved.
    pub fn generate(&mut self) -> Result<Snowflake> {
        let timestamp = Self::current_timestamp()?;
        let mut snowflake = 0;

        snowflake |= (timestamp - self.epoch) << 22;
        snowflake |= self.worker_id << 17;
        snowflake |= self.process_id << 12;
        snowflake |= self.sequence;

        self.sequence += 1;

        Ok(Snowflake(snowflake))
    }

    /// Generate a new Snowflake, panicking if an error occurs.
    /// 
    /// # Returns
    /// 
    /// A new Snowflake.
    /// 
    /// # Panics
    /// 
    /// This method will panic if the current timestamp cannot be retrieved.
    pub fn must_generate(&mut self) -> Snowflake {
        self.generate().unwrap()
    }

    fn current_timestamp() -> Result<u64> {
        Ok(SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs())
    }
}

impl Default for SnowflakeGenerator {
    fn default() -> Self {
        SnowflakeGenerator::new(1420070400000, 0, 0)
    }
}