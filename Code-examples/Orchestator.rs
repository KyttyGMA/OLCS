use tokio::sync::mpsc;
use std::error::Error;
use tokio::task;
use std::sync::Arc;

// Define the data structure that holds input and output data for each organism
#[derive(Debug, Clone)]
struct Data {
    input: String,
    output: Option<String>,
}

// Basic Organism structure
struct Organism {
    id: u32,
    data: Data,
}

impl Organism {
    // Create a new organism
    fn new(id: u32, input: String) -> Self {
        Organism {
            id,
            data: Data {
                input,
                output: None,
            },
        }
    }

    // Process data (for this example, just convert input to uppercase)
    async fn process_data(&mut self) {
        println!("Organism {} is processing data...", self.id);
        self.data.output = Some(self.data.input.to_uppercase());
    }

    // Infer result
    fn infer(&self) -> String {
        if let Some(ref output) = self.data.output {
            format!("Organism {} inference: {}", self.id, output)
        } else {
            String::from("No data processed yet.")
        }
    }
}

// Define the Orchestrator, which manages multiple organisms
struct Orchestrator {
    organisms: Vec<Arc<tokio::sync::Mutex<Organism>>>,  // List of organisms
}

impl Orchestrator {
    // Create a new orchestrator with a set of organisms
    fn new(organisms: Vec<Arc<tokio::sync::Mutex<Organism>>>) -> Self {
        Orchestrator { organisms }
    }

    // Assign tasks to organisms and coordinate their processing
    async fn orchestrate(&self) -> Result<(), Box<dyn Error>> {
        let (tx, mut rx) = mpsc::channel(32);  // Channel for communication
        let mut tasks = vec![];

        // For each organism, spawn a task to process the data and send results to the orchestrator
        for organism in self.organisms.clone() {
            let tx_clone = tx.clone();
            let organism_clone = organism.clone();

            let task = task::spawn(async move {
                let mut org = organism_clone.lock().await;
                org.process_data().await;  // Each organism processes its data
                let result = org.infer();  // Get the inference result
                tx_clone.send(result).await.unwrap();  // Send result to orchestrator
            });
            tasks.push(task);
        }

        // Await all tasks
        for task in tasks {
            task.await?;
        }

        // Collect all results from organisms
        while let Some(result) = rx.recv().await {
            println!("Orchestrator received: {}", result);
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a few organisms with different inputs
    let organisms = vec![
        Arc::new(tokio::sync::Mutex::new(Organism::new(1, "task one".to_string()))),
        Arc::new(tokio::sync::Mutex::new(Organism::new(2, "task two".to_string()))),
        Arc::new(tokio::sync::Mutex::new(Organism::new(3, "task three".to_string()))),
    ];

    // Create an orchestrator that manages these organisms
    let orchestrator = Orchestrator::new(organisms);

    // Start orchestration: assign tasks and gather results
    orchestrator.orchestrate().await?;

    Ok(())
}
