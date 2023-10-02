use mongodb::bson::oid::ObjectId;

use crate::data_source::{CVDataSource, CVDataSourceError, CommentDataSource};
use crate::models::comment::CreateCommentInput;
use crate::models::cv::{UpdateCVInput, CV};

pub struct CVService {}

impl CVService {
    pub async fn change_title(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        title: String
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

    pub async fn add_tag(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        tag: String,
    ) -> Result<CV, CVDataSourceError> {
        let cv = database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let mut tags = cv.clone().tags;
                tags.push(tag.clone());
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

    pub async fn add_tags(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        to_add_tags: Vec<String>,
    ) -> Result<CV, CVDataSourceError> {
        let cv = database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let mut tags = cv.clone().tags;
                for tag in to_add_tags {
                    tags.push(tag.clone());
                }
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

    pub async fn remove_tag(
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

    pub async fn remove_tags(
        database: &(impl CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
        to_del_tags: Vec<String>,
    ) -> Result<CV, CVDataSourceError> {
        let cv = database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let mut tags = cv.clone().tags;
                for tag in to_del_tags {
                    if tags.iter().position(|x| *x == tag).is_some() {
                        tags.remove(tags.iter().position(|x| *x == tag).unwrap());
                    }
                }
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

    pub async fn add_comment(
        cv_database: &(impl CVDataSource + std::marker::Sync),
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        cv_id: ObjectId,
        author_id: ObjectId,
        content: String,
    ) -> Result<CV, CVDataSourceError> {
        let cv = cv_database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let input: CreateCommentInput = CreateCommentInput {
                    author: author_id.into(),
                    content
                };
                let rs = cmt_database.create_comment(input).await;
                match rs {
                    Ok(rs) => {
                        let rs = cv_database.add_comment_to_cv(cv.id.into(), rs.id.into()).await;
                        match rs {
                            Ok(rs) => {
                                return Ok(rs);
                            }
                            Err(_) => {
                                return Err(CVDataSourceError::AddCommentFailed);
                            }
                        }
                    }
                    Err(_) => {
                        return Err(CVDataSourceError::AddCommentFailed);
                    }
                }
            }
            Err(_) => {
                return Err(CVDataSourceError::IdNotFound(cv_id));
            }
        }
    }
    pub async fn remove_comment(
        cv_database: &(impl CVDataSource + std::marker::Sync),
        cmt_database: &(impl CommentDataSource + std::marker::Sync),
        cv_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<CV, CVDataSourceError> {
        let cv = cv_database.get_cv_by_id(cv_id).await;
        match cv {
            Ok(cv) => {
                let rs = cmt_database.delete_comment(comment_id).await;
                match rs {
                    Ok(_) => {
                        let rs = cv_database.remove_comment_from_cv(cv.id.into(), comment_id).await;
                        match rs {
                            Ok(rs) => {
                                return Ok(rs);
                            }
                            Err(_) => {
                                return Err(CVDataSourceError::RemoveCommentFailed);
                            }
                        }
                    }
                    Err(_) => {
                        return Err(CVDataSourceError::RemoveCommentFailed);
                    }
                }
            }
            Err(_) => {
                return Err(CVDataSourceError::IdNotFound(cv_id));
            }
        }
    }
}
