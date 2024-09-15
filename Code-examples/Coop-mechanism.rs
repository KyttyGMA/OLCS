use crate::organism::Organism;
use std::sync::{Arc, Mutex};
use tokio::task;
use std::error::Error;

// Define the structure of the cooperation manager
pub struct CooperationManager {
    organisms: Vec<Organism>,
}

impl CooperationManager {
    // Create a new cooperation manager with a list of organisms
    pub fn new(organisms: Vec<Organism>) -> Self {
        CooperationManager { organisms }
    }

    // Function to handle cooperation between organisms
    pub async fn handle_cooperation(&mut self) -> Result<(), Box<dyn Error>> {
        let shared_data = Arc::new(Mutex::new(Vec::new()));

        // Create tasks for each organism to perform a search
        let mut tasks = vec![];
        for organism in &mut self.organisms {
            let shared_data_clone = Arc::clone(&shared_data);
            let org_id = organism.id;
            let query = organism.query.clone();

            let task = task::spawn(async move {
                // Each organism performs its search
                if let Ok(_) = organism.search_and_process().await {
                    let result = organism.infer();
                    println!("Organism {} finished: {}", org_id, result);

                    // Store the result in shared data
                    let mut data = shared_data_clone.lock().unwrap();
                    data.push((org_id, result));
                }
            });

            tasks.push(task);
        }

        // Await completion of all tasks
        for task in tasks {
            task.await?;
        }

        // Display the results collected from all organisms
        let final_data = shared_data.lock().unwrap();
        println!("\nCollected Data from all organisms:");
        for (id, result) in final_data.iter() {
            println!("Organism {} result: {}", id, result);
        }

        Ok(())
    }
}
