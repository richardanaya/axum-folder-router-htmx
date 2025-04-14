mod auth;

use auth::AuthService;
use std::sync::OnceLock;

// Global singleton
static AUTH_SERVICE: OnceLock<AuthService> = OnceLock::new();

pub fn auth_service() -> &'static AuthService {
    AUTH_SERVICE.get_or_init(|| AuthService::new())
}
