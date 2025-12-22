# Media Proxy Worker

A Cloudflare Worker that protects R2 bucket access through referer validation and provides cache optimization.

## Architecture

```
Allowed Sites (example.com, cms.example.com)
     ↓ <img src="https://media.example.com/image.webp">

r2.example.com (Workers Proxy)
     ├─ Referer validation
     ├─ Cache-Control headers
     └─ CORS headers
     ↓
R2 Bucket (private, binding access only)
```

## Features

- Referer validation to prevent hotlinking
- Configurable allowed origins
- Long-term cache headers for immutable assets
- CORS support for allowed origins
- R2 bucket integration via binding

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/) CLI
- Cloudflare account with Workers and R2 enabled

## Setup

### 1. Clone and configure

```bash
# Clone the repository
git clone https://github.com/akito19/media-proxy-worker.git
cd media-proxy-worker

# Copy configuration files
cp wrangler.example.toml wrangler.toml
cp .dev.vars.example .dev.vars
```

### 2. Edit wrangler.toml

Update the following values in `wrangler.toml`:

```toml
[[r2_buckets]]
binding = "MEDIA_BUCKET"
bucket_name = "your-actual-bucket-name"  # Your R2 bucket name

[routes]
pattern = "r2.your-domain.com/*"  # Your domain
zone_name = "your-domain.com"     # Your zone
```

### 3. Configure environment variables

#### For local development

Edit `.dev.vars`:

```bash
ALLOWED_ORIGINS=https://localhost:3000,https://your-domain.com
BLOCK_NO_REFERER=true
CACHE_CONTROL=public, max-age=31536000, immutable
```

#### For production

Set environment variables via Cloudflare Dashboard or wrangler CLI:

```bash
# Using wrangler CLI
wrangler secret put ALLOWED_ORIGINS
# Enter: https://your-domain.com,https://www.your-domain.com

# Optional
wrangler secret put BLOCK_NO_REFERER
wrangler secret put CACHE_CONTROL
```

Or via Cloudflare Dashboard:
1. Go to Workers & Pages
2. Select your worker
3. Settings > Variables
4. Add environment variables

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `ALLOWED_ORIGINS` | Yes | - | Comma-separated list of allowed origins |
| `BLOCK_NO_REFERER` | No | `true` | Block requests without Referer header |
| `CACHE_CONTROL` | No | `public, max-age=31536000, immutable` | Cache-Control header value |

## Development

```bash
# Run locally
wrangler dev

# Build
wrangler build

# Deploy
wrangler deploy
```

## API

### GET /{path}

Retrieve a file from the R2 bucket.

**Request Headers**

| Header | Required | Description |
|--------|----------|-------------|
| Referer | Conditional | Required if `BLOCK_NO_REFERER=true` |
| Origin | No | For CORS support |

**Response Codes**

| Status | Description |
|--------|-------------|
| 200 | File found and access allowed |
| 403 | Invalid or missing referer |
| 404 | File not found |
| 405 | Method not allowed (non-GET) |

**Response Headers**

| Header | Value |
|--------|-------|
| Content-Type | From R2 object metadata |
| Cache-Control | `public, max-age=31536000, immutable` (configurable) |
| ETag | From R2 object |
| Access-Control-Allow-Origin | Request origin (if allowed) |

## Security Notes

Referer validation provides protection against:
- Hotlinking from unauthorized websites
- Casual unauthorized access

It does **not** protect against:
- Programmatic access with spoofed headers
- Determined attackers

For highly sensitive content, consider implementing signed URLs.

## License

MIT
