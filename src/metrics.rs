//metrics data structure
//basic function: inc/dec/snapshot
use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let count = data.entry(key.into()).or_insert(0);
        *count += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let count = data.entry(key.into()).or_insert(0);
        *count -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .clone())
    }
}
