mod mariadb;
mod redis;

pub struct AppState {

}

impl AppState {
    fn new() -> AppState {
        AppState {}
    }
    fn init_database_connection(&self)  {
        //db 연결 및 pool저장
    }
    fn init_redis_connection(&self)  {
        //redis 연결 및 pool저장
    }
}

pub fn create_app_state() -> AppState {
    let app_state = AppState::new();
    app_state.init_redis_connection();
    app_state.init_redis_connection();
    app_state
}