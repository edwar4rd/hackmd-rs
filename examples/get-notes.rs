use hackmd_rs::{context::Context, note::SimplifiedNote};
use std::env;

#[tokio::main]
async fn main() {
    let context = Context::new(&env::var("HACKMD_TOKEN").expect("HACKMD_TOKEN not found!"));
    let notes = SimplifiedNote::get_all_user(&context).await.unwrap();
    println!("Notes of the user: {:?}", notes);
    if notes.len() > 0 {
        let note = notes[0].get_complete(&context).await.unwrap();
        println!("First note of the user:\n\n{:50}...", note.content);
    }
}
