use hackmd_rs::{context::Context, user::User};
use std::env;

#[tokio::main]
async fn main() {
    let context = Context::new(&env::var("HACKMD_TOKEN").expect("HACKMD_TOKEN not found!"));
    let me = User::me(&context).await.expect("Failed getting user info");
    println!("User {}: {:?}", me.name, me);

    let latest = User::get_history(&context)
        .await
        .expect("Failed getting user history");
    if latest.is_empty() {
        return;
    }

    println!("");

    println!("Latest read notes:\n");
    for note in latest {
        println!("\t{} ({})", note.title, note.publish_link);
    }
}
