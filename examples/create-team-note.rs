use hackmd_rs::{
    context::Context,
    note::{Note, NoteCreate, NoteUpdate},
    permisson::{CommentPermisson, RWPermission},
    team::Team,
};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::main]
async fn main() {
    let context = Context::new(&env::var("HACKMD_TOKEN").expect("HACKMD_TOKEN not found!"));
    let note_create = NoteCreate {
        title: format!(
            "[TEST]{} hackmd-rs TEAM API Test",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("WTF")
                .as_secs()
        ),
        content: format!(
            "# [TEST]{} hackmd-rs TEAM API Test\n\nTest text here...\\\nTest text here...\\\nTest text here...\n",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("WTF")
                .as_secs()
        ),
        read_permission: RWPermission::SignedIn,
        write_permission: RWPermission::Owner,
        comment_permission: CommentPermisson::Everyone,
        permalink: format!(
            "api-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("WTF")
                .as_secs()
        ),
    };

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

    println!("Creating note in team {}...", &teams[0].name);
    let new_note = note_create
        .execute_in_team(&context, &teams[0].path)
        .await
        .expect("Failed creating note...");
    println!("Link: {}", new_note.publish_link);

    {
        println!("Editing note...");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
    }

    let update = NoteUpdate {
        content: Some(format!(
            "{}\n## Changed at {}\n\nTest text here...\\\nTest text here...\\\nTest text here...\n",
            new_note.content,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("WTF")
                .as_secs()
        )),
        read_permission: None,
        write_permission: None,
        permalink: None,
    };

    new_note
        .update(&context, &update)
        .await
        .expect("Failed updating note...");

    {
        println!("Deleting note...");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
    }

    Note::delete(&context, &new_note.id)
        .await
        .expect("Failed deleting note...");
}
