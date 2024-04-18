#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    #[serde(default)]
    offset: Offset,
    #[serde(default)]
    limit: Limit,
}

impl Pagination {

    pub fn offset(&self) -> &Offset {
        &self.offset
    }

    pub fn limit(&self) -> &Limit {
        &self.limit
    }
    
}

#[derive(Debug, serde::Deserialize)]
pub struct Offset(pub i64);

impl Default for Offset {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Limit(pub i8);

impl Default for Limit {
    fn default() -> Self {
        Self(10)
    }
}
