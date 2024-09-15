use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::io::{self, Write};

// Structure to deserialize the DuckDuckGo API response
#[derive(Deserialize, Debug)]
struct DuckDuckGoResult {
    Heading: String,
    AbstractText: String,
    AbstractURL: String,
    RelatedTopics: Vec<RelatedTopic>,
}

// Structure for related topics returned by DuckDuckGo
#[derive(Deserialize, Debug)]
struct RelatedTopic {
    Text: String,
    FirstURL: String,
}

// Structure to store the input and output data of the organism
struct Data {
    input: String,
    output: Option<String>,
}

// Basic structure of an Organism
struct Organism {
    id: u32,
    data: Data,
}

impl Organism {
    // Create a new organism with a unique ID and input data
    fn new(id: u32, input: String) -> Self {
        Organism {
            id,
            data: Data {
                input,
                output: None,
            },
        }
    }

    // Method to process data, in this case, it performs a search using the DuckDuckGo API
    async fn process_data(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Organism {} is processing data...", self.id);

        // Create an HTTP client
        let client = Client::new();

        // Construct the DuckDuckGo API URL using the organism's input data (search query)
        let url = format!("https://api.duckduckgo.com/?q={}&format=json&no_redirect=1", self.data.input);

        // Perform the HTTP request to DuckDuckGo and deserialize the JSON response into DuckDuckGoResult
        let response = client.get(&url).send().await?.json::<DuckDuckGoResult>().await?;

        // Store the processed data (search results) in the organism's output field
        self.data.output = Some(format!(
            "Heading: {}\nAbstract: {}\nURL: {}",
            response.Heading, response.AbstractText, response.AbstractURL
        ));

        println!("Organism {} processed data.", self.id);

        Ok(())
    }

    // Method to return an inference based on the processed data
    fn infer(&self) -> String {
        // If processed data exists, return the formatted result
        if let Some(ref output) = self.data.output {
            format!("Inference:\n{}", output)
        } else {
            String::from("No data processed yet.")
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Ask the user for a search query
    print!("Enter a search query: ");
    io::stdout().flush()?; // Ensures the prompt is printed before user input
    let mut search_query = String::new();
    io::stdin().read_line(&mut search_query)?;
    let search_query = search_query.trim().to_string(); // Trim input to remove extra spaces or newlines

    // Create a new organism with the input search query
    let mut organism = Organism::new(1, search_query);

    // Call the process_data method to perform the search
    organism.process_data().await?;

    // Get the inference from the processed data
    let inference = organism.infer();
    println!("{}", inference);

    Ok(())
}
