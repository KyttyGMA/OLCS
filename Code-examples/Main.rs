mod organism;
mod orchestrator;
mod cooperation;

use organism::Organism;
use cooperation::CooperationManager;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a list of organisms with different queries
    let organisms = vec![
        Organism::new(1, String::from("Rust programming language")),
        Organism::new(2, String::from("Tokio async Rust")),
        Organism::new(3, String::from("DuckDuckGo API")),
    ];

    // Create a cooperation manager and assign the organisms to it
    let mut manager = CooperationManager::new(organisms);

    // Run the cooperation mechanism
    manager.handle_cooperation().await?;

    Ok(())
}
