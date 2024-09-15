use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct DuckDuckGoResponse {
    Abstract: String,
    RelatedTopics: Vec<RelatedTopic>,
}

#[derive(Debug, Deserialize)]
struct RelatedTopic {
    Text: String,
}

pub struct Organism {
    pub id: u32,
    pub query: String,
    pub result: Option<String>,
}

impl Organism {
    pub fn new(id: u32, query: String) -> Self {
        Organism {
            id,
            query,
            result: None,
        }
    }

    pub async fn search_and_process(&mut self) -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let url = format!("https://api.duckduckgo.com/?q={}&format=json", self.query);

        let response = client.get(&url).send().await?;
        let body = response.text().await?;

        let duck_response: DuckDuckGoResponse = serde_json::from_str(&body)?;

        if !duck_response.Abstract.is_empty() {
            self.result = Some(duck_response.Abstract.clone());
        } else if let Some(first_topic) = duck_response.RelatedTopics.get(0) {
            self.result = Some(first_topic.Text.clone());
        } else {
            self.result = Some(String::from("No relevant information found."));
        }

        Ok(())
    }

    pub fn infer(&self) -> String {
        match &self.result {
            Some(result) => format!("Organism {} inference: {}", self.id, result),
            None => String::from("No data processed yet."),
        }
    }
}
