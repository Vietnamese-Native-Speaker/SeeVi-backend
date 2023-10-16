use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::cv::share::ShareDataSource,
    models::cv::{Like, CV},
};

use super::error::CVServiceError;

pub struct LikeService;

impl LikeService {
    pub async fn like_cv(
        db: &impl ShareDataSource,
        user_id: ObjectId,
        cv_id: ObjectId,
    ) -> Result<(), CVServiceError> {
        todo!()
    }

    pub async fn unlike_cv(
        db: &impl ShareDataSource,
        user_id: ObjectId,
        cv_id: ObjectId,
    ) -> Result<(), CVServiceError> {
        todo!()
    }

    pub async fn get_likes(
        db: &impl ShareDataSource,
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Like, CVServiceError>>, CVServiceError> {
        todo!()
    }

    pub async fn get_likes_count(
        db: &impl ShareDataSource,
        cv_id: ObjectId,
    ) -> Result<i32, CVServiceError> {
        todo!()
    }
}
