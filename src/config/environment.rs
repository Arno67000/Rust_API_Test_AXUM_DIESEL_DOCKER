use std::{env, net::Ipv4Addr, path::PathBuf};

#[derive(Debug)]
pub struct Environment {
    pub port: u16,
    pub host: Ipv4Addr,
    pub database_url: String,
    pub log_level: String,
}

impl Environment {
    fn init() -> Option<PathBuf> {
        let current_env = env::var("ENVIRONMENT").unwrap_or(String::from("development"));
        println!("Running in {}", current_env);

        if current_env == "development" {
            dotenv::dotenv().ok()
        } else {
            None
        }
    }

    pub fn get() -> Environment {
        Environment::init();

        let mut host = env::var("HOST").expect("Missing HOST");

        if host.contains("localhost") {
            host = String::from("0.0.0.0");
        }

        Environment {
            port: env::var("PORT")
                .expect("Missing PORT in env file")
                .parse::<u16>()
                .expect("PORT is not a valid number"),
            host: host
                .parse::<Ipv4Addr>()
                .expect("Malformed IPV4 for HOST variable"),
            database_url: env::var("DATABASE_URL").expect("Missing DATABASE_URL in env file"),
            log_level: env::var("LOG_LEVEL").unwrap_or(String::from("debug")),
        }
    }
}
