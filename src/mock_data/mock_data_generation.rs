use std::sync::Arc;

use futures::{future::join_all, join, FutureExt};

use crate::{
    data_source::{mongo::{MongoForTesting, MongoDB}, CVDataSource, CommentDataSource, UserDataSource},
    models::comment::create_comment_input::CreateCommentInputBuilder,
    object_id::ScalarObjectId,
};

use super::{
    comments::{COMMENT_INPUTS, REPLY_INPUTS},
    cvs::CV_INPUTS,
    users::USER_INPUTS,
};

async fn add_cvs_to_user(
    mongodb: MongoDB,
    user_id: ScalarObjectId,
) -> Vec<ScalarObjectId> {
    let mut cv_inputs = CV_INPUTS.clone();
    let handlers = cv_inputs.iter_mut().map(|cv_input| {
        let cv = cv_input.clone().with_author_id(user_id).build().unwrap();
        let mongodb = mongodb.clone();
        tokio::spawn(async move {
            let cv = mongodb.create_cv(cv).await.unwrap();
            cv.id
        })
    });
    let rs = futures::future::join_all(handlers).await;
    rs.into_iter().map(|id| id.unwrap()).collect::<Vec<_>>()
}

async fn add_comment_to_cv(
    mongodb: MongoDB,
    cv_id: ScalarObjectId,
    user_id: ScalarObjectId,
    mut comment_buider: CreateCommentInputBuilder,
) -> ScalarObjectId {
    let comment = comment_buider.author(user_id).build().unwrap();
    let rs = mongodb.create_comment(comment).await.unwrap();
    mongodb
        .add_comment_to_cv(cv_id.into(), rs.clone().into())
        .await
        .unwrap();
    rs.id.into()
}

async fn reply_to_comment(
    mongodb: MongoDB,
    comment_id: ScalarObjectId,
    reply_user_id: ScalarObjectId,
    mut comment_buider: CreateCommentInputBuilder,
) -> ScalarObjectId {
    let reply = comment_buider.author(reply_user_id).build().unwrap();
    let rs = mongodb.create_comment(reply).await.unwrap();
    mongodb
        .add_reply_to_comment(comment_id.into(), rs.id.clone().into())
        .await
        .unwrap();
    rs.id.into()
}

pub async fn populate_mocked_data(mongodb: MongoDB) {
    let mongodb = mongodb.clone();
    let rs = USER_INPUTS.iter().map(|user_input| {
        let mongodb = mongodb.clone();
        let input = user_input.clone().build().unwrap();
        tokio::spawn(async move {
            let user = mongodb.create_user(input).await.unwrap();
            user.id
        })
    });
    let rs = futures::future::join_all(rs).await;
    let user_ids = rs.into_iter().map(|id| id.unwrap()).collect::<Vec<_>>();

    let db_ref = mongodb.clone();
    add_cvs_to_user(db_ref.clone(), user_ids[0].clone())
        .then(|cv_ids| {
            let user_ids = user_ids.clone();
            let cv_ids = cv_ids.clone();
            add_comment_to_cv(
                mongodb.clone(),
                cv_ids[0],
                user_ids[1],
                COMMENT_INPUTS[0].clone(),
            )
            .then(move |commend_id| {
                let commend_id = commend_id.clone();
                reply_to_comment(
                    db_ref.clone(),
                    commend_id,
                    user_ids[2],
                    REPLY_INPUTS[0].clone(),
                )
            })
        })
        .await;
    let db_ref = mongodb.clone();
    add_cvs_to_user(db_ref.clone(), user_ids[1].clone())
        .then(|cv_ids| {
            join_all([
                add_comment_to_cv(
                    mongodb.clone(),
                    cv_ids[0],
                    user_ids[1],
                    COMMENT_INPUTS[0].clone(),
                ),
                add_comment_to_cv(
                    mongodb.clone(),
                    cv_ids[1],
                    user_ids[2],
                    COMMENT_INPUTS[0].clone(),
                ),
                add_comment_to_cv(
                    mongodb.clone(),
                    cv_ids[2],
                    user_ids[3],
                    COMMENT_INPUTS[0].clone(),
                ),
            ])
        })
        .await;
    let db_ref = mongodb.clone();
    add_cvs_to_user(db_ref.clone(), user_ids[2].clone())
        .then(|cv_ids| {
            join_all([
                add_comment_to_cv(
                    mongodb.clone(),
                    cv_ids[0],
                    user_ids[1],
                    COMMENT_INPUTS[0].clone(),
                ),
                add_comment_to_cv(
                    mongodb.clone(),
                    cv_ids[1],
                    user_ids[2],
                    COMMENT_INPUTS[0].clone(),
                ),
                add_comment_to_cv(
                    mongodb.clone(),
                    cv_ids[2],
                    user_ids[3],
                    COMMENT_INPUTS[0].clone(),
                ),
            ])
        })
        .await;
}
