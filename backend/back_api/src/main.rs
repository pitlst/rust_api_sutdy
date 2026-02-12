use clickhouse::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = Client::default()
        .with_url("http://clickhouse:8123")
        .with_user("cheakf")
        .with_password("Swq8855830.")
        .with_database("default");

    // 简单查询
    let result: u8 = client.query("SELECT 1").fetch_one().await?;

    println!("Result: {}", result);
    Ok(())
}
