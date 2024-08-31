use crate::shared::env::get_env_with_panic;

pub fn get_address() -> String {
    get_env_with_panic("ADDRESS")
}

pub fn get_port() -> String {
    get_env_with_panic("PORT")
}

pub fn get_database_url() -> String {
    get_env_with_panic("DATABASE_URL")
}