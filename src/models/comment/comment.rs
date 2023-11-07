use async_graphql as gql;
use async_graphql::{ComplexObject, Context, SimpleObject};
use gql::futures_util::StreamExt;
use gql::{connection, ErrorExtensions};
use mongodb::bson::{self, DateTime};
use serde::{Deserialize, Serialize};

use crate::{
    data_source::mongo::{MongoDB, MongoForTesting},
    object_id::ScalarObjectId,
    services::cv_service::comment_service::CommentService,
};

use super::create_comment_input::CreateCommentInput;
use super::Like;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, Builder)]
#[graphql(complex)]
pub struct Comment {
    #[serde(rename = "_id")]
    pub id: ScalarObjectId,
    pub author: ScalarObjectId,

    pub content: String,

    #[graphql(skip)]
    pub created: DateTime,

    #[graphql(skip)]
    pub replies: Vec<ScalarObjectId>,
}

#[ComplexObject]
impl Comment {
    async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }

    /// Get replies of this comment.
    async fn replies(
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
        let comments_list = match CommentService::get_replies_of_comment(db, self.id.into()).await {
            Ok(comments_list) => comments_list,
            Err(e) => return Err(e.extend()),
        };
        let comments_list = comments_list.collect::<Vec<_>>().await;
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

    /// Get likes of this comment
    async fn likes(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> gql::Result<
        connection::Connection<
            ScalarObjectId,
            Like,
            connection::EmptyFields,
            connection::EmptyFields,
        >,
    > {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        let likes_list = CommentService::get_likes(db, self.id.into())
            .await?
            .collect::<Vec<_>>()
            .await;
        connection::query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let likes_list = if let Some(after) = after {
                    likes_list
                        .into_iter()
                        .skip_while(|like| like.as_ref().unwrap().key.user_id != after)
                        .skip(1)
                        .map(|like| like)
                        .collect::<Vec<_>>()
                } else if let Some(before) = before {
                    likes_list
                        .into_iter()
                        .take_while(|like| like.as_ref().unwrap().key.user_id != before)
                        .map(|like| like)
                        .collect::<Vec<_>>()
                } else {
                    likes_list.into_iter().collect::<Vec<_>>()
                };
                let friends_list = if let Some(first) = first {
                    likes_list
                        .into_iter()
                        .take(first as usize)
                        .collect::<Vec<_>>()
                } else if let Some(last) = last {
                    let size = likes_list.len() as usize;
                    likes_list
                        .into_iter()
                        .skip(size - last as usize)
                        .collect::<Vec<_>>()
                } else {
                    panic!("Must have either 'first' or 'last' argument")
                };
                let mut connection = connection::Connection::new(true, false);
                connection
                    .edges
                    .extend(friends_list.into_iter().map(|like| {
                        connection::Edge::new(like.as_ref().unwrap().key.user_id, like.unwrap())
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }

    /// Get the number of likes of this comment.
    async fn likes_count(&self, ctx: &Context<'_>, comment_id: ScalarObjectId) -> gql::Result<i32> {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        let rs = CommentService::get_likes_count(db, comment_id.into()).await;
        match rs {
            Ok(count) => Ok(count),
            Err(e) => Err(e.extend()),
        }
    }
}

impl Comment {
    pub fn new(content: String, author: bson::oid::ObjectId) -> Self {
        Self {
            id: bson::oid::ObjectId::new().into(),
            author: author.into(),
            content,
            created: bson::DateTime::now(),
            replies: vec![],
        }
    }
}

impl From<CreateCommentInput> for Comment {
    fn from(input: CreateCommentInput) -> Self {
        Self::new(input.content, input.author.into())
    }
}
