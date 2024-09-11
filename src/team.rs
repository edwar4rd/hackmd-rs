use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{context::Context, error::Result, note::SimplifiedNote};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub path: String,
    pub name: String,
    pub logo: String,
    pub description: String,
    pub visibility: TeamVisibility,
    // TODO: we should use chrono here
    pub created_at: usize,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TeamVisibility {
    Public,
    Private,
}

impl Team {
    pub async fn mine(context: &Context) -> Result<Vec<Team>> {
        context.get("teams").await
    }

    pub async fn notes(&self, context: &Context) -> Result<Vec<SimplifiedNote>> {
        SimplifiedNote::get_all_team(context, &self.path).await
    }
}
