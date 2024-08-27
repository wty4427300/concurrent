use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct Metrics {
    pub date: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            date: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut date = self.date
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?;
        let counter = date.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let date = self.date
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone();
        Ok(date)
    }
}