use stripe::Client;

use std::env;

pub async fn initialise_client() -> Client {
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    Client::new(secret_key)
}
