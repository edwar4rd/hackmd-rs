use serde::{Deserialize, Serialize};

use crate::{
    context::Context,
    error::Result,
    permisson::{CommentPermisson, RWPermission},
    user::SimplifiedUser,
};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub created_at: usize,
    // TODO: should be an enum, not enough documentation
    pub publish_type: String,
    pub published_at: Option<usize>,
    pub permalink: Option<String>,
    pub short_id: String,
    pub content: String,
    pub last_changed_at: usize,
    pub last_changed_user: Option<SimplifiedUser>,
    pub user_path: String,
    pub team_path: Option<String>,
    pub read_permission: RWPermission,
    pub write_permission: RWPermission,
    pub publish_link: String,
}

impl Note {
    pub async fn get(context: &Context, id: &str) -> Result<Note> {
        let path = format!("notes/{id}");

        context.get(&path).await
    }

    pub async fn update(&self, context: &Context, update: &NoteUpdate) -> Result<()> {
        update.apply(context, &self.id).await
    }

    pub async fn delete(context: &Context, id: &str) -> Result<()> {
        context.delete(&format!("notes/{id}")).await
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedNote {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub created_at: usize,
    // TODO: should be an enum, not enough documentation
    pub publish_type: String,
    pub published_at: Option<usize>,
    pub permalink: Option<String>,
    pub short_id: String,
    pub last_changed_at: usize,
    pub last_changed_user: Option<SimplifiedUser>,
    pub user_path: String,
    pub team_path: Option<String>,
    pub read_permission: RWPermission,
    pub write_permission: RWPermission,
    pub publish_link: String,
}

impl SimplifiedNote {
    pub async fn get_all(context: &Context) -> Result<Vec<SimplifiedNote>> {
        context.get("notes").await
    }

    pub async fn get_complete(&self, context: &Context) -> Result<Note> {
        Note::get(context, &self.id).await
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteCreate {
    /// seemed to be unused by hackmd api by now
    pub title: String,
    pub content: String,
    pub read_permission: RWPermission,
    pub write_permission: RWPermission,
    pub comment_permission: CommentPermisson,
    pub permalink: String,
}

impl NoteCreate {
    pub async fn execute(mut self, context: &Context) -> Result<Note> {
        if self.read_permission > self.write_permission {
            self.write_permission = self.read_permission;
        }

        context.post("notes", &self).await
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_permission: Option<RWPermission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_permission: Option<RWPermission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
}

impl NoteUpdate {
    pub async fn apply(&self, context: &Context, id: &str) -> Result<()> {
        let path = format!("notes/{id}");

        context.patch(&path, self).await
    }
}
