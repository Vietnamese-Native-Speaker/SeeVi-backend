//! Implement graphql-specific field for User

use async_graphql as gql;
use async_graphql::{connection, Context};
use gql::futures_util::StreamExt;

use crate::data_source::mongo::{MongoDB, MongoForTesting};
use crate::models::cv::CV;
use crate::models::users::User;
use crate::object_id::ScalarObjectId;
use crate::services::cv_service::bookmark_service::BookmarkService;
use crate::services::cv_service::cv_service::CVService;
use crate::services::user_service::UserService;

#[async_graphql::ComplexObject]
impl User {
    async fn friends(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> gql::Result<
        connection::Connection<
            ScalarObjectId,
            User,
            connection::EmptyFields,
            connection::EmptyFields,
        >,
    > {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        let friends_list = UserService::friend_lists(db, self.id.into())
            .await
            .collect::<Vec<_>>()
            .await;
        connection::query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let friends_list = if let Some(after) = after {
                    friends_list
                        .into_iter()
                        .skip_while(|friend| friend.as_ref().unwrap().id != after)
                        .skip(1)
                        .map(|friend| friend)
                        .collect::<Vec<_>>()
                } else if let Some(before) = before {
                    friends_list
                        .into_iter()
                        .take_while(|friend| friend.as_ref().unwrap().id != before)
                        .map(|friend| friend)
                        .collect::<Vec<_>>()
                } else {
                    friends_list.into_iter().collect::<Vec<_>>()
                };
                let friends_list = if let Some(first) = first {
                    friends_list
                        .into_iter()
                        .take(first as usize)
                        .collect::<Vec<_>>()
                } else if let Some(last) = last {
                    let size = friends_list.len() as usize;
                    friends_list
                        .into_iter()
                        .skip(size - last as usize)
                        .collect::<Vec<_>>()
                } else {
                    panic!("Must have either 'first' or 'last' argument")
                };
                let mut connection = connection::Connection::new(true, false);
                connection
                    .edges
                    .extend(friends_list.into_iter().map(|friend| {
                        connection::Edge::new(friend.as_ref().unwrap().id, friend.unwrap())
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }

    async fn cvs(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> gql::Result<
        connection::Connection<
            ScalarObjectId,
            CV,
            connection::EmptyFields,
            connection::EmptyFields,
        >,
    > {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        let cvs = CVService::get_cvs_by_user_id(db, self.id.into())
            .await?
            .collect::<Vec<_>>()
            .await;
        connection::query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let cvs = if let Some(after) = after {
                    cvs.into_iter()
                        .skip_while(|cv| cv.as_ref().unwrap().id != after)
                        .skip(1)
                        .collect::<Vec<_>>()
                } else if let Some(before) = before {
                    cvs.into_iter()
                        .take_while(|cv| cv.as_ref().unwrap().id != before)
                        .collect::<Vec<_>>()
                } else {
                    cvs.into_iter().collect::<Vec<_>>()
                };
                let cvs = if let Some(first) = first {
                    cvs.into_iter().take(first as usize).collect::<Vec<_>>()
                } else if let Some(last) = last {
                    let size = cvs.len() as usize;
                    cvs.into_iter()
                        .skip(size - last as usize)
                        .collect::<Vec<_>>()
                } else {
                    panic!("Must have either 'first' or 'last' argument")
                };
                let mut connection = connection::Connection::new(true, false);
                connection.edges.extend(cvs.into_iter().map(|friend| {
                    connection::Edge::new(friend.as_ref().unwrap().id, friend.unwrap())
                }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }

    async fn bookmarked_cvs(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> gql::Result<
        connection::Connection<
            ScalarObjectId,
            CV,
            connection::EmptyFields,
            connection::EmptyFields,
        >,
    > {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        let shares = BookmarkService::get_bookmarked_cvs_of_user(db, self.id.into())
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
                        .skip_while(|cv| cv.as_ref().unwrap().id != after)
                        .skip(1)
                        .collect::<Vec<_>>()
                } else if let Some(before) = before {
                    shares
                        .into_iter()
                        .take_while(|cv| cv.as_ref().unwrap().id != before)
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
                connection.edges.extend(
                    comments_list
                        .into_iter()
                        .map(|cv| connection::Edge::new(cv.as_ref().unwrap().id, cv.unwrap())),
                );
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}
