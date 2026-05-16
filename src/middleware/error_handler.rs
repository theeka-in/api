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
                        .to_lowercase()
                        .replace("parse request payload error: ", "")
                        .replace("\"", "`");

                    if normalized_error.starts_with("failed to parse") {
                        let (_, parse_details) = normalized_error.split_once(": ").unwrap();

                        let detail_parts: Vec<&str> = parse_details.split(". ").collect();

                        let field_name = detail_parts[0]
                            .split_once("`")
                            .unwrap()
                            .1
                            .split_once("`")
                            .unwrap()
                            .0;

                        let expected_type = detail_parts[1];

                        return json!({
                            "error": {
                                "field": field_name,
                                "should_have": expected_type,
                            }
                        })
                        .to_string();
                    }

                    if normalized_error.starts_with("expected input type") {
                        let (type_mismatch, _) = normalized_error.split_once(". ").unwrap();

                        let type_summary = type_mismatch
                            .replace("expected input type ", "")
                            .replace("found ", "")
                            .replace("`", "");

                        let (expected_type, actual_type) = type_summary.split_once(", ").unwrap();

                        return json!({
                            "error": {
                                "expected": expected_type,
                                "found": actual_type,
                            }
                        })
                        .to_string();
                    }

                    json!({
                        "error": {
                            "message": normalized_error,
                        }
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
