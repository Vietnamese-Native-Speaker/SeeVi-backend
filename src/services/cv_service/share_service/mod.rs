use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::cv::share::ShareDataSource,
    models::cv::{interactions::Share, CV},
};

use super::error::CVServiceError;

pub struct ShareService;

impl ShareService {
    pub async fn share_cv(
        db: &impl ShareDataSource,
        user_id: ObjectId,
        cv_id: ObjectId,
    ) -> Result<(), CVServiceError> {
        todo!()
    }

    pub async fn unshare_cv(
        db: &impl ShareDataSource,
        user_id: ObjectId,
        cv_id: ObjectId,
    ) -> Result<(), CVServiceError> {
        todo!()
    }

    pub async fn get_shared_cvs_of_user(
        db: &impl ShareDataSource,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, CVServiceError>>, CVServiceError> {
        todo!()
    }

    pub async fn get_shares(
        db: &impl ShareDataSource,
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Share, CVServiceError>>, CVServiceError> {
        todo!()
    }
}
