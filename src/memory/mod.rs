use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use uuid::Uuid;

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
            "sqlite" => Box::new(SqliteBackend::new(&config.memory).await?),
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
            connected: true, // TODO: Implement proper connection checking
            last_sync: Some(self.current_timestamp()),
            pending_operations: 0, // TODO: Implement pending operations tracking
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
    
    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

// SQLite Backend Implementation
use rusqlite::{Connection, params};
use tokio::sync::Mutex;

struct SqliteBackend {
    connection: Mutex<Connection>,
    config: crate::config::MemoryConfig,
}

impl SqliteBackend {
    async fn new(config: &crate::config::MemoryConfig) -> Result<Self> {
        let db_path = Self::expand_path(&config.database_path);
        
        // Create directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        let conn = Connection::open(&db_path)
            .with_context(|| format!("Failed to open SQLite database: {}", db_path.display()))?;
        
        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS memory_entries (
                id TEXT PRIMARY KEY,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                namespace TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                metadata TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_namespace_key ON memory_entries(namespace, key)",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_namespace ON memory_entries(namespace)",
            [],
        )?;
        
        info!("SQLite memory backend initialized: {}", db_path.display());
        
        Ok(Self {
            connection: Mutex::new(conn),
            config: config.clone(),
        })
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

#[async_trait::async_trait]
impl MemoryBackend for SqliteBackend {
    async fn store(&self, entry: &MemoryEntry) -> Result<()> {
        let conn = self.connection.lock().await;
        let metadata_json = serde_json::to_string(&entry.metadata)?;
        
        conn.execute(
            "INSERT OR REPLACE INTO memory_entries 
             (id, key, value, namespace, created_at, updated_at, metadata) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                entry.id.to_string(),
                entry.key,
                entry.value,
                entry.namespace,
                entry.created_at,
                entry.updated_at,
                metadata_json
            ],
        )?;
        
        Ok(())
    }
    
    async fn retrieve(&self, namespace: &str, key: &str) -> Result<Option<MemoryEntry>> {
        let conn = self.connection.lock().await;
        
        let mut stmt = conn.prepare(
            "SELECT id, key, value, namespace, created_at, updated_at, metadata 
             FROM memory_entries WHERE namespace = ?1 AND key = ?2"
        )?;
        
        let mut rows = stmt.query_map(params![namespace, key], |row| {
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
            params![namespace, key],
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
        
        // TODO: Implement proper storage size calculation
        let storage_size_bytes = total_entries * 1024; // Rough estimate
        
        Ok(MemoryStats {
            total_entries,
            namespaces,
            storage_size_bytes,
            compression_ratio: 1.0, // TODO: Implement compression tracking
            cache_hit_rate: 0.85,   // TODO: Implement cache hit rate tracking
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
                params![
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

// JSON Backend Implementation (simplified)
struct JsonBackend {
    config: crate::config::MemoryConfig,
}

impl JsonBackend {
    async fn new(config: &crate::config::MemoryConfig) -> Result<Self> {
        // TODO: Implement JSON file-based backend
        Ok(Self {
            config: config.clone(),
        })
    }
}

#[async_trait::async_trait]
impl MemoryBackend for JsonBackend {
    async fn store(&self, _entry: &MemoryEntry) -> Result<()> {
        // TODO: Implement JSON backend
        Ok(())
    }
    
    async fn retrieve(&self, _namespace: &str, _key: &str) -> Result<Option<MemoryEntry>> {
        // TODO: Implement JSON backend
        Ok(None)
    }
    
    async fn query(&self, _namespace: Option<&str>, _pattern: &str) -> Result<Vec<MemoryEntry>> {
        // TODO: Implement JSON backend
        Ok(vec![])
    }
    
    async fn delete(&self, _namespace: &str, _key: &str) -> Result<bool> {
        // TODO: Implement JSON backend
        Ok(false)
    }
    
    async fn list_namespaces(&self) -> Result<Vec<String>> {
        // TODO: Implement JSON backend
        Ok(vec![])
    }
    
    async fn get_stats(&self) -> Result<MemoryStats> {
        // TODO: Implement JSON backend
        Ok(MemoryStats {
            total_entries: 0,
            namespaces: 0,
            storage_size_bytes: 0,
            compression_ratio: 1.0,
            cache_hit_rate: 0.0,
        })
    }
    
    async fn export(&self, _namespace: Option<&str>) -> Result<Vec<MemoryEntry>> {
        // TODO: Implement JSON backend
        Ok(vec![])
    }
    
    async fn import(&self, _entries: &[MemoryEntry]) -> Result<u64> {
        // TODO: Implement JSON backend
        Ok(0)
    }
    
    async fn optimize(&self) -> Result<()> {
        // TODO: Implement JSON backend
        Ok(())
    }
}

// In-Memory Backend Implementation (simplified)
struct InMemoryBackend {
    config: crate::config::MemoryConfig,
}

impl InMemoryBackend {
    async fn new(config: &crate::config::MemoryConfig) -> Result<Self> {
        // TODO: Implement in-memory backend
        Ok(Self {
            config: config.clone(),
        })
    }
}

#[async_trait::async_trait]
impl MemoryBackend for InMemoryBackend {
    async fn store(&self, _entry: &MemoryEntry) -> Result<()> {
        // TODO: Implement in-memory backend
        Ok(())
    }
    
    async fn retrieve(&self, _namespace: &str, _key: &str) -> Result<Option<MemoryEntry>> {
        // TODO: Implement in-memory backend
        Ok(None)
    }
    
    async fn query(&self, _namespace: Option<&str>, _pattern: &str) -> Result<Vec<MemoryEntry>> {
        // TODO: Implement in-memory backend
        Ok(vec![])
    }
    
    async fn delete(&self, _namespace: &str, _key: &str) -> Result<bool> {
        // TODO: Implement in-memory backend
        Ok(false)
    }
    
    async fn list_namespaces(&self) -> Result<Vec<String>> {
        // TODO: Implement in-memory backend
        Ok(vec![])
    }
    
    async fn get_stats(&self) -> Result<MemoryStats> {
        // TODO: Implement in-memory backend
        Ok(MemoryStats {
            total_entries: 0,
            namespaces: 0,
            storage_size_bytes: 0,
            compression_ratio: 1.0,
            cache_hit_rate: 0.0,
        })
    }
    
    async fn export(&self, _namespace: Option<&str>) -> Result<Vec<MemoryEntry>> {
        // TODO: Implement in-memory backend
        Ok(vec![])
    }
    
    async fn import(&self, _entries: &[MemoryEntry]) -> Result<u64> {
        // TODO: Implement in-memory backend
        Ok(0)
    }
    
    async fn optimize(&self) -> Result<()> {
        // TODO: Implement in-memory backend
        Ok(())
    }
}