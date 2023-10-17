//! Implement graphql-specific field for CV

use async_graphql as gql;
use async_graphql::{connection, futures_util::StreamExt, ComplexObject, Context};

use crate::models::cv::interactions::Share;
use crate::models::cv::Like;
use crate::services::cv_service::like_service::LikeService;
use crate::services::cv_service::share_service::ShareService;
use crate::{
    data_source::mongo::{MongoDB, MongoForTesting},
    models::{comment::Comment, cv::CV},
    object_id::ScalarObjectId,
    services::cv_service::comment_service::CommentService,
};

#[ComplexObject]
impl CV {
    async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }

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
        let likes = LikeService::get_likes_by_cv(db, self.id.into())
            .await?
            .collect::<Vec<_>>()
            .await;
        connection::query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let likes = if let Some(after) = after {
                    likes
                        .into_iter()
                        .skip_while(|like| {
                            ScalarObjectId::from(*like.as_ref().unwrap().user_id()) != after
                        })
                        .skip(1)
                        .map(|comment| comment)
                        .collect::<Vec<_>>()
                } else if let Some(before) = before {
                    likes
                        .into_iter()
                        .take_while(|like| {
                            ScalarObjectId::from(*like.as_ref().unwrap().user_id()) != before
                        })
                        .map(|comment| comment)
                        .collect::<Vec<_>>()
                } else {
                    likes.into_iter().collect::<Vec<_>>()
                };
                let comments_list = if let Some(first) = first {
                    likes.into_iter().take(first as usize).collect::<Vec<_>>()
                } else if let Some(last) = last {
                    let size = likes.len() as usize;
                    likes
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
                        connection::Edge::new(
                            (*comment.as_ref().unwrap().user_id()).into(),
                            comment.unwrap(),
                        )
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }

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

    async fn shares(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> gql::Result<
        connection::Connection<
            ScalarObjectId,
            Share,
            connection::EmptyFields,
            connection::EmptyFields,
        >,
    > {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        let shares = ShareService::get_shares_of_cv(db, self.id.into())
            .await?
            .collect::<Vec<_>>()
            .await;
        connection::query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let likes = if let Some(after) = after {
                    shares
                        .into_iter()
                        .skip_while(|like| {
                            ScalarObjectId::from(*like.as_ref().unwrap().user_id()) != after
                        })
                        .skip(1)
                        .map(|comment| comment)
                        .collect::<Vec<_>>()
                } else if let Some(before) = before {
                    shares
                        .into_iter()
                        .take_while(|like| {
                            ScalarObjectId::from(*like.as_ref().unwrap().user_id()) != before
                        })
                        .map(|comment| comment)
                        .collect::<Vec<_>>()
                } else {
                    shares.into_iter().collect::<Vec<_>>()
                };
                let comments_list = if let Some(first) = first {
                    likes.into_iter().take(first as usize).collect::<Vec<_>>()
                } else if let Some(last) = last {
                    let size = likes.len() as usize;
                    likes
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
                        connection::Edge::new(
                            (*comment.as_ref().unwrap().user_id()).into(),
                            comment.unwrap(),
                        )
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }

    async fn likes_count(&self, ctx: &Context<'_>) -> gql::Result<u64> {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        LikeService::get_likes_count(db, self.id.into())
            .await
            .map(|count| count as u64)
            .map_err(|err| err.into())
    }

    async fn shares_count(&self, ctx: &Context<'_>) -> gql::Result<u64> {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        ShareService::get_shares_count_of_cv(db, self.id.into())
            .await
            .map(|count| count as u64)
            .map_err(|err| err.into())
    }
}
