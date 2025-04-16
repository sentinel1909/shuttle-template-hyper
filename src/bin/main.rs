// src/main.rs

// dependencies
use shuttle_hyper_template_lib::actors::PingCounterActor;
use shuttle_hyper_template_lib::HyperService;
use shuttle_runtime::Error;

// main function
#[shuttle_runtime::main]
async fn main() -> Result<HyperService, Error> {
    let (ping_tx, _handle) = PingCounterActor::start();

    Ok(HyperService { ping_tx })
}
