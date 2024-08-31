use axum::Router;



mod room;
mod user;
mod chat;

pub fn create_routes() -> Router {
    Router::new()
        .nest("/user",user::create_user_routes())
        .nest("/chat",chat::create_chat_routes())
        .nest("/room",room::create_room_routes())
}

