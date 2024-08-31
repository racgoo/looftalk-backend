pub mod config;
use std::env;
use std::path::PathBuf;
use dotenv::from_filename;

pub fn load_env() -> Option<PathBuf>   {
    match env::var("CARGO_ENV").as_deref() {
        Ok("production") => {
            from_filename(".env.production").ok()
        }
        Ok("development") => {
            println!("developemtn");
            from_filename(".env.development").ok()
        }
        _ => from_filename(".env.development").ok()
    }
}

pub fn get_env(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(env_data)=> {
            Some(env_data)
        },
        Err(_) => {
            None
        }
    }
}

pub fn get_env_with_panic(key: &str) -> String {
    if let Some(env_data) = get_env(key){
        env_data
    }else{
        panic!("INVALID_ENV");
    }
}