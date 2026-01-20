use process_cpu_auto::{ServiceRunner, ServiceError};
use std::env;

fn main() -> Result<(), ServiceError> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let config_path = if args.len() > 1 {
        &args[1]
    } else {
        "config.toml"
    };

    println!("Windows Process CPU Affinity Auto Service");
    println!("==========================================");
    println!();

    // Create and run service
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
