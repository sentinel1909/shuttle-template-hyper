// src/main.rs

// dependencies
use shuttle_hyper_template_lib::HyperService;
use shuttle_runtime::Error;

// main function
#[shuttle_runtime::main]
async fn main() -> Result<HyperService, Error> {
    
    Ok(HyperService {})
}
