[package]
name = "olcs"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = "0.11"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
