//metrics data structure
//basic function: inc/dec/snapshot
use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let count = data.entry(key.into()).or_insert(0);
        *count += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let count = data.entry(key.into()).or_insert(0);
        *count -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .clone())
    }
}

impl std::fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data.read().map_err(|_| std::fmt::Error {})?;
        for (key, value) in data.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
