
use clap::Parser;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio;
use copypasta::{ClipboardContext, ClipboardProvider};

const API_PATH: &str = "https://file.io";

#[derive(Parser)]
struct Cli {
    file_path: std::path::PathBuf,
    #[clap(long, short('d'), action)]
    no_delete: bool,
    #[clap(long, short('n'), action)]
    max_downloads: i32
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
    let res = post_file(file_path, file_name, args.no_delete, args.max_downloads).await;
    match res {
        Ok(()) => {}
        Err(error) => println!("File could not be uploaded\n{}", error.to_string())
    }

    Ok(())
}

async fn post_file(file_path: std::path::PathBuf, file_name: String, no_delete: bool, max_downloads: i32 ) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read(&file_path)?;
    println!("{}", (!no_delete).to_string());
    println!("{}", max_downloads.to_string());
    let file_part = reqwest::multipart::Part::bytes(bytes)
        .file_name(file_name);


    let form = reqwest::multipart::Form::new()
        .part("file", file_part);
        //.text("autoDelete", format!("\"{}\"", (!no_delete).to_string()))
        //.text("maxDownloads", max_downloads.to_string());

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
