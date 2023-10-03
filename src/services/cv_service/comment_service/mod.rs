use mongodb::bson::oid::ObjectId;
use async_graphql::futures_util::{stream::BoxStream, StreamExt};

mod error;

pub use error::CommentServiceError;

use crate::data_source::CommentDataSource;
use crate::models::comment::{UpdateCommentInput, Comment, CreateCommentInput};

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

    pub async fn get_comments_by_cv_id(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        cv_id: ObjectId,
    ) -> BoxStream<Result<Comment, CommentServiceError>> {
        cmt_database.get_comments_by_cv_id(cv_id).await.map(|rs| rs.map_or_else(
            |err| Err(err.into()),
            |rs| Ok(rs),
        )).boxed()    
    }

    pub async fn update_content_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
        content: String,
    ) -> Result<Comment, CommentServiceError> {
        let comment = cmt_database.get_comment_by_id(comment_id).await;
        match comment {
            Ok(comment) => {
                let input: UpdateCommentInput = UpdateCommentInput {
                    id: comment_id.into(),
                    content: Some(content),
                    likes: Some(comment.likes),
                    bookmarks: Some(comment.bookmarks),
                    shares: Some(comment.shares),
                    replies: Some(comment.replies),
                };
                let rs = cmt_database.update_comment(input).await;
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

    pub async fn add_like_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> 
    {
        let comment = cmt_database.get_comment_by_id(comment_id).await;
        match comment {
            Ok(comment) => {
                let input = UpdateCommentInput {
                    id: comment_id.into(),
                    likes: Some(comment.likes + 1),
                    shares: Some(comment.shares),
                    bookmarks: Some(comment.bookmarks),
                    replies: Some(comment.replies),
                    content: Some(comment.content),
                };
                let rs = cmt_database.update_comment(input).await;
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
    pub async fn remove_like_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> 
    {
        let comment = cmt_database.get_comment_by_id(comment_id).await;
        match comment {
            Ok(comment) => {
                let tmp = comment.likes;
                if tmp < 1 {
                    return Err(CommentServiceError::InvalidLikes)
                }
                let input = UpdateCommentInput {
                    id: comment_id.into(),
                    likes: Some(comment.likes - 1),
                    shares: Some(comment.shares),
                    bookmarks: Some(comment.bookmarks),
                    replies: Some(comment.replies),
                    content: Some(comment.content),
                };
                let rs = cmt_database.update_comment(input).await;
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
    
    pub async fn add_bookmark(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let cv = cmt_database.get_comment_by_id(comment_id).await;
        match cv {
            Ok(cv) => {
                let bookmarks = cv.bookmarks + 1;
                let input = UpdateCommentInput {
                    id: comment_id.into(),
                    bookmarks: Some(bookmarks),
                    likes: Some(cv.likes),
                    shares: Some(cv.shares),
                    replies: Some(cv.replies),
                    content: Some(cv.content),
                };
                let rs = cmt_database.update_comment(input).await;
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
    
    pub async fn remove_bookmark(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let cmt = cmt_database.get_comment_by_id(comment_id).await;
        match cmt {
            Ok(cmt) => {
                let tmp = cmt.bookmarks;
                if tmp == 0 {
                    return Err(CommentServiceError::InvalidBookmarks)
                }
                let input = UpdateCommentInput {
                    id: comment_id.into(),
                    bookmarks: Some(cmt.bookmarks - 1),
                    likes: Some(cmt.likes),
                    shares: Some(cmt.shares),
                    replies: Some(cmt.replies),
                    content: Some(cmt.content),
                };
                let rs = cmt_database.update_comment(input).await;
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
                let input = UpdateCommentInput {
                    id: comment_id.into(),
                    shares: Some(cmt.shares + 1),
                    likes: Some(cmt.likes),
                    bookmarks: Some(cmt.bookmarks),
                    replies: Some(cmt.replies),
                    content: Some(cmt.content),
                };
                let rs = cmt_database.update_comment(input).await;
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
                        let rs = cmt_database.add_reply_to_comment(cmt.id.into(), new_cmt.id.into()).await;
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
            Err(err) => {
                let err: CommentServiceError = err.into();
                return Err(err);
            }
        }
    }
    pub async fn remove_reply_comment(
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        comment_id: ObjectId,
        reply_id: ObjectId,
    ) -> Result<Comment, CommentServiceError> {
        let cmt = cmt_database.get_comment_by_id(comment_id).await;
        match cmt {
            Ok(cmt) => {
                let rs = cmt_database.remove_reply_from_comment(cmt.id.into(), reply_id).await;
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
}