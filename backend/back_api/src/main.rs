mod config;
mod database;
mod logger;
mod utils;

use database::clickhouse::ClickhouseConnect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let db = ClickhouseConnect::new();
    // 简单查询
    let result: u8 = db.client.query("SELECT 1").fetch_one().await?;
    println!("Result: {}", result);
    Ok(())
}
