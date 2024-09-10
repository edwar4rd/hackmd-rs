use std::env;
use hackmd_rs::{context::Context, user::User};

#[tokio::main]
async fn main() {
    let context = Context::new(&env::var("HACKMD_TOKEN").expect("HACKMD_TOKEN not found!"));
    let me = User::me(&context).await.expect("Failed to parse result");
    println!("User {}: {:?}", me.name, me);
}
