use process_cpu_auto::{service, ServiceRunner, ServiceError};
use std::env;

fn main() -> Result<(), ServiceError> {
    let args: Vec<String> = env::args().collect();

    // Check if running in service mode
    if args.contains(&"--service".to_string()) {
        // Running as Windows Service
        return service::run_service();
    }

    // Running in CLI mode
    println!("Windows Process CPU Affinity Auto Service");
    println!("==========================================");
    println!();

    // Check administrator privileges first
    process_cpu_auto::utils::privilege::require_administrator()?;

    println!("✓ Running with Administrator privileges");
    println!();

    // Parse command line arguments
    let config_path = if args.len() > 1 && !args[1].starts_with("--") {
        &args[1]
    } else {
        "config.toml"
    };

    // Create and run service in CLI mode
    let mut runner = ServiceRunner::new(config_path)?;

    // Run the service
    match runner.run() {
        Ok(_) => {
            println!("Service stopped normally");
            Ok(())
        }
        Err(e) => {
            eprintln!("Service error: {}", e);
            Err(e)
        }
    }
}
