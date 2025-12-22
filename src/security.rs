use crate::config::Config;

/// Result of referer validation
#[derive(Debug, PartialEq)]
pub enum RefererValidation {
    /// Referer is valid and matches an allowed origin
    Valid,
    /// Referer header is missing
    Missing,
    /// Referer does not match any allowed origin
    Invalid,
}

/// Validate the Referer header against allowed origins
///
/// The referer URL must start with one of the allowed origins.
/// For example, if allowed_origins contains "https://example.com",
/// these referers would be valid:
/// - https://example.com
/// - https://example.com/
/// - https://example.com/page
///
/// But these would be invalid:
/// - https://example.com.evil.com (different domain)
/// - http://example.com (different scheme)
pub fn validate_referer(referer: Option<&str>, config: &Config) -> RefererValidation {
    let referer = match referer {
        Some(r) => r,
        None => return RefererValidation::Missing,
    };

    // Check if referer starts with any allowed origin
    for origin in &config.allowed_origins {
        // Ensure we match the full origin (prevent example.com.evil.com attacks)
        if referer == origin
            || referer.starts_with(&format!("{}/", origin))
            || referer.starts_with(&format!("{}?", origin))
            || referer.starts_with(&format!("{}#", origin))
        {
            return RefererValidation::Valid;
        }
    }

    RefererValidation::Invalid
}

/// Check if a request origin is allowed for CORS
pub fn is_origin_allowed(origin: Option<&str>, config: &Config) -> bool {
    match origin {
        Some(o) => config.allowed_origins.iter().any(|allowed| allowed == o),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            allowed_origins: vec![
                "https://example.com".to_string(),
                "https://cms.example.com".to_string(),
            ],
            block_no_referer: true,
            cache_control: "public, max-age=31536000, immutable".to_string(),
            r2_binding: "MEDIA_BUCKET".to_string(),
        }
    }

    #[test]
    fn test_valid_referer_exact_match() {
        let config = create_test_config();
        assert_eq!(
            validate_referer(Some("https://example.com"), &config),
            RefererValidation::Valid
        );
    }

    #[test]
    fn test_valid_referer_with_path() {
        let config = create_test_config();
        assert_eq!(
            validate_referer(Some("https://example.com/page"), &config),
            RefererValidation::Valid
        );
    }

    #[test]
    fn test_valid_referer_subdomain() {
        let config = create_test_config();
        assert_eq!(
            validate_referer(Some("https://cms.example.com/admin"), &config),
            RefererValidation::Valid
        );
    }

    #[test]
    fn test_missing_referer() {
        let config = create_test_config();
        assert_eq!(validate_referer(None, &config), RefererValidation::Missing);
    }

    #[test]
    fn test_invalid_referer_different_domain() {
        let config = create_test_config();
        assert_eq!(
            validate_referer(Some("https://evil.com"), &config),
            RefererValidation::Invalid
        );
    }

    #[test]
    fn test_invalid_referer_spoofed_subdomain() {
        let config = create_test_config();
        // This should NOT match - it's a different domain trying to look like example.com
        assert_eq!(
            validate_referer(Some("https://example.com.evil.com"), &config),
            RefererValidation::Invalid
        );
    }

    #[test]
    fn test_origin_allowed() {
        let config = create_test_config();
        assert!(is_origin_allowed(Some("https://example.com"), &config));
        assert!(is_origin_allowed(Some("https://cms.example.com"), &config));
        assert!(!is_origin_allowed(Some("https://evil.com"), &config));
        assert!(!is_origin_allowed(None, &config));
    }
}
