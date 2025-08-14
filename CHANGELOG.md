# Changelog

## [0.4.0] - 2025-08-13
### Added
- **Persistent Catalog Storage (JSON)**  
  - FerriteDB now saves the table catalog (schemas) to `catalog.json` whenever tables are created or dropped.
  - The catalog is automatically loaded at startup, so table definitions survive restarts without needing to recreate them manually.
  - Uses a human-readable JSON format for easy inspection and debugging.

### Changed
- Updated internal `Catalog` struct to derive `Serialize` and `Deserialize` so schemas can be stored on disk.
- `main.rs` now loads the catalog from disk if available; falls back to an empty catalog otherwise.

### Developer Notes
- The catalog file is stored in the same directory as your database `.tbl` files.
- JSON was chosen over binary encoding to make debugging and manual editing easier.
- Future schema changes may require migration logic, but JSON format makes this simpler.
