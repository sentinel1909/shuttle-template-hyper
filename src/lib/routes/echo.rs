// src/lib/routes/echo.rs

// dependencies
use crate::state::AppState;
use crate::types::{HandlerResult, SvcReq, SvcResp};
use crate::utilities::{json_response_msg, parse_query_string, set_content_type_json};
use hyper::Response;

// echo handler function
pub fn handle_echo(request: SvcReq, _state: AppState) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Echo endpoint reached");
        let query = request.uri().query().unwrap_or("");
        let parsed = parse_query_string(query);

        let mut response: SvcResp = Response::new(json_response_msg(parsed));
        set_content_type_json(&mut response);

        Ok(response)
    })
}
