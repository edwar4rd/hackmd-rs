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
    for team in teams.iter() {
        println!("\t{} ({})", team.name, team.description);
    }

    let notes = teams[0]
        .notes(&context)
        .await
        .expect("Failed getting notes");
    if notes.is_empty() {
        return;
    }
    let note = &notes[0];
    println!("Note from team: {}", note.title);
    let complete = notes[0]
        .get_complete(&context)
        .await
        .expect("Failed getting complete note");
    println!("Note content: \n{}\n", &complete.content);
}
