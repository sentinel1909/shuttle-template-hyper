// src/lib/routes/echo.rs

// dependencies
use crate::state::AppState;
use crate::types::HandlerResult;
use crate::utilities::{json_response_msg, parse_query_string, set_content_type_json};
use hyper::{Request, Response, body::Incoming};

// echo handler function
pub fn handle_echo(request: Request<Incoming>, _state: AppState) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Echo endpoint reached");
        let query = request.uri().query().unwrap_or("");
        let parsed = parse_query_string(query);

        let mut response = Response::new(json_response_msg(parsed));
        set_content_type_json(&mut response);

        Ok(response)
    })
}
