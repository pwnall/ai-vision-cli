use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use clap::Parser;
use reqwest::blocking::Client;
use serde_json::{Value, json};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about = "A CLI tool for passing images into LLMs", long_about = None)]
struct Cli {
    #[arg(short, long)]
    image: Vec<PathBuf>,

    #[arg(short, long)]
    text: Option<PathBuf>,

    prompt: Option<String>,

    #[arg(short, long, default_value = "gemini-pro-latest")]
    model: String,

    #[arg(
        long,
        default_value = "https://generativelanguage.googleapis.com/v1beta/openai"
    )]
    api_base_url: String,
}

fn get_mime_type(path: &std::path::Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?.to_lowercase();
    match ext.as_str() {
        "png" => Some("image/png"),
        "jpeg" | "jpg" => Some("image/jpeg"),
        "webp" => Some("image/webp"),
        _ => None,
    }
}

fn main() {
    let cli = Cli::parse();

    let api_key = env::var("GEMINI_API_KEY")
        .or_else(|_| env::var("OPENAI_API_KEY"))
        .unwrap_or_else(|_| {
            eprintln!("API key not found. Set GEMINI_API_KEY or OPENAI_API_KEY.");
            process::exit(1);
        });

    let text_prompt = if let Some(text_path) = cli.text {
        fs::read_to_string(&text_path).unwrap_or_else(|_| {
            eprintln!("File not found: {}", text_path.display());
            process::exit(1);
        })
    } else {
        cli.prompt.unwrap_or_else(String::new)
    };

    let mut content = Vec::new();

    if !text_prompt.is_empty() {
        content.push(json!({
            "type": "text",
            "text": text_prompt,
        }));
    }

    for image_path in &cli.image {
        if !image_path.exists() {
            eprintln!("File not found: {}", image_path.display());
            process::exit(1);
        }

        let mime_type = get_mime_type(image_path).unwrap_or_else(|| {
            eprintln!("Unsupported image format: {}", image_path.display());
            process::exit(1);
        });

        let image_data = fs::read(image_path).unwrap_or_else(|_| {
            eprintln!("File not found: {}", image_path.display());
            process::exit(1);
        });

        let base64_image = BASE64.encode(image_data);
        let data_url = format!("data:{};base64,{}", mime_type, base64_image);

        content.push(json!({
            "type": "image_url",
            "image_url": {
                "url": data_url
            }
        }));
    }

    let payload = json!({
        "model": cli.model,
        "messages": [
            {
                "role": "user",
                "content": content
            }
        ]
    });

    let client = Client::new();
    let url = format!("{}/chat/completions", cli.api_base_url);

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .unwrap_or_else(|e| {
            eprintln!("Request failed: {}", e);
            process::exit(1);
        });

    if !response.status().is_success() {
        eprintln!("API error: {}", response.status());
        eprintln!("{}", response.text().unwrap_or_default());
        process::exit(1);
    }

    let response_body: Value = response.json().unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON response: {}", e);
        process::exit(1);
    });

    if let Some(content) = response_body["choices"][0]["message"]["content"].as_str() {
        println!("{}", content);
    } else {
        eprintln!("Unexpected response format: {}", response_body);
        process::exit(1);
    }
}
