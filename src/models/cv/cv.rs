use async_graphql as gql;
use async_graphql::{ComplexObject, SimpleObject};
use gql::futures_util::StreamExt;
use gql::{connection, Context};
use mongodb::bson::{self, DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::{
    data_source::mongo::{MongoDB, MongoForTesting},
    models::{comment::Comment, ResourceIdentifier},
    object_id::ScalarObjectId,
    services::cv_service::comment_service::CommentService,
};

use super::CreateCVInput;

/// Struct represents CV defined in the Diagram. Note that this struct only
/// represents the metadata of a CV.
#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(complex)]
pub struct CV {
    #[serde(rename = "_id")]
    pub id: ScalarObjectId,
    pub author_id: ScalarObjectId,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    #[graphql(skip)]
    pub comments: Vec<bson::oid::ObjectId>,
    #[graphql(skip)]
    pub created: DateTime,
    /// The resource identifier of the CV, can be used to query the actual CV data on the storage.
    pub cv: Option<ResourceIdentifier>,
}

#[ComplexObject]
impl CV {
    async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }

    /// Get comments of this CV.
    async fn comments(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> gql::Result<
        connection::Connection<
            ScalarObjectId,
            Comment,
            connection::EmptyFields,
            connection::EmptyFields,
        >,
    > {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        let comments_list = CommentService::get_comments_list_by_cv_id(db, self.id.into())
            .await
            .collect::<Vec<_>>()
            .await;
        connection::query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let comments_list = if let Some(after) = after {
                    comments_list
                        .into_iter()
                        .skip_while(|comment| comment.as_ref().unwrap().id != after)
                        .skip(1)
                        .map(|comment| comment)
                        .collect::<Vec<_>>()
                } else if let Some(before) = before {
                    comments_list
                        .into_iter()
                        .take_while(|comment| comment.as_ref().unwrap().id != before)
                        .map(|comment| comment)
                        .collect::<Vec<_>>()
                } else {
                    comments_list.into_iter().collect::<Vec<_>>()
                };
                let comments_list = if let Some(first) = first {
                    comments_list
                        .into_iter()
                        .take(first as usize)
                        .collect::<Vec<_>>()
                } else if let Some(last) = last {
                    let size = comments_list.len() as usize;
                    comments_list
                        .into_iter()
                        .skip(size - last as usize)
                        .collect::<Vec<_>>()
                } else {
                    panic!("Must have either 'first' or 'last' argument")
                };
                let mut connection = connection::Connection::new(true, false);
                connection
                    .edges
                    .extend(comments_list.into_iter().map(|comment| {
                        connection::Edge::new(comment.as_ref().unwrap().id, comment.unwrap())
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

impl From<CreateCVInput> for CV {
    fn from(input: CreateCVInput) -> Self {
        Self {
            id: bson::oid::ObjectId::new().into(),
            author_id: input.author_id.into(),
            title: input.title,
            description: input.description,
            tags: input.tags,
            comments: Vec::default(),
            created: DateTime::now(),
            cv: Uuid::new().into(),
        }
    }
}
