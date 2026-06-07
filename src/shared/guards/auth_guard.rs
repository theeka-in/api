use std::sync::Arc;

use poem::Request;
use poem_openapi::{SecurityScheme, auth::Bearer};

use crate::modules::auth::{AuthService, SessionDto};

#[derive(SecurityScheme)]
#[oai(ty = "bearer", checker = "verify_auth")]
pub struct AuthGuard(pub (SessionDto));

pub async fn verify_auth(req: &Request, bearer: Bearer) -> Option<(SessionDto)> {
    let auth_service = req.data::<Arc<AuthService>>()?;

    auth_service.get_session(bearer.token).await.ok()
}
