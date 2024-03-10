use std::path::{Path, PathBuf};

use example::QueryCache;

struct CacheDirStorage {
    cache: QueryCache,
}

impl CacheDirStorage {
    pub fn new(cache: QueryCache) -> Self {
        Self { cache }
    }
}

async fn test_ice() {
    CacheDirStorage::new(QueryCache()).await.unwrap();
}
