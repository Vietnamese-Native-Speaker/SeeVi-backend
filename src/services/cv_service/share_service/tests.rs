use super::ShareService;
use crate::{
    models::{
        cv::{create_cv_input::CreateCVInputBuilder, interactions::Share, Like as CVLike, CV},
        sex::Sex,
        users::create_user_input::CreateUserInputBuilder,
    },
    services::{cv_service::cv_service::CVService, user_service::UserService},
};
use async_graphql::futures_util::{self, StreamExt};
use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::cv,
    services::tests::{CVInteractionError, MockDatabase},
};

#[async_trait::async_trait]
impl cv::share::ShareDataSource for MockDatabase {
    type Error = CVInteractionError;

    async fn get_shares_of_cv(
        &self,
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Share, Self::Error>>, Self::Error> {
        let shares = self.cv_shares.lock().unwrap().clone();
        let stream = futures_util::stream::iter(shares.into_iter());
        let stream = stream
            .filter(move |share| {
                let share = share.clone();
                async move { share.cv_id() == &cv_id }
            })
            .map(|share| Ok(share));
        Ok(stream.boxed())
    }

    async fn get_shared_cvs_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, Self::Error>>, Self::Error> {
        let shares = self
            .cv_shares
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|share| share.user_id() == &user_id)
            .collect::<Vec<_>>();
        let cvs = self
            .cvs
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|cv| shares.iter().any(|share| *share.cv_id() == cv.id.into()))
            .collect::<Vec<_>>();
        Ok(futures_util::stream::iter(cvs.into_iter())
            .map(|cv| Ok(cv))
            .boxed())
    }

    async fn add_share(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        let mut shares = self.cv_shares.lock().unwrap();
        for share in shares.iter() {
            if *share.user_id() == user_id && *share.cv_id() == cv_id {
                return Err(CVInteractionError::AlreadyExists);
            }
        }
        shares.push(Share::new(user_id, cv_id));
        Ok(())
    }

    async fn delete_share(
        &self,
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<(), Self::Error> {
        let mut shares = self.cv_shares.lock().unwrap();
        for share in shares.iter_mut() {
            if *share.user_id() == user_id && *share.cv_id() == comment_id {
                shares.retain(|share| *share.user_id() != user_id || *share.cv_id() != comment_id);
                return Ok(());
            }
        }
        Err(CVInteractionError::NotFound)
    }

    async fn get_shares_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Share>, Self::Error> {
        let shares = self.cv_shares.lock().unwrap().clone();
        let stream = futures_util::stream::iter(shares.into_iter());
        let stream = stream.filter(move |share| {
            let share = share.clone();
            async move { *share.user_id() == user_id }
        });
        Ok(stream.map(|share| share).boxed())
    }

    async fn get_share(
        &self,
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<Share, Self::Error> {
        let shares = self.cv_shares.lock().unwrap();
        for share in shares.iter() {
            if *share.user_id() == user_id && *share.cv_id() == comment_id {
                return Ok(share.clone());
            }
        }
        Err(CVInteractionError::NotFound)
    }
}

#[tokio::test]
async fn basic() {
    let db = MockDatabase::new();
    let dummy_user_id = ObjectId::new();
    let dummy_cv_id = ObjectId::new();
    ShareService::share_cv(&db, dummy_user_id, dummy_cv_id)
        .await
        .expect_err("Should fail as no cv or user available");
    let user_id = UserService::create_user(
        &db,
        CreateUserInputBuilder::default()
            .with_primary_email("email1@email.com")
            .with_username("testuser1")
            .with_password("testuser1")
            .with_sex(Sex::Male)
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .id;
    let user_id2 = UserService::create_user(
        &db,
        CreateUserInputBuilder::default()
            .with_primary_email("email2@email.com")
            .with_username("testuser2")
            .with_password("testuser2")
            .with_sex(Sex::Male)
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .id;
    let user_id3 = UserService::create_user(
        &db,
        CreateUserInputBuilder::default()
            .with_primary_email("email3@email.com")
            .with_username("testuser3")
            .with_password("testuser3")
            .with_sex(Sex::Female)
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .id;
    let cv_id = CVService::create_cv(
        &db,
        user_id.into(),
        "Test CV".to_string(),
        "Test CV".to_string(),
    )
    .await
    .unwrap()
    .id;
    ShareService::share_cv(&db, user_id.into(), cv_id.into())
        .await
        .expect_err("Cannot share your own CV");
    ShareService::share_cv(&db, user_id2.into(), cv_id.into())
        .await
        .unwrap();
    ShareService::share_cv(&db, user_id3.into(), cv_id.into())
        .await
        .unwrap();

    let likes_count = ShareService::get_shares_count_of_cv(&db, cv_id.into())
        .await
        .unwrap();
    assert_eq!(likes_count, 2);
    let shares = ShareService::get_shares_of_cv(&db, cv_id.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    let user_ids = shares
        .iter()
        .map(|share| share.as_ref().unwrap().user_id())
        .collect::<Vec<_>>();
    assert_eq!(user_ids.len(), 2);
    assert!(!user_ids.contains(&&user_id.into()));
    assert!(user_ids.contains(&&user_id2.into()));
    assert!(user_ids.contains(&&user_id3.into()));
    let shared_cvs = ShareService::get_shared_cvs_of_user(&db, user_id2.into())
        .await
        .unwrap();
    let shared_cvs = shared_cvs.collect::<Vec<_>>().await;
    assert_eq!(shared_cvs.len(), 1);

    ShareService::unshare_cv(&db, user_id2.into(), cv_id.into())
        .await
        .unwrap();

    let shares_count = ShareService::get_shares_count_of_cv(&db, cv_id.into())
        .await
        .unwrap();
    assert_eq!(1, shares_count);
    let shares = ShareService::get_shares_of_cv(&db, cv_id.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    let user_ids = shares
        .iter()
        .map(|cv| cv.as_ref().unwrap().user_id())
        .collect::<Vec<_>>();
    assert_eq!(user_ids.len(), 1);
    assert!(!user_ids.contains(&&user_id.into()));
    assert!(!user_ids.contains(&&user_id2.into()));
    assert!(user_ids.contains(&&user_id3.into()));
    let shared_cvs = ShareService::get_shared_cvs_of_user(&db, user_id2.into())
        .await
        .unwrap();
    assert_eq!(0, shared_cvs.count().await);
}
