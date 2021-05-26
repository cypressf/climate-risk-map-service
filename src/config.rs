use dotenv::dotenv;
use std::env;

struct AppConfig {
    url: String,
    port: u16,
}
struct DaoConfig {
    user: String,
    password: String,
    address: String,
    database: String,
}
pub struct Config {
    app: AppConfig,
    dao: DaoConfig,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let app_config = AppConfig {
            url: env::var("APP_URL").expect("APP_URL is not set"),
            port: env::var("APP_PORT")
                .expect("APP_PORT is not set")
                .parse::<u16>()
                .unwrap(),
        };
        let dao_config = DaoConfig {
            user: env::var("DATABASE_USER").expect("DATABASE_USER is not set"),
            password: env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD is not set"),
            address: env::var("DATABASE_ADDRESS").expect("DATABASE_ADDRESS is not set"),
            database: env::var("DATABASE").expect("DATABASE is not set"),
        };
        Config {
            app: app_config,
            dao: dao_config,
        }
    }

    pub fn get_app_url(&self) -> String {
        format!("{0}:{1}", self.app.url, self.app.port)
    }

    pub fn get_database_url(&self) -> String {
        format!(
            "mysql://{0}:{1}@{2}/{3}",
            self.dao.user, self.dao.password, self.dao.address, self.dao.database
        )
    }
}
