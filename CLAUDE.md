# CLAUDE.md

## Project Overview

Cloudflare Workers で動作する R2 バケットのメディアプロキシ。リファラー検証によるホットリンク防止とキャッシュ最適化を提供する。

## Build & Test Commands

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Local development (requires wrangler.toml)
wrangler dev

# Deploy
wrangler deploy
```

## Project Structure

```
src/
├── lib.rs       # Entry point, #[event(fetch)] handler
├── config.rs    # Environment variable parsing
├── handler.rs   # Request handling, R2 access
└── security.rs  # Referer validation logic
```

## Code Style

- Rust 2021 edition
- Error handling: Use `worker::Result` and `?` operator
- Prefer `worker` crate APIs over raw JS bindings

## Configuration

- Environment variables are used for configuration (not config files)
- `wrangler.toml` is gitignored; use `wrangler.example.toml` as template
- Required env: `ALLOWED_ORIGINS`
- Optional env: `BLOCK_NO_REFERER`, `CACHE_CONTROL`

## Testing

- Unit tests are in each module (`#[cfg(test)]`)
- Integration tests require `wrangler dev` with R2 bucket binding
