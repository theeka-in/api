use std::sync::Arc;

use poem::Request;
use poem_openapi::{SecurityScheme, auth::Bearer};

use crate::modules::auth::{AccountDto, AuthService, SessionDto};

#[derive(SecurityScheme)]
#[oai(ty = "bearer", checker = "verify_token")]
pub struct BearerAuth(pub (AccountDto, SessionDto));

async fn verify_token(req: &Request, bearer: Bearer) -> Option<(AccountDto, SessionDto)> {
    let auth_service = req.data::<Arc<AuthService>>()?;

    auth_service
        .find_account_and_session_by_token(bearer.token)
        .await
        .ok()
}
