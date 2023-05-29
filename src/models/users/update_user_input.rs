use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

use crate::models::education::Education;

/// An InputObject for User update query in GraphQL
#[derive(Serialize, Deserialize, Clone, InputObject)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<String>,
    pub skills: Option<String>,
    pub primary_email: Option<String>,
    pub about: Option<String>,
    pub education: Option<Vec<Education>>,
}
