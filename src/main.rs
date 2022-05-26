mod config;
mod routes;
mod models;
fn main() {
    config::print_config();
    routes::health_route::print_health_route();
    models::user_model::print_user_model();
    println!("main");
}
