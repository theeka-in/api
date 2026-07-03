use std::sync::Arc;

use poem::Request;
use poem_openapi::{
    SecurityScheme,
    auth::{Basic, Bearer},
};
use sqlx::encode::IsNull::No;

use crate::{
    modules::auth::{AuthService, SessionDto},
    shared::guards::admin_guard,
};

#[derive(SecurityScheme)]
#[oai(ty = "basic", checker = "verify_admin")]
pub struct AdminGuard(());

pub async fn verify_admin(req: &Request, bearer: Basic) -> Option<()> {
    let admin_username = std::env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME not found");
    let admin_password = std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD not found");

    if admin_username == bearer.username && admin_password == bearer.password {
        return Some(());
    }

    None
}
