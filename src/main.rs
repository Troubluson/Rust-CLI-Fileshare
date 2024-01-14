
use clap::Parser;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio;
use copypasta::{ClipboardContext, ClipboardProvider};

const API_PATH: &str = "https://file.io";

#[derive(Parser)]
struct Cli {
    file_path: std::path::PathBuf,
    #[clap(long, short('o'), default_value_t=String::new())]
    output_name: String
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
    let res = post_file(file_path, file_name, args.output_name).await;
    match res {
        Ok(()) => {}
        Err(error) => println!("File could not be uploaded\n{}", error.to_string())
    }

    Ok(())
}

async fn post_file(file_path: std::path::PathBuf, file_name: String, output_name: String ) -> Result<(), Box<dyn std::error::Error>> {
    let output_file_name = if !output_name.is_empty() { output_name } else { file_name };
    let bytes = std::fs::read(&file_path)?;

    let file_part = reqwest::multipart::Part::bytes(bytes)
        .file_name(output_file_name);


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
