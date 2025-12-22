use worker::{Env, Error};

/// Configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// List of allowed origins for referer validation
    pub allowed_origins: Vec<String>,
    /// Whether to block requests without Referer header
    pub block_no_referer: bool,
    /// Cache-Control header value
    pub cache_control: String,
    /// R2 bucket binding name
    pub r2_binding: String,
}

impl Config {
    /// Default Cache-Control header for immutable assets
    const DEFAULT_CACHE_CONTROL: &'static str = "public, max-age=31536000, immutable";
    /// Default R2 bucket binding name
    const DEFAULT_R2_BINDING: &'static str = "MEDIA_BUCKET";

    /// Load configuration from environment variables
    ///
    /// Required:
    /// - ALLOWED_ORIGINS: Comma-separated list of allowed origins
    ///
    /// Optional:
    /// - BLOCK_NO_REFERER: "true" or "false" (default: true)
    /// - CACHE_CONTROL: Custom Cache-Control header value
    /// - R2_BUCKET_BINDING: R2 bucket binding name (default: "MEDIA_BUCKET")
    pub fn from_env(env: &Env) -> Result<Self, Error> {
        // Parse ALLOWED_ORIGINS (required)
        let origins_str = env
            .var("ALLOWED_ORIGINS")
            .map_err(|_| {
                Error::RustError("ALLOWED_ORIGINS environment variable is required".into())
            })?
            .to_string();

        let allowed_origins: Vec<String> = origins_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if allowed_origins.is_empty() {
            return Err(Error::RustError(
                "ALLOWED_ORIGINS must contain at least one origin".into(),
            ));
        }

        // Parse BLOCK_NO_REFERER (optional, default: true)
        let block_no_referer = env
            .var("BLOCK_NO_REFERER")
            .map(|v| v.to_string().to_lowercase() != "false")
            .unwrap_or(true);

        // Parse CACHE_CONTROL (optional)
        let cache_control = env
            .var("CACHE_CONTROL")
            .map(|v| v.to_string())
            .unwrap_or_else(|_| Self::DEFAULT_CACHE_CONTROL.to_string());

        // Parse R2_BUCKET_BINDING (optional)
        let r2_binding = env
            .var("R2_BUCKET_BINDING")
            .map(|v| v.to_string())
            .unwrap_or_else(|_| Self::DEFAULT_R2_BINDING.to_string());

        Ok(Self {
            allowed_origins,
            block_no_referer,
            cache_control,
            r2_binding,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cache_control() {
        assert_eq!(
            Config::DEFAULT_CACHE_CONTROL,
            "public, max-age=31536000, immutable"
        );
    }
}
