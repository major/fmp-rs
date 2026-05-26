# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0]

### Breaking changes

- Flat command form removed. Use `fmp-agent <group> <command>`.
- See the README migration note for the command tree changes.

### Added

- Top-level aliases: `quote`, `historical`, `profile`, `earnings`.
- `schema_version: 2` for `fmp-agent schema`.

### Removed

- Legacy flat domain-prefixed commands.

## [0.3.0] - 2026-05-21

### <!-- 0 -->🚀 Features

- *(cli)* Add confirmed news endpoints

### <!-- 1 -->🐛 Bug Fixes

- *(cli)* Show help for bare command
- *(cli)* Handle env-backed bare help

### <!-- 6 -->🧪 Testing

- *(cli)* Print bare command failure details
- *(cli)* Cover binary parse paths

### <!-- 7 -->⚙️ Miscellaneous

- *(lint)* Deny unused group in Cargo.toml and add cargo-machete


## [0.2.0] - 2026-05-21

### <!-- 0 -->🚀 Features

- *(cli)* Add confirmed FMP endpoints

### <!-- 2 -->📚 Documentation

- Add crate badges

### <!-- 7 -->⚙️ Miscellaneous

- Release v0.1.0


## [0.1.0] - 2026-05-21

### <!-- 7 -->⚙️ Miscellaneous

- Import project
- Add renovate config
- Rename crate to rusty-fmp
