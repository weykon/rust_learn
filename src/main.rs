mod config;
mod routes;
fn main() {
    config::print_config();
    routes::print_health_route();
    println!("main");
}
