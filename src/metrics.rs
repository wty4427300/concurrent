use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct Metrics {
    pub data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut date = self.data
            .write()
            .map_err(|e| anyhow!(e.to_string()))?;
        let counter = date.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let date = self.data
            .read()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone();
        Ok(date)
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self.data.read().map_err(|_| fmt::Error {})?;
        for (key, value) in data.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}