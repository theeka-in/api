use crate::errors::ErrorDto;
use poem::http::StatusCode;
use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};
use serde_json::json;

pub struct ErrorHandlerMiddleware;

impl<E: Endpoint> Middleware<E> for ErrorHandlerMiddleware {
    type Output = ErrorHandlerEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ErrorHandlerEndpoint(ep)
    }
}

pub struct ErrorHandlerEndpoint<E>(E);

impl<E: Endpoint> Endpoint for ErrorHandlerEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        match self.0.call(req).await {
            Ok(resp) => Ok(resp.into_response()),

            Err(err) => {
                let status = err.status();

                let json_body = (|| {
                    let normalized_error = err
                        .to_string()
                        // .to_lowercase()
                        .replace("parse request payload error: ", "")
                        .replace("\"", "`");
                    json!({
                        "message": normalized_error,
                    })
                    .to_string()
                })();

                Ok(Response::builder()
                    .status(status)
                    .content_type("application/json")
                    .body(json_body))
            }
        }
    }
}
