use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let body = reqwest::get("http://localhost:8080")
        .await?
        .text()
        .await?;
    println!("{body:?}");

    Ok(())
}
