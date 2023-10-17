//! Implements the `BookmarkDataSource` trait for `MongoDB`.

use std::fmt::Display;

use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;
use async_graphql::futures_util::stream::StreamExt;
use crate::{
    data_source::cv::bookmark::BookmarkDataSource,
    models::cv::{Bookmark, CV, Like},
    services::cv_service::error::CVServiceError,
};

use super::MongoDB;
use mongodb::bson;
const CV_BOOKMARK_COLLECTION: &str = "cv_bookmarks";
const CV_COLLECTION: &str = "cvs";
/// Error type for `BookmarkDataSource` operations.
#[derive(Debug)]
pub enum BookmarkError {
    // fail to add bookmark
    AddBookmarkFail,

    // fail to remove bookmark
    DeleteBookmarkFail,

    // invalid cv-id
    InvalidCVId(ObjectId),

    // invalid user-id
    InvalidUserId(ObjectId),

    // cannot find bookmark
    BookmarkNotFound,

    // Bookmark already exists
    BookmarkAlreadyExists,

    // fail to do queries
    QueryFail,
}

impl Display for BookmarkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookmarkError::AddBookmarkFail => {
                write!(f, "fail to add bookmark!")
            },
            BookmarkError::DeleteBookmarkFail =>{
                write!(f, "fail to remove bookmark!")
            },
            BookmarkError::BookmarkAlreadyExists => {
                write!(f, "bookmark already exists!")
            },
            BookmarkError::BookmarkNotFound => {
                write!(f, "cannot find bookmark!")
            },
            BookmarkError::InvalidCVId(id) => {
                write!(f, "cv-id {:?} is invalid!", id)
            },
            BookmarkError::InvalidUserId(id) => {
                write!(f, "user-id {:?} is invalid!", id)
            }
            BookmarkError::QueryFail => {
                write!(f, "fail to do queries!")
            }
        }
    }
}

impl std::error::Error for BookmarkError {}

impl From<BookmarkError> for CVServiceError {
    fn from(value: BookmarkError) -> Self {
        match value{
            BookmarkError::AddBookmarkFail => CVServiceError::UpdateBookmarkFailed,
            BookmarkError::DeleteBookmarkFail => CVServiceError::UpdateBookmarkFailed,
            BookmarkError::InvalidCVId(id) => CVServiceError::InvalidId(id),
            BookmarkError::InvalidUserId(id) => CVServiceError::AuthorIdNotFound(id),
            BookmarkError::BookmarkAlreadyExists => CVServiceError::UpdateBookmarkFailed,
            BookmarkError::BookmarkNotFound => CVServiceError::BookmarkNotFound,
            BookmarkError::QueryFail => CVServiceError::QueryFail
        }
    }
}

#[async_trait::async_trait]
impl BookmarkDataSource for MongoDB {
    type Error = BookmarkError;
    async fn add_bookmark(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        let collection = self.db.collection::<Bookmark>(CV_BOOKMARK_COLLECTION);
        let filter = bson::doc!{
            "key.user_id": user_id.clone(),
            "key.cv_id": cv_id
        };
        let result_exist = collection.find_one(filter, None).await;
        match result_exist {
            Ok(bookmark_option) => {
                match bookmark_option {
                    Some(bookmark) => Err(BookmarkError::BookmarkAlreadyExists),
                    None => {
                        let bookmark = Bookmark::new(user_id, cv_id);
                        let add_result = collection.insert_one(bookmark, None).await;
                        match add_result{
                            Ok(_) => Ok(()),
                            Err(_) => Err(BookmarkError::AddBookmarkFail)
                        }

                    }
                }
            },
            Err(_) => Err(BookmarkError::QueryFail)
        }
    }

    async fn delete_bookmark(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        let collection = self.db.collection::<Bookmark>(CV_BOOKMARK_COLLECTION);
        let filter = bson::doc!{
            "key.user_id": user_id,
            "key.cv_id": cv_id
        };
        let result = collection.find_one_and_delete(filter, None).await;
        match result{
            Ok(_) => Ok(()),
            Err(_) => Err(BookmarkError::DeleteBookmarkFail)
        }
    }

    async fn get_bookmarks_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Bookmark>, Self::Error> {
        let collection = self.db.collection::<Bookmark>(CV_BOOKMARK_COLLECTION);
        let filter = bson::doc!{
            "key.user_id": user_id
        };
        let result = collection.find(filter, None).await;
        match result {
            Ok(cursor) => Ok(cursor.map(|bookmark|bookmark.unwrap()).boxed()),
            Err(_) => Err(BookmarkError::QueryFail)
        }
    }

    async fn get_bookmark(
        &self,
        user_id: ObjectId,
        cv_id: ObjectId,
    ) -> Result<Bookmark, Self::Error> {
        let collection = self.db.collection::<Bookmark>(CV_BOOKMARK_COLLECTION);
        let filter = bson::doc!{
            "key.user_id": user_id,
            "key.cv_id": cv_id
        };
        let result = collection.find_one(filter, None).await;
        match result{
            Ok(bookmark_option) => {
                match bookmark_option {
                    Some(bookmark) => Ok(bookmark),
                    None => Err(BookmarkError::BookmarkNotFound)
                }
            },
            Err(_) => Err(BookmarkError::QueryFail)
        }
    }

    async fn get_bookmarks_of_cv(
        &self,
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Bookmark, Self::Error>>, Self::Error> {
        let collection = self.db.collection::<Bookmark>(CV_BOOKMARK_COLLECTION);
        let filter = bson::doc!{
            "key.cv_id": cv_id
        };
        let result = collection.find(filter, None).await;
        match result {
            Ok(cursor) => Ok(cursor.map(|bookmark|Ok(bookmark.unwrap())).boxed()),
            Err(_) => Err(BookmarkError::QueryFail)
        }
    }

    async fn get_bookmarked_cvs_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, Self::Error>>, Self::Error> {
        let bookmark_collection = self.db.collection::<Bookmark>(CV_BOOKMARK_COLLECTION);
        let bookmark_filter = bson::doc!{
            "key.user_id": user_id
        };
        let result = bookmark_collection.find(bookmark_filter, None).await;
        match result{
            Ok(bookmark_cursor) =>{
                let list_cv_id = bookmark_cursor.map(|bookmark|bookmark.unwrap().cv_id().to_owned()).collect::<Vec<ObjectId>>().await;
                let cv_collection = self.db.collection::<CV>(CV_COLLECTION);
                let filter = bson::doc!{
                    "id": {"$in": list_cv_id}
                };
                let find_result = cv_collection.find(filter, None).await;
                match find_result {
                    Ok(cv_cursor) => Ok(cv_cursor.map(|cv|Ok(cv.unwrap())).boxed()),
                    Err(_) => Err(BookmarkError::QueryFail)
                }
            }
            Err(_) => Err(BookmarkError::QueryFail)
        }
    }

    /// This default implementation is not efficient, reimplement it if you can.
    async fn get_bookmarks_count_of_cv(&self, cv_id: ObjectId) -> Result<u64, Self::Error> {
        let collection = self.db.collection::<Bookmark>(CV_BOOKMARK_COLLECTION);
        let filter = bson::doc!{
            "key.cv_id": cv_id
        };
        let result = collection.count_documents(filter, None).await;
        match result{
            Ok(count) => Ok(count),
            Err(_) => Err(BookmarkError::QueryFail) 
        }
    }
}
