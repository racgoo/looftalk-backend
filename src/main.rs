mod controller;
mod shared;
use std::sync::Arc;
use axum::Extension;
use crate::controller::create_routes;
use crate::shared::env::config::{get_address, get_port};
use crate::shared::init::boot_strap;
use crate::shared::state::create_app_state;

#[tokio::main]
async fn main() {
    boot_strap();

    let app_state = Arc::new(create_app_state());
    let application = create_routes().layer(Extension(app_state.clone()));

    let server_ip = format!("{}:{}", get_address(), get_port());
    let tcp_listener = tokio::net::TcpListener::bind(server_ip).await.unwrap();

    axum::serve(tcp_listener, application.into_make_service()).await.unwrap_or_else( |err|{
        println!("{:?}",err)
    } );
}
