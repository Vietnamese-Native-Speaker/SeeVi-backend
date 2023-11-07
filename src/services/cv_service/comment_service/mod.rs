use async_graphql::futures_util::FutureExt;
use async_graphql::futures_util::{stream::BoxStream, StreamExt, TryStreamExt};
use mongodb::bson::oid::ObjectId;

mod error;

pub use error::CommentServiceError;

use crate::data_source::BookmarkDataSource;
use crate::data_source::LikeDataSource;
use crate::data_source::{CVDataSource, CVDataSourceError, CommentDataSource};
use crate::models::comment::{Comment, CreateCommentInput, CommentLike, UpdateCommentInput};

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
        let ids: Result<Vec<ObjectId>, CVDataSourceError> = database
            .get_comments_by_cv_id(cv_id)
            .await
            .map_err(|err| err.into());
        let rs = database.get_comments_list(ids.unwrap()).await;
        rs.map(|item| item.map_err(|err| err.into())).boxed()
    }

    pub async fn get_replies_of_comment(
        database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<BoxStream<Result<Comment, CommentServiceError>>, CommentServiceError> {
        let comment = match database.get_comment_by_id(comment_id).await {
            Ok(comment) => comment,
            Err(err) => return Err(err.into()),
        };
        let rs = database
            .get_comments_list(
                comment
                    .replies
                    .into_iter()
                    .map(|item| item.into())
                    .collect::<Vec<_>>(),
            )
            .map(|item| item.map_err(|err| err.into()))
            .await;
        Ok(rs.boxed())
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
        cmt_database: &(impl CommentDataSource + LikeDataSource + std::marker::Sync),
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let comment = cmt_database.get_comment_by_id(comment_id).await;
        match comment {
            Ok(_) => {
                let rs = cmt_database.add_like(user_id, comment_id).await;
                match rs {
                    Ok(_) => {
                        return Ok(comment.unwrap());
                    }
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    }
    pub async fn remove_like_comment(
        cmt_database: &(impl CommentDataSource + LikeDataSource + std::marker::Sync),
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let comment = cmt_database.get_comment_by_id(comment_id).await;
        match comment {
            Ok(_) => {
                let rs = cmt_database.delete_like(user_id, comment_id).await;
                match rs {
                    Ok(_) => {
                        return Ok(comment.unwrap());
                    }
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    }

    pub async fn add_bookmark(
        cmt_database: &(impl CommentDataSource + BookmarkDataSource + std::marker::Sync),
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let comment = cmt_database.get_comment_by_id(comment_id).await;
        match comment {
            Ok(_) => {
                let rs = cmt_database.add_bookmark(user_id, comment_id).await;
                match rs {
                    Ok(_) => {
                        return Ok(comment.unwrap());
                    }
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    }

    pub async fn remove_bookmark(
        cmt_database: &(impl CommentDataSource + BookmarkDataSource + std::marker::Sync),
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let comment = cmt_database.get_comment_by_id(comment_id).await;
        match comment {
            Ok(_) => {
                let rs = cmt_database.delete_bookmark(user_id, comment_id).await;
                match rs {
                    Ok(_) => {
                        return Ok(comment.unwrap());
                    }
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    }

    pub async fn get_bookmarks_count(
        cmt_database: &(impl CommentDataSource + BookmarkDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<i32, CommentServiceError> {
        let rs = cmt_database.get_bookmarks_count(comment_id).await;
        rs.map(|rs| rs).map_err(|err| err.into())
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

    pub async fn get_likes_count(
        cmt_database: &(impl CommentDataSource + LikeDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<i32, CommentServiceError> {
        let rs = cmt_database.get_likes_count_of_comment(comment_id).await;
        rs.map(|rs| rs).map_err(|err| err.into())
    }

    pub async fn get_likes(
        cmt_database: &(impl CommentDataSource + LikeDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<BoxStream<Result<CommentLike, CommentServiceError>>, CommentServiceError> {
        let likes = match cmt_database.get_likes(comment_id).await {
            Ok(likes) => likes,
            Err(err) => return Err(err.into()),
        };
        let rs = likes.map(|item| Ok(item)).boxed();
        Ok(rs)
    }
}
