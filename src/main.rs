
use clap::Parser;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio;
use copypasta::{ClipboardContext, ClipboardProvider};

const API_PATH: &str = "https://file.io";

#[derive(Parser)]
struct Cli {
    file_path: std::path::PathBuf,
}
#[derive(Debug, Serialize, Deserialize)]
struct FileRequest {
    file: String,
    
}

#[derive(Debug, Serialize, Deserialize)]

struct FileResponse {
    link: String
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let file_path = args.file_path;
    let file_name = String::from(file_path.file_name().unwrap().to_str().unwrap());
    post_file(file_path, file_name).await?;

    Ok(())
}

async fn post_file(file_path: std::path::PathBuf, file_name: String ) -> Result<()> {
    let bytes = std::fs::read(&file_path).unwrap();    //let byte_string = String::from_utf8(bytes).unwrap();
    
    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name(file_name);
    let form = reqwest::multipart::Form::new()
        .part("file", part);

    let result_json =  reqwest::Client::new()
        .post(API_PATH)
        .multipart(form)
        .send()
        .await?;
    
    let response = result_json.json::<FileResponse>().await?;
    
    // Create clipboardContext
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(response.link.to_owned()).unwrap();
    
    println!("Download page link copied to clipboard");
    println!("{}", response.link);

    Ok(())
}
