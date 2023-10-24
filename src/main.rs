use std::env;

use seevi_backend::{run_server, run_server_for_test};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "--test" {
        println!("Running server for test");
        run_server_for_test().await;
    } else {
        run_server().await;
    }
}
