use clickhouse::Client;

// 数据库连接池
pub struct ClickhouseConnect {
    pub client: Client,
}

impl ClickhouseConnect {
    pub fn new() -> Self {
        let client = Client::default()
            .with_url("http://clickhouse:8123")
            .with_user("cheakf")
            .with_password("Swq8855830.")
            .with_database("default");
        let connect = ClickhouseConnect { client: client };
        connect
    }
}

impl Drop for ClickhouseConnect {
    fn drop(&mut self) {
        let client = std::mem::replace(&mut self.client, Client::default());
        drop(client);
    }
}