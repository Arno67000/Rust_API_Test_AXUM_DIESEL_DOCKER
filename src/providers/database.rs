use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Debug, Clone)]
pub struct DB {
    pub url: String,
}

impl DB {
    pub fn new(url: String) -> DB {
        DB { url }
    }
    pub fn db_connect(self) -> PgConnection {
        PgConnection::establish(&self.url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &self.url))
    }
}
