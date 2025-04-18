// src/main.rs

// dependencies
use shuttle_hyper_template_lib::actors::PingCounterActor;
use shuttle_hyper_template_lib::HyperService;
use shuttle_runtime::Error as HyperServiceError;

// main function
#[shuttle_runtime::main]
async fn main() -> Result<HyperService, HyperServiceError> {
    // start up the ping counter actor
    let (tx, _handle) = PingCounterActor::start();

    // start up the service
    Ok(HyperService { tx })
}
