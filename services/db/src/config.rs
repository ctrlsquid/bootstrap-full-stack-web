// This file creates a configuration including database configuration info


pub struct Config {
    database: DatabaseConfig,
}

struct DatabaseConfig {
    database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                database_url: "postgres://postgres:password@localhost:5432/postgres"
                    .to_string(),
            },
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn database_url(&self) -> &str {
        &self.database.database_url
    }

    pub fn set_database_url(&mut self, database_url: String) {
        self.database.database_url = database_url;
    }

    pub fn set_database_url_from_env(&mut self) {
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            self.set_database_url(database_url);
        }
    }
}