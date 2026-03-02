use anyhow::Result; // Much simpler than error_chain!

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Make the request
    let res = reqwest::get("https://www.google.com").await?;

    // 2. Print metadata
    println!("Status: {}", res.status()); // Note: .status() is a method
    println!("Headers: {:#?}", res.headers());

    // 3. Get the body text
    let body = res.text().await?;
    println!("Body length: {} characters", body.len());
    
    Ok(())
}