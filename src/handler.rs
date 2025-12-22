use worker::{Env, Request, Response, Result};

use crate::config::Config;
use crate::security::{is_origin_allowed, validate_referer, RefererValidation};

/// Handle incoming requests
pub async fn handle_request(req: Request, env: Env, config: &Config) -> Result<Response> {
    // Only allow GET requests
    if req.method() != worker::Method::Get {
        return Response::error("Method Not Allowed", 405);
    }

    // Extract headers
    let headers = req.headers();
    let referer = headers.get("Referer").ok().flatten();
    let origin = headers.get("Origin").ok().flatten();

    // Validate referer
    match validate_referer(referer.as_deref(), config) {
        RefererValidation::Valid => {}
        RefererValidation::Missing => {
            if config.block_no_referer {
                return Response::error("Forbidden: Missing Referer", 403);
            }
        }
        RefererValidation::Invalid => {
            return Response::error("Forbidden: Invalid Referer", 403);
        }
    }

    // Get the path from the URL (remove leading slash)
    let url = req.url()?;
    let path = url.path();
    let key = path.trim_start_matches('/');

    if key.is_empty() {
        return Response::error("Not Found", 404);
    }

    // Get R2 bucket binding
    let bucket = env.bucket(&config.r2_binding)?;

    // Fetch object from R2
    let object = match bucket.get(key).execute().await? {
        Some(obj) => obj,
        None => return Response::error("Not Found", 404),
    };

    // Get object body
    let body = match object.body() {
        Some(b) => b,
        None => return Response::error("Not Found", 404),
    };

    // Build response with appropriate headers
    let mut response = Response::from_stream(body.stream()?)?;

    // Set Content-Type from R2 metadata
    if let Some(content_type) = object.http_metadata().content_type {
        response.headers_mut().set("Content-Type", &content_type)?;
    }

    // Set Cache-Control
    response
        .headers_mut()
        .set("Cache-Control", &config.cache_control)?;

    // Set ETag if available
    let etag = object.http_etag();
    if !etag.is_empty() {
        response.headers_mut().set("ETag", &etag)?;
    }

    // Set CORS headers if Origin is allowed
    if let Some(ref origin_value) = origin {
        if is_origin_allowed(Some(origin_value), config) {
            response
                .headers_mut()
                .set("Access-Control-Allow-Origin", origin_value)?;
            response
                .headers_mut()
                .set("Access-Control-Allow-Methods", "GET")?;
        }
    }

    Ok(response)
}
