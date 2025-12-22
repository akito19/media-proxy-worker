use worker::*;

mod config;
mod handler;
mod security;

use config::Config;
use handler::handle_request;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Set panic hook for better error messages in console
    console_error_panic_hook::set_once();

    // Load configuration from environment variables
    let config = match Config::from_env(&env) {
        Ok(c) => c,
        Err(e) => {
            console_error!("Configuration error: {:?}", e);
            return Response::error("Internal Server Error", 500);
        }
    };

    // Handle the request
    handle_request(req, env, &config).await
}
