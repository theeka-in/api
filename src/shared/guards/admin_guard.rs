use poem::Request;
use poem_openapi::{auth::Basic, SecurityScheme};

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
