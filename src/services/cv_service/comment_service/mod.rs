use async_graphql::futures_util::{stream::BoxStream, StreamExt};
use mongodb::bson::oid::ObjectId;

mod error;

pub use error::CommentServiceError;

use crate::data_source::{CommentDataSource, CVDataSource, CVDataSourceError};
use crate::models::comment::{Comment, CreateCommentInput, UpdateCommentInput};

pub struct CommentService {}

impl CommentService {
    pub async fn create_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        input: CreateCommentInput,
    ) -> Result<Comment, CommentServiceError> {
        let rs = cmt_database.create_comment(input).await;
        match rs {
            Ok(rs) => {
                return Ok(rs);
            }
            Err(err) => {
                let err: CommentServiceError = err.into();
                return Err(err);
            }
        }
    }

    pub async fn get_comment_by_id(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let rs = cmt_database.get_comment_by_id(comment_id).await;
        rs.map_or_else(|err| Err(err.into()), |rs| Ok(rs))
    }

    pub async fn get_comments_list_by_cv_id(
        database: &(impl CVDataSource + CommentDataSource + std::marker::Sync),
        cv_id: ObjectId,
    ) -> BoxStream<Result<Comment, CommentServiceError>> {
        let ids: Result<Vec<ObjectId>, CVDataSourceError> = database.get_comments_by_cv_id(cv_id).await.map_err(|err| err.into());
        let rs = database.get_comments_list(ids.unwrap()).await;
        rs.map(|item| {
            item.map_err(|err| {
                err.into()
            })
        }).boxed()
    }

    pub async fn update_content_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
        content: String,
    ) -> Result<Comment, CommentServiceError> {
        let input = UpdateCommentInput::builder()
            .with_content(content)
            .build()
            .unwrap();
        let rs = cmt_database
            .find_and_update_comment(comment_id, input)
            .await;
        rs.map(|rs| rs).map_err(|err| err.into())
    }

    pub async fn add_like_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let cmt = cmt_database.get_comment_by_id(comment_id).await;
        match cmt {
            Ok(cmt) => {
                let input = UpdateCommentInput::builder()
                    .with_likes(cmt.likes + 1)
                    .build()
                    .unwrap();
                let rs = cmt_database.find_and_update_comment(cmt.id.into(), input).await;
                match rs {
                    Ok(rs) => Ok(rs),
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    }
    pub async fn remove_like_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let comment = cmt_database.get_comment_by_id(comment_id).await;
        match comment {
            Ok(comment) => {
                let tmp = comment.likes;
                if tmp < 1 {
                    return Err(CommentServiceError::NoLikes);
                }
                let input = UpdateCommentInput::builder()
                    .with_likes(comment.likes - 1)
                    .build()
                    .unwrap();
                let rs = cmt_database.find_and_update_comment(comment_id, input).await;
                match rs {
                    Ok(rs) => Ok(rs),
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    }

    pub async fn add_bookmark(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let cv = cmt_database.get_comment_by_id(comment_id).await;
        match cv {
            Ok(cv) => {
                let input = UpdateCommentInput::builder()
                    .with_bookmarks(cv.bookmarks + 1)
                    .build()
                    .unwrap();
                let rs = cmt_database.find_and_update_comment(comment_id, input).await;
                match rs {
                    Ok(rs) => Ok(rs),
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    }

    pub async fn remove_bookmark(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let cmt = cmt_database.get_comment_by_id(comment_id).await;
        match cmt {
            Ok(cmt) => {
                let tmp = cmt.bookmarks;
                if tmp == 0 {
                    return Err(CommentServiceError::NoBookmarks);
                }
                let input = UpdateCommentInput::builder()
                    .with_bookmarks(cmt.bookmarks - 1)
                    .build()
                    .unwrap();
                let rs = cmt_database.find_and_update_comment(comment_id, input).await;
                match rs {
                    Ok(rs) => {
                        return Ok(rs);
                    }
                    Err(err) => {
                        let err: CommentServiceError = err.into();
                        return Err(err);
                    }
                }
            }
            Err(err) => {
                let err: CommentServiceError = err.into();
                return Err(err);
            }
        }
    }
    pub async fn add_share_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let cmt = cmt_database.get_comment_by_id(comment_id).await;
        match cmt {
            Ok(cmt) => {
                let input = UpdateCommentInput::builder()
                    .with_shares(cmt.shares + 1)
                    .build()
                    .unwrap();
                let rs = cmt_database.find_and_update_comment(comment_id, input).await;
                match rs {
                    Ok(rs) => Ok(rs),
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    }
    pub async fn add_reply_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
        user_id: ObjectId,
        content: String,
    ) -> Result<Comment, CommentServiceError> {
        let cmt = cmt_database.get_comment_by_id(comment_id).await;
        match cmt {
            Ok(cmt) => {
                let create_input = CreateCommentInput {
                    author: user_id.into(),
                    content,
                };
                let new_cmt = cmt_database.create_comment(create_input).await;
                match new_cmt {
                    Ok(new_cmt) => {
                        let rs = cmt_database
                            .add_reply_to_comment(cmt.id.into(), new_cmt.id.into())
                            .await;
                        match rs {
                            Ok(rs) => Ok(rs),
                            Err(err) => Err(err.into()),
                        }
                    }
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    }
    pub async fn remove_reply_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
        reply_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let rs = cmt_database
            .find_and_remove_reply(comment_id, reply_id)
            .await;
        rs.map(|rs| rs).map_err(|err| err.into())
    }
}
