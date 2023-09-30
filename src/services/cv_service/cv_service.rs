use mongodb::bson::oid::ObjectId;

use crate::data_source::{CVDataSource, CVDataSourceError};
use crate::models::cv::{UpdateCVInput, CV};
use std::boxed::Box;
use std::fmt::Debug;

pub struct CVService {}

impl CVService {
    pub async fn change_title(
        title: String,
        cv_id: ObjectId,
        database: &(impl CVDataSource + std::marker::Sync),
    ) -> Result<CV, CVDataSourceError> {
        let cv = database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let input: UpdateCVInput = UpdateCVInput {
                    id: cv_id.into(),
                    author_id: cv.author_id,
                    title: Some(title),
                    description: None,
                    tags: None,
                };
                let rs = database.update_cv_info(input).await;
                return Ok(rs?);
            }
            Err(_) => {
                return Err(CVDataSourceError::IdNotFound(cv_id));
            }
        }
    }

    pub async fn change_item_interactions(&mut self) -> Result<CV, CVDataSourceError> {
        todo!()
    }
    
    pub async fn change_description(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        description: String,
    ) -> Result<CV, CVDataSourceError> {
        let cv = database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let input: UpdateCVInput = UpdateCVInput {
                    id: cv_id.into(),
                    author_id: cv.author_id.into(),
                    title: None,
                    description: Some(description),
                    tags: None,
                };
                let rs = database.update_cv_info(input).await;
                return Ok(rs?);
            }
            Err(_) => {
                return Err(CVDataSourceError::IdNotFound(cv_id));
            }
        }
    }
    pub async fn add_tags(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        tag: String,
    ) -> Result<CV, CVDataSourceError> {
        let cv = database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let mut tags = cv.clone().tags;
                tags.push(tag);
                let input: UpdateCVInput = UpdateCVInput {
                    id: cv_id.into(),
                    author_id: cv.author_id.into(),
                    title: None,
                    description: None,
                    tags: Some(tags),
                };
                let rs = database.update_cv_info(input).await;
                return Ok(rs?);
            }
            Err(_) => {
                return Err(CVDataSourceError::IdNotFound(cv_id));
            }
        }
    }
    pub async fn remove_tags(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        tag: String,
    ) -> Result<CV, CVDataSourceError> {
        let cv = database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let mut tags = cv.clone().tags;
                tags.remove(tags.iter().position(|x| *x == tag).unwrap());
                let input: UpdateCVInput = UpdateCVInput {
                    id: cv_id.into(),
                    author_id: cv.author_id.into(),
                    title: None,
                    description: None,
                    tags: Some(tags),
                };
                let rs = database.update_cv_info(input).await;
                return Ok(rs?);
            }
            Err(_) => {
                return Err(CVDataSourceError::IdNotFound(cv_id));
            }
        }
    }
}
