use toml;


pub struct GlobalConfig
{
    pub database_url: String,
    pub database_user: String,
    pub database_password: String,
    pub database_db: String,
}

impl GlobalConfig {
    pub fn init() -> Self{
        
    }
}

