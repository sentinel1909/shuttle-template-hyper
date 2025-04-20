// src/main.rs

// dependencies
use shuttle_hyper_template_lib::HyperService;
use shuttle_hyper_template_lib::actors::AnalyticsActor;
use shuttle_hyper_template_lib::actors::PingCounterActor;
use shuttle_runtime::Error as HyperServiceError;

// main function
#[shuttle_runtime::main]
async fn main() -> Result<HyperService, HyperServiceError> {
    // start up the ping counter actor
    let (ping_tx, _handle) = PingCounterActor::start_ping_actor();
    let (analytics_tx, _handle) = AnalyticsActor::start_analytics_actor();

    // start up the service
    Ok(HyperService {
        analytics_tx,
        ping_tx,
    })
}
