use mongodb::bson::oid::ObjectId;

use crate::data_source::{CVDataSource, CVDataSourceError, CommentDataSource, CVDetailsDataSource};
use crate::models::comment::CreateCommentInput;
use crate::models::cv::{UpdateCVInput, CV, CreateCVInput};

use crate::models::cv_details::CVDetails;
use std::boxed::Box;
use std::fmt::Debug;
use futures_core::stream::BoxStream;
use std::pin::Pin;
use tokio_stream::Stream;

use super::error::CVServiceError;

pub struct CVService {}

impl CVService {
    pub async fn create_cv(
        database: &(impl CVDataSource + std::marker::Sync),
        input: CreateCVInput,
    ) -> Result<CV, CVDataSourceError> {
        let rs = database.create_cv(input).await;
        rs.map_err(|err| err.into())
    }

    pub async fn change_title(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        title: String,
    ) -> Result<CV, CVDataSourceError> {
        let input = UpdateCVInput::builder().with_title(title).build().unwrap();
        let rs = database.find_and_update_cv(cv_id, input).await;
        rs.map_err(|err| err.into())
    }

    pub async fn change_description(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        description: String,
    ) -> Result<CV, CVDataSourceError> {
        let input = UpdateCVInput::builder()
            .with_description(description)
            .build()
            .unwrap();
        let rs = database.find_and_update_cv(cv_id, input).await;
        rs.map_err(|err| err.into())
    }

    pub async fn add_tag(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        tag: String,
    ) -> Result<CV, CVDataSourceError> {
        let cv = database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let mut tags = cv.clone().tags;
                if tags.iter().position(|x| *x == tag).is_none() {
                    tags.push(tag);
                }
                let input: UpdateCVInput =
                    UpdateCVInput::builder().with_tags(tags).build().unwrap();
                let rs = database.find_and_update_cv(cv_id.into(), input).await;
                rs.map_err(|err| err.into())
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    pub async fn remove_tag(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        tag: String,
    ) -> Result<CV, CVDataSourceError> {
        let cv = database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let mut tags = cv.clone().tags;
                if let Some(index) = tags.iter().position(|x| *x == tag) {
                    tags.remove(index);
                }
                let input: UpdateCVInput =
                    UpdateCVInput::builder().with_tags(tags).build().unwrap();
                let rs = database.find_and_update_cv(cv_id.into(), input).await;
                rs.map_err(|err| err.into())
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    pub async fn add_comment(
        database: &(impl CVDataSource + CommentDataSource + std::marker::Sync),
        cv_id: ObjectId,
        author_id: ObjectId,
        content: String,
    ) -> Result<CV, CVDataSourceError> {
        let input = CreateCommentInput {
            author: author_id.into(),
            content,
        };
        let rs = database.create_comment(input).await;
        match rs {
            Ok(comment) => {
                let rs = database.add_comment_to_cv(cv_id, comment).await;
                match rs {
                    Ok(cv) => {
                        Ok(cv)
                    }
                    Err(err) => {
                        Err(err.into())
                    }
                }
            }
            Err(_err) => {
                Err(CVDataSourceError::AddCommentFailed)
            }
        }
    }

    pub async fn remove_comment(
        database: &(impl CVDataSource + CommentDataSource + std::marker::Sync),
        cv_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<CV, CVDataSourceError> {
        let rs = database.remove_comment_from_cv(cv_id, comment_id).await;
        rs.map_err(|err| err.into())
    }
    // NOTE: The return type should `Stream<Item = CV>`
    pub async fn find_suggested_cvs<'a>(&'a self, database: &'a(impl CVDetailsDataSource + std::marker::Sync), cv_details: CVDetails) -> Result<Pin<Box<dyn Stream<Item = CV>>>, CVServiceError> {
        let stream = database.get_cvs_by_filter(cv_details).await;
        stream.map_err(|err|err.into())
    }
}
