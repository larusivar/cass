use anyhow::Result;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    pub agents: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SearchHit {
    pub title: String,
    pub snippet: String,
    pub score: f32,
    pub source_path: String,
}

#[derive(Clone)]
pub struct SearchClient;

impl SearchClient {
    pub fn open(_path: &Path) -> Result<Option<Self>> {
        Ok(None)
    }

    pub fn search(
        &self,
        _query: &str,
        _filters: SearchFilters,
        _limit: usize,
    ) -> Result<Vec<SearchHit>> {
        Ok(Vec::new())
    }
}
