//metrics data structure
//basic function: inc/dec/snapshot
use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut count = self.data.entry(key.into()).or_insert(0);
        *count += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut count = self.data.entry(key.into()).or_insert(0);
        *count -= 1;
        Ok(())
    }
}

impl std::fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
