use rusqlite::{Connection, Result};
use std::path::Path;

pub fn initialize(path: &Path) -> Result<()> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "PRAGMA foreign_keys=ON;
         PRAGMA journal_mode=WAL;
         CREATE TABLE IF NOT EXISTS settings(
           key TEXT PRIMARY KEY,
           value TEXT NOT NULL
         );
         CREATE TABLE IF NOT EXISTS hardware_profiles(
           id INTEGER PRIMARY KEY AUTOINCREMENT,
           payload_json TEXT NOT NULL,
           created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         CREATE TABLE IF NOT EXISTS models(
           id TEXT PRIMARY KEY,
           name TEXT NOT NULL,
           provider TEXT NOT NULL,
           task TEXT,
           metadata_json TEXT,
           created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         CREATE TABLE IF NOT EXISTS model_variants(
           id TEXT NOT NULL,
           model_id TEXT NOT NULL,
           format TEXT NOT NULL,
           quantization TEXT,
           file_size INTEGER NOT NULL DEFAULT 0,
           estimated_ram INTEGER,
           estimated_vram INTEGER,
           PRIMARY KEY(id, model_id),
           FOREIGN KEY(model_id) REFERENCES models(id) ON DELETE CASCADE
         );
         CREATE TABLE IF NOT EXISTS installed_models(
           id INTEGER PRIMARY KEY AUTOINCREMENT,
           model_id TEXT NOT NULL,
           variant_id TEXT,
           path TEXT NOT NULL UNIQUE,
           status TEXT NOT NULL,
           last_used_at TEXT,
           created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         CREATE TABLE IF NOT EXISTS downloads(
           id TEXT PRIMARY KEY,
           model_id TEXT NOT NULL,
           url TEXT NOT NULL,
           destination TEXT NOT NULL,
           status TEXT NOT NULL,
           progress REAL NOT NULL DEFAULT 0,
           downloaded_bytes INTEGER NOT NULL DEFAULT 0,
           total_bytes INTEGER,
           error TEXT,
           updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         CREATE TABLE IF NOT EXISTS chat_sessions(
           id TEXT PRIMARY KEY,
           model_id TEXT,
           title TEXT NOT NULL,
           created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
           updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         CREATE TABLE IF NOT EXISTS chat_messages(
           id TEXT PRIMARY KEY,
           session_id TEXT NOT NULL,
           role TEXT NOT NULL,
           content TEXT NOT NULL,
           created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
           FOREIGN KEY(session_id) REFERENCES chat_sessions(id) ON DELETE CASCADE
         );
         CREATE TABLE IF NOT EXISTS model_benchmarks(
           id INTEGER PRIMARY KEY AUTOINCREMENT,
           model_id TEXT NOT NULL,
           tokens_per_second REAL,
           prompt_tokens_per_second REAL,
           load_ms INTEGER,
           peak_ram INTEGER,
           peak_vram INTEGER,
           created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );",
    )?;
    Ok(())
}
