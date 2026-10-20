use log::{error, info, warn};

fn main() {
    // Initialize the logger so messages actually print to the console
    env_logger::init();

    // 1. Standard log (uses the module path as the default target, e.g., "custom_log_example")
    info!("Application started successfully.");

    // 2. Custom target: "database"
    info!(target: "database", "Successfully connected to the connection pool.");
    warn!(target: "database", "Slow query detected on table 'users' (120ms).");

    // 3. Custom target: "auth"
    error!(target: "auth", "Invalid password attempt for user: alice@example.com.");
}
