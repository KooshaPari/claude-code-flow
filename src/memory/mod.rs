use anyhow::{Result, Context};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use uuid::Uuid;

// SQLite backend temporarily disabled due to rusqlite/sqlx conflict
// #[cfg(feature = "sqlite")]
// use rusqlite::{Connection, params};

use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: Uuid,
    pub key: String,
    pub value: String,
    pub namespace: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_entries: u64,
    pub namespaces: u64,
    pub storage_size_bytes: u64,
    pub compression_ratio: f32,
    pub cache_hit_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatus {
    pub backend: String,
    pub connected: bool,
    pub last_sync: Option<u64>,
    pub pending_operations: u64,
}

pub trait MemoryBackend: Send + Sync {
    async fn store(&self, entry: &MemoryEntry) -> Result<()>;
    async fn retrieve(&self, namespace: &str, key: &str) -> Result<Option<MemoryEntry>>;
    async fn query(&self, namespace: Option<&str>, pattern: &str) -> Result<Vec<MemoryEntry>>;
    async fn delete(&self, namespace: &str, key: &str) -> Result<bool>;
    async fn list_namespaces(&self) -> Result<Vec<String>>;
    async fn get_stats(&self) -> Result<MemoryStats>;
    async fn export(&self, namespace: Option<&str>) -> Result<Vec<MemoryEntry>>;
    async fn import(&self, entries: &[MemoryEntry]) -> Result<u64>;
    async fn optimize(&self) -> Result<()>;
}

pub struct MemoryManager {
    backend: Box<dyn MemoryBackend>,
    cache: RwLock<HashMap<String, MemoryEntry>>,
    config: crate::config::MemoryConfig,
}

impl MemoryManager {
    pub async fn new(config: &Config) -> Result<Self> {
        info!("Initializing memory manager with backend: {}", config.memory.backend);
        
        let backend: Box<dyn MemoryBackend> = match config.memory.backend.as_str() {
            "sqlite" => {
                warn!("SQLite backend temporarily disabled due to dependency conflicts, falling back to JSON");
                Box::new(JsonBackend::new(&config.memory).await?)
            },
            "json" => Box::new(JsonBackend::new(&config.memory).await?),
            "memory" => Box::new(InMemoryBackend::new(&config.memory).await?),
            _ => {
                error!("Unknown memory backend: {}", config.memory.backend);
                return Err(anyhow::anyhow!("Unknown memory backend: {}", config.memory.backend));
            }
        };
        
        Ok(Self {
            backend,
            cache: RwLock::new(HashMap::new()),
            config: config.memory.clone(),
        })
    }
    
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing memory system");
        
        // Perform any necessary setup
        self.backend.optimize().await?;
        
        info!("Memory system initialized successfully");
        Ok(())
    }
    
    pub async fn store(&self, key: &str, value: &str, namespace: &str) -> Result<()> {
        let entry = MemoryEntry {
            id: Uuid::new_v4(),
            key: key.to_string(),
            value: value.to_string(),
            namespace: namespace.to_string(),
            created_at: self.current_timestamp(),
            updated_at: self.current_timestamp(),
            metadata: HashMap::new(),
        };
        
        debug!("Storing memory entry: {} in namespace: {}", key, namespace);
        
        // Store in backend
        self.backend.store(&entry).await?;
        
        // Update cache
        let cache_key = format!("{}:{}", namespace, key);
        self.cache.write().await.insert(cache_key, entry);
        
        Ok(())
    }
    
    pub async fn retrieve(&self, namespace: &str, key: &str) -> Result<Option<MemoryEntry>> {
        let cache_key = format!("{}:{}", namespace, key);
        
        // Check cache first
        if let Some(entry) = self.cache.read().await.get(&cache_key) {
            debug!("Cache hit for: {}", cache_key);
            return Ok(Some(entry.clone()));
        }
        
        // Fallback to backend
        debug!("Cache miss for: {}, querying backend", cache_key);
        let entry = self.backend.retrieve(namespace, key).await?;
        
        // Update cache if found
        if let Some(ref entry) = entry {
            self.cache.write().await.insert(cache_key, entry.clone());
        }
        
        Ok(entry)
    }
    
    pub async fn query(&self, pattern: &str, namespace: Option<&str>) -> Result<Vec<MemoryEntry>> {
        debug!("Querying memory: pattern='{}', namespace={:?}", pattern, namespace);
        self.backend.query(namespace, pattern).await
    }
    
    pub async fn delete(&self, namespace: &str, key: &str) -> Result<bool> {
        debug!("Deleting memory entry: {} in namespace: {}", key, namespace);
        
        let cache_key = format!("{}:{}", namespace, key);
        self.cache.write().await.remove(&cache_key);
        
        self.backend.delete(namespace, key).await
    }
    
    pub async fn list_namespaces(&self) -> Result<Vec<String>> {
        self.backend.list_namespaces().await
    }
    
    pub async fn get_stats(&self) -> Result<MemoryStats> {
        self.backend.get_stats().await
    }
    
    pub async fn get_status(&self) -> Result<MemoryStatus> {
        let stats = self.backend.get_stats().await?;
        
        Ok(MemoryStatus {
            backend: self.config.backend.clone(),
            connected: self.test_backend_connection().await,
            last_sync: Some(self.current_timestamp()),
            pending_operations: self.get_pending_operations_count().await
        })
    }
    
    pub async fn export_to_file(&self, file_path: &str, namespace: Option<&str>) -> Result<()> {
        info!("Exporting memory to file: {}", file_path);
        
        let entries = self.backend.export(namespace).await?;
        let json = serde_json::to_string_pretty(&entries)?;
        
        tokio::fs::write(file_path, json).await
            .with_context(|| format!("Failed to write export file: {}", file_path))?;
        
        info!("Exported {} entries to {}", entries.len(), file_path);
        Ok(())
    }
    
    pub async fn import_from_file(&self, file_path: &str) -> Result<()> {
        info!("Importing memory from file: {}", file_path);
        
        let content = tokio::fs::read_to_string(file_path).await
            .with_context(|| format!("Failed to read import file: {}", file_path))?;
        
        let entries: Vec<MemoryEntry> = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse import file: {}", file_path))?;
        
        let imported = self.backend.import(&entries).await?;
        
        // Clear cache to force reload
        self.cache.write().await.clear();
        
        info!("Imported {} entries from {}", imported, file_path);
        Ok(())
    }
    
    pub async fn optimize_storage(&self) -> Result<()> {
        info!("Optimizing memory storage");
        self.backend.optimize().await?;
        
        // Clear cache to force reload with optimized data
        self.cache.write().await.clear();
        
        info!("Memory storage optimization complete");
        Ok(())
    }
    
    async fn test_backend_connection(&self) -> bool {
        // Test backend connectivity by performing a simple operation
        match self.backend.get_stats().await {
            Ok(_) => true,
            Err(e) => {
                warn!("Backend connection test failed: {}", e);
                false
            }
        }
    }
    
    async fn get_pending_operations_count(&self) -> u64 {
        // In a real implementation, this would track queued/pending operations
        // For now, return 0 as operations are synchronous
        0
    }
    
    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

// SQLite Backend Implementation - DISABLED due to rusqlite/sqlx conflict
// #[cfg(feature = "sqlite")]
// use tokio::sync::Mutex;

// #[cfg(feature = "sqlite")]
// struct SqliteBackend {
//     connection: Mutex<Connection>,
//     config: crate::config::MemoryConfig,
// }

// #[cfg(feature = "sqlite")]
// impl SqliteBackend {
    // async fn new(config: &crate::config::MemoryConfig) -> Result<Self> {
    //     let db_path = Self::expand_path(&config.database_path);
    //     
    //     // Create directory if it doesn't exist
    //     if let Some(parent) = db_path.parent() {
    //         tokio::fs::create_dir_all(parent).await?;
    //     }
    //     
    //     let conn = Connection::open(&db_path)
    //         .with_context(|| format!("Failed to open SQLite database: {}", db_path.display()))?;
    //     
    //     // Create tables
    //     conn.execute(
    //         "CREATE TABLE IF NOT EXISTS memory_entries (
    //             id TEXT PRIMARY KEY,
    //             key TEXT NOT NULL,
    //             value TEXT NOT NULL,
    //             namespace TEXT NOT NULL,
    //             created_at INTEGER NOT NULL,
    //             updated_at INTEGER NOT NULL,
    //             metadata TEXT NOT NULL
    //         )",
    //         [],
    //     )?;
    //     
    //     conn.execute(
    //         "CREATE INDEX IF NOT EXISTS idx_namespace_key ON memory_entries(namespace, key)",
    //         [],
    //     )?;
    //     
    //     conn.execute(
    //         "CREATE INDEX IF NOT EXISTS idx_namespace ON memory_entries(namespace)",
    //         [],
    //     )?;
    //     
    //     info!("SQLite memory backend initialized: {}", db_path.display());
    //     
    //     Ok(Self {
    //         connection: Mutex::new(conn),
    //         config: config.clone(),
    //     })
    // }
    // 
    // fn expand_path(path: &str) -> std::path::PathBuf {
    //     if path.starts_with("~/") {
    //         if let Some(home) = directories::UserDirs::new() {
    //             return home.home_dir().join(&path[2..]);
    //         }
    //     }
    //     std::path::PathBuf::from(path)
    // }
// }

/* SQLite Backend Implementation - DISABLED due to rusqlite/sqlx dependency conflict
#[cfg(feature = "sqlite")]
#[async_trait]
impl MemoryBackend for SqliteBackend {
    async fn store(&self, entry: &MemoryEntry) -> Result<()> {
        // Implementation disabled
        Ok(())
    }
    
    async fn retrieve(&self, namespace: &str, key: &str) -> Result<Option<MemoryEntry>> {
        let conn = self.connection.lock().await;
        
        let mut stmt = conn.prepare(
            "SELECT id, key, value, namespace, created_at, updated_at, metadata 
             FROM memory_entries WHERE namespace = ?1 AND key = ?2"
        )?;
        
        let mut rows = stmt.query_map(rusqlite::params![namespace, key], |row| {
            let metadata_json: String = row.get(6)?;
            let metadata: HashMap<String, String> = serde_json::from_str(&metadata_json)
                .unwrap_or_default();
            
            Ok(MemoryEntry {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                key: row.get(1)?,
                value: row.get(2)?,
                namespace: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                metadata,
            })
        })?;
        
        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }
    
    async fn query(&self, namespace: Option<&str>, pattern: &str) -> Result<Vec<MemoryEntry>> {
        let conn = self.connection.lock().await;
        
        let (sql, params): (String, Vec<&str>) = match namespace {
            Some(ns) => (
                "SELECT id, key, value, namespace, created_at, updated_at, metadata 
                 FROM memory_entries WHERE namespace = ?1 AND (key LIKE ?2 OR value LIKE ?2)".to_string(),
                vec![ns, pattern]
            ),
            None => (
                "SELECT id, key, value, namespace, created_at, updated_at, metadata 
                 FROM memory_entries WHERE key LIKE ?1 OR value LIKE ?1".to_string(),
                vec![pattern]
            ),
        };
        
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            let metadata_json: String = row.get(6)?;
            let metadata: HashMap<String, String> = serde_json::from_str(&metadata_json)
                .unwrap_or_default();
            
            Ok(MemoryEntry {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                key: row.get(1)?,
                value: row.get(2)?,
                namespace: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                metadata,
            })
        })?;
        
        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }
        
        Ok(entries)
    }
    
    async fn delete(&self, namespace: &str, key: &str) -> Result<bool> {
        let conn = self.connection.lock().await;
        
        let changes = conn.execute(
            "DELETE FROM memory_entries WHERE namespace = ?1 AND key = ?2",
            rusqlite::params![namespace, key],
        )?;
        
        Ok(changes > 0)
    }
    
    async fn list_namespaces(&self) -> Result<Vec<String>> {
        let conn = self.connection.lock().await;
        
        let mut stmt = conn.prepare("SELECT DISTINCT namespace FROM memory_entries")?;
        let rows = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;
        
        let mut namespaces = Vec::new();
        for row in rows {
            namespaces.push(row?);
        }
        
        Ok(namespaces)
    }
    
    async fn get_stats(&self) -> Result<MemoryStats> {
        let conn = self.connection.lock().await;
        
        let total_entries: u64 = conn.query_row(
            "SELECT COUNT(*) FROM memory_entries",
            [],
            |row| row.get(0)
        )?;
        
        let namespaces: u64 = conn.query_row(
            "SELECT COUNT(DISTINCT namespace) FROM memory_entries",
            [],
            |row| row.get(0)
        )?;
        
        // Calculate actual storage size by analyzing database file
        let storage_size_bytes = if let Ok(metadata) = tokio::fs::metadata(&Self::expand_path(&self.config.database_path)).await {
            metadata.len()
        } else {
            total_entries * 1024 // Fallback estimate
        };
        
        // Calculate compression ratio by comparing actual vs estimated uncompressed size
        let estimated_uncompressed = total_entries * 2048; // Rough estimate of uncompressed size
        let compression_ratio = if estimated_uncompressed > 0 {
            storage_size_bytes as f32 / estimated_uncompressed as f32
        } else {
            1.0
        };
        
        // Simple cache hit rate simulation - in production this would track actual cache performance
        let cache_hit_rate = 0.85; // This would be calculated from actual cache metrics
        
        Ok(MemoryStats {
            total_entries,
            namespaces,
            storage_size_bytes,
            compression_ratio,
            cache_hit_rate,
        })
    }
    
    async fn export(&self, namespace: Option<&str>) -> Result<Vec<MemoryEntry>> {
        let conn = self.connection.lock().await;
        
        let (sql, params): (String, Vec<&str>) = match namespace {
            Some(ns) => (
                "SELECT id, key, value, namespace, created_at, updated_at, metadata 
                 FROM memory_entries WHERE namespace = ?1".to_string(),
                vec![ns]
            ),
            None => (
                "SELECT id, key, value, namespace, created_at, updated_at, metadata 
                 FROM memory_entries".to_string(),
                vec![]
            ),
        };
        
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            let metadata_json: String = row.get(6)?;
            let metadata: HashMap<String, String> = serde_json::from_str(&metadata_json)
                .unwrap_or_default();
            
            Ok(MemoryEntry {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                key: row.get(1)?,
                value: row.get(2)?,
                namespace: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                metadata,
            })
        })?;
        
        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }
        
        Ok(entries)
    }
    
    async fn import(&self, entries: &[MemoryEntry]) -> Result<u64> {
        let conn = self.connection.lock().await;
        let mut imported = 0;
        
        for entry in entries {
            let metadata_json = serde_json::to_string(&entry.metadata)?;
            
            conn.execute(
                "INSERT OR REPLACE INTO memory_entries 
                 (id, key, value, namespace, created_at, updated_at, metadata) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![
                    entry.id.to_string(),
                    entry.key,
                    entry.value,
                    entry.namespace,
                    entry.created_at,
                    entry.updated_at,
                    metadata_json
                ],
            )?;
            
            imported += 1;
        }
        
        Ok(imported)
    }
    
    async fn optimize(&self) -> Result<()> {
        let conn = self.connection.lock().await;
        
        // Run SQLite optimization commands
        conn.execute("VACUUM", [])?;
        conn.execute("ANALYZE", [])?;
        
        Ok(())
    }
}
*/ // End SQLite Backend Implementation

// JSON Backend Implementation
struct JsonBackend {
    config: crate::config::MemoryConfig,
    file_path: std::path::PathBuf,
    entries: RwLock<HashMap<String, MemoryEntry>>,
}

impl JsonBackend {
    async fn new(config: &crate::config::MemoryConfig) -> Result<Self> {
        let file_path = Self::expand_path(&config.database_path);
        
        // Create directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
        
        let mut backend = Self {
            config: config.clone(),
            file_path,
            entries: RwLock::new(HashMap::new()),
        };
        
        // Load existing data
        backend.load_from_file().await?;
        
        info!("JSON memory backend initialized: {}", backend.file_path.display());
        Ok(backend)
    }
    
    async fn load_from_file(&self) -> Result<()> {
        if !self.file_path.exists() {
            debug!("JSON file does not exist, starting with empty storage: {}", self.file_path.display());
            return Ok(());
        }
        
        let content = tokio::fs::read_to_string(&self.file_path).await
            .with_context(|| format!("Failed to read JSON file: {}", self.file_path.display()))?;
        
        if content.trim().is_empty() {
            debug!("JSON file is empty, starting with empty storage");
            return Ok(());
        }
        
        let entries: Vec<MemoryEntry> = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON file: {}", self.file_path.display()))?;
        
        let mut storage = self.entries.write().await;
        for entry in entries {
            let key = format!("{}:{}", entry.namespace, entry.key);
            storage.insert(key, entry);
        }
        
        info!("Loaded {} entries from JSON file", storage.len());
        Ok(())
    }
    
    async fn save_to_file(&self) -> Result<()> {
        let entries = self.entries.read().await;
        let entries_vec: Vec<&MemoryEntry> = entries.values().collect();
        
        let json = serde_json::to_string_pretty(&entries_vec)
            .with_context(|| "Failed to serialize entries to JSON")?;
        
        // Write atomically using a temporary file
        let temp_path = self.file_path.with_extension("tmp");
        tokio::fs::write(&temp_path, json).await
            .with_context(|| format!("Failed to write temporary file: {}", temp_path.display()))?;
        
        tokio::fs::rename(&temp_path, &self.file_path).await
            .with_context(|| format!("Failed to rename temporary file to: {}", self.file_path.display()))?;
        
        debug!("Saved {} entries to JSON file", entries_vec.len());
        Ok(())
    }
    
    fn expand_path(path: &str) -> std::path::PathBuf {
        if path.starts_with("~/") {
            if let Some(home) = directories::UserDirs::new() {
                return home.home_dir().join(&path[2..]);
            }
        }
        std::path::PathBuf::from(path)
    }
}

#[async_trait]
impl MemoryBackend for JsonBackend {
    async fn store(&self, entry: &MemoryEntry) -> Result<()> {
        let key = format!("{}:{}", entry.namespace, entry.key);
        
        {
            let mut entries = self.entries.write().await;
            entries.insert(key, entry.clone());
        }
        
        self.save_to_file().await?;
        debug!("Stored entry: {} in namespace: {}", entry.key, entry.namespace);
        Ok(())
    }
    
    async fn retrieve(&self, namespace: &str, key: &str) -> Result<Option<MemoryEntry>> {
        let lookup_key = format!("{}:{}", namespace, key);
        let entries = self.entries.read().await;
        Ok(entries.get(&lookup_key).cloned())
    }
    
    async fn query(&self, namespace: Option<&str>, pattern: &str) -> Result<Vec<MemoryEntry>> {
        let entries = self.entries.read().await;
        let mut results = Vec::new();
        
        for entry in entries.values() {
            // Check namespace filter
            if let Some(ns) = namespace {
                if entry.namespace != ns {
                    continue;
                }
            }
            
            // Check pattern match in key or value
            if entry.key.contains(pattern) || entry.value.contains(pattern) {
                results.push(entry.clone());
            }
        }
        
        Ok(results)
    }
    
    async fn delete(&self, namespace: &str, key: &str) -> Result<bool> {
        let lookup_key = format!("{}:{}", namespace, key);
        
        let removed = {
            let mut entries = self.entries.write().await;
            entries.remove(&lookup_key).is_some()
        };
        
        if removed {
            self.save_to_file().await?;
            debug!("Deleted entry: {} from namespace: {}", key, namespace);
        }
        
        Ok(removed)
    }
    
    async fn list_namespaces(&self) -> Result<Vec<String>> {
        let entries = self.entries.read().await;
        let mut namespaces: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        for entry in entries.values() {
            namespaces.insert(entry.namespace.clone());
        }
        
        let mut result: Vec<String> = namespaces.into_iter().collect();
        result.sort();
        Ok(result)
    }
    
    async fn get_stats(&self) -> Result<MemoryStats> {
        let entries = self.entries.read().await;
        let namespaces = self.list_namespaces().await?;
        
        // Calculate storage size (rough estimate)
        let storage_size = if self.file_path.exists() {
            tokio::fs::metadata(&self.file_path).await
                .map(|m| m.len())
                .unwrap_or(0)
        } else {
            0
        };
        
        Ok(MemoryStats {
            total_entries: entries.len() as u64,
            namespaces: namespaces.len() as u64,
            storage_size_bytes: storage_size,
            compression_ratio: 1.0, // JSON is not compressed
            cache_hit_rate: 1.0, // All data is in memory
        })
    }
    
    async fn export(&self, namespace: Option<&str>) -> Result<Vec<MemoryEntry>> {
        let entries = self.entries.read().await;
        let mut results = Vec::new();
        
        for entry in entries.values() {
            if let Some(ns) = namespace {
                if entry.namespace == ns {
                    results.push(entry.clone());
                }
            } else {
                results.push(entry.clone());
            }
        }
        
        Ok(results)
    }
    
    async fn import(&self, entries: &[MemoryEntry]) -> Result<u64> {
        let mut imported = 0u64;
        
        {
            let mut storage = self.entries.write().await;
            for entry in entries {
                let key = format!("{}:{}", entry.namespace, entry.key);
                storage.insert(key, entry.clone());
                imported += 1;
            }
        }
        
        self.save_to_file().await?;
        info!("Imported {} entries to JSON backend", imported);
        Ok(imported)
    }
    
    async fn optimize(&self) -> Result<()> {
        // For JSON backend, optimization means rewriting the file to remove any fragmentation
        self.save_to_file().await?;
        info!("JSON backend optimization complete");
        Ok(())
    }
}

// In-Memory Backend Implementation
struct InMemoryBackend {
    config: crate::config::MemoryConfig,
    entries: RwLock<HashMap<String, MemoryEntry>>,
    cache_stats: RwLock<CacheStats>,
}

#[derive(Debug, Default)]
struct CacheStats {
    hits: u64,
    misses: u64,
    total_requests: u64,
}

impl InMemoryBackend {
    async fn new(config: &crate::config::MemoryConfig) -> Result<Self> {
        info!("Initializing in-memory backend with capacity optimization");
        
        let backend = Self {
            config: config.clone(),
            entries: RwLock::new(HashMap::with_capacity(1000)), // Pre-allocate for efficiency
            cache_stats: RwLock::new(CacheStats::default()),
        };
        
        info!("In-memory backend initialized successfully");
        Ok(backend)
    }
    
    async fn update_cache_stats(&self, hit: bool) {
        let mut stats = self.cache_stats.write().await;
        stats.total_requests += 1;
        if hit {
            stats.hits += 1;
        } else {
            stats.misses += 1;
        }
    }
    
    async fn calculate_cache_hit_rate(&self) -> f32 {
        let stats = self.cache_stats.read().await;
        if stats.total_requests == 0 {
            0.0
        } else {
            stats.hits as f32 / stats.total_requests as f32
        }
    }
    
    fn estimate_entry_size(entry: &MemoryEntry) -> usize {
        // Rough estimate of memory usage per entry
        entry.key.len() + 
        entry.value.len() + 
        entry.namespace.len() + 
        entry.metadata.iter().map(|(k, v)| k.len() + v.len()).sum::<usize>() +
        64 // Overhead for struct fields and HashMap entry
    }
    
    fn simple_wildcard_match(text: &str, pattern_parts: &[&str]) -> bool {
        if pattern_parts.is_empty() {
            return true;
        }
        if pattern_parts.len() == 1 {
            return text.contains(pattern_parts[0]);
        }
        
        let mut text_pos = 0;
        for (i, part) in pattern_parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }
            
            if let Some(pos) = text[text_pos..].find(part) {
                text_pos += pos + part.len();
            } else {
                return false;
            }
        }
        true
    }
}

#[async_trait]
impl MemoryBackend for InMemoryBackend {
    async fn store(&self, entry: &MemoryEntry) -> Result<()> {
        let key = format!("{}:{}", entry.namespace, entry.key);
        
        let mut entries = self.entries.write().await;
        entries.insert(key, entry.clone());
        
        debug!("Stored entry: {} in namespace: {} (in-memory)", entry.key, entry.namespace);
        Ok(())
    }
    
    async fn retrieve(&self, namespace: &str, key: &str) -> Result<Option<MemoryEntry>> {
        let lookup_key = format!("{}:{}", namespace, key);
        let entries = self.entries.read().await;
        let result = entries.get(&lookup_key).cloned();
        
        // Update cache statistics
        self.update_cache_stats(result.is_some()).await;
        
        Ok(result)
    }
    
    async fn query(&self, namespace: Option<&str>, pattern: &str) -> Result<Vec<MemoryEntry>> {
        let entries = self.entries.read().await;
        let mut results = Vec::new();
        
        // Use simple pattern matching (avoiding regex dependency for now)
        let use_wildcard = pattern.contains('*') || pattern.contains('?');
        
        for entry in entries.values() {
            // Check namespace filter
            if let Some(ns) = namespace {
                if entry.namespace != ns {
                    continue;
                }
            }
            
            // Check pattern match with simple wildcard support
            let matches = if use_wildcard {
                // Simple wildcard matching
                let pattern_parts: Vec<&str> = pattern.split('*').collect();
                if pattern_parts.len() == 1 {
                    entry.key.contains(pattern) || entry.value.contains(pattern)
                } else {
                    // Basic wildcard support - check if all parts are present in order
                    let key_matches = Self::simple_wildcard_match(&entry.key, &pattern_parts);
                    let value_matches = Self::simple_wildcard_match(&entry.value, &pattern_parts);
                    key_matches || value_matches
                }
            } else {
                entry.key.contains(pattern) || entry.value.contains(pattern)
            };
            
            if matches {
                results.push(entry.clone());
            }
        }
        
        // Sort results by creation time (newest first)
        results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        debug!("Query returned {} results for pattern: {}", results.len(), pattern);
        Ok(results)
    }
    
    async fn delete(&self, namespace: &str, key: &str) -> Result<bool> {
        let lookup_key = format!("{}:{}", namespace, key);
        let mut entries = self.entries.write().await;
        let removed = entries.remove(&lookup_key).is_some();
        
        if removed {
            debug!("Deleted entry: {} from namespace: {} (in-memory)", key, namespace);
        }
        
        Ok(removed)
    }
    
    async fn list_namespaces(&self) -> Result<Vec<String>> {
        let entries = self.entries.read().await;
        let mut namespaces: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        for entry in entries.values() {
            namespaces.insert(entry.namespace.clone());
        }
        
        let mut result: Vec<String> = namespaces.into_iter().collect();
        result.sort();
        Ok(result)
    }
    
    async fn get_stats(&self) -> Result<MemoryStats> {
        let entries = self.entries.read().await;
        let namespaces = self.list_namespaces().await?;
        
        // Calculate total memory usage
        let storage_size_bytes: usize = entries.values()
            .map(|entry| Self::estimate_entry_size(entry))
            .sum();
        
        let cache_hit_rate = self.calculate_cache_hit_rate().await;
        
        Ok(MemoryStats {
            total_entries: entries.len() as u64,
            namespaces: namespaces.len() as u64,
            storage_size_bytes: storage_size_bytes as u64,
            compression_ratio: 1.0, // No compression in memory
            cache_hit_rate,
        })
    }
    
    async fn export(&self, namespace: Option<&str>) -> Result<Vec<MemoryEntry>> {
        let entries = self.entries.read().await;
        let mut results = Vec::new();
        
        for entry in entries.values() {
            if let Some(ns) = namespace {
                if entry.namespace == ns {
                    results.push(entry.clone());
                }
            } else {
                results.push(entry.clone());
            }
        }
        
        // Sort by creation time for consistent export order
        results.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        
        debug!("Exported {} entries from in-memory backend", results.len());
        Ok(results)
    }
    
    async fn import(&self, entries: &[MemoryEntry]) -> Result<u64> {
        let mut imported = 0u64;
        
        {
            let mut storage = self.entries.write().await;
            for entry in entries {
                let key = format!("{}:{}", entry.namespace, entry.key);
                storage.insert(key, entry.clone());
                imported += 1;
            }
        }
        
        info!("Imported {} entries to in-memory backend", imported);
        Ok(imported)
    }
    
    async fn optimize(&self) -> Result<()> {
        // For in-memory backend, optimization means shrinking the HashMap capacity
        // to fit the current data and clearing cache stats
        
        let current_len = {
            let entries = self.entries.read().await;
            entries.len()
        };
        
        // Create a new HashMap with optimal capacity
        let new_capacity = (current_len * 4 / 3).max(16); // 25% overhead, minimum 16
        
        {
            let mut entries = self.entries.write().await;
            let old_entries = std::mem::take(&mut *entries);
            *entries = HashMap::with_capacity(new_capacity);
            
            for (key, value) in old_entries {
                entries.insert(key, value);
            }
        }
        
        // Reset cache statistics
        {
            let mut stats = self.cache_stats.write().await;
            *stats = CacheStats::default();
        }
        
        info!("In-memory backend optimization complete - capacity: {}", new_capacity);
        Ok(())
    }
}