use hackmd_rs::{context::Context, team::Team};
use std::env;

#[tokio::main]
async fn main() {
    let context = Context::new(&env::var("HACKMD_TOKEN").expect("HACKMD_TOKEN not found!"));
    let teams = Team::mine(&context)
        .await
        .expect("Failed getting team info");

    if teams.is_empty() {
        return;
    }

    println!("User teams:");
    for team in teams {
        println!("\t{} ({})", team.name, team.description);
    }
}
