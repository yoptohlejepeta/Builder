use std::fs::File;
use std::io::Write;
use reqwest::StatusCode;

/// Function to get gitignore file for specified language
pub async fn get_ignore_file( language: String) -> () {
    let url = format!("https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore", language);

    let response = reqwest::get(&url).await.unwrap();

    match response.status() {
        StatusCode::OK => {
            let response_text = response.text().await.unwrap();
            let file = File::create(".gitignore");
            let _ = file.unwrap().write_all(response_text.as_bytes());
            println!("Gitignore file created");
        }
        StatusCode::NOT_FOUND => {
            println!("Gitignore file not found for language: {}", language);
        }
        _ => {
            println!("Failed to get gitignore file for language: {}", language);
        }
    }
}