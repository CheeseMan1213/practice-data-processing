use web_sys::window;

pub fn get_api_host() -> String {
    // Try to get API host from environment variable via window object
    // This can be set during build time or via JavaScript
    if let Some(window) = window() {
        if let Ok(host) = window.location().hostname() {
            // If running on localhost, use localhost backend
            if host == "localhost" || host == "127.0.0.1" {
                return "http://localhost:3002".to_string();
            }
        }
    }
    
    // Default to Kubernetes service name for deployed environments
    "http://pdp-app-backend:3002".to_string()
}

