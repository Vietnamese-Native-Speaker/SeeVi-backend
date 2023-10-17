use crate::{
    models::{
        cv::{create_cv_input::CreateCVInputBuilder, Bookmark, Like as CVLike, CV},
        users::create_user_input::CreateUserInputBuilder,
    },
    services::{
        cv_service::{bookmark_service::BookmarkService, cv_service::CVService},
        user_service::UserService,
    },
};
use async_graphql::futures_util::{self, StreamExt};
use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::cv,
    services::tests::{CVInteractionError, MockDatabase},
};

#[async_trait::async_trait]
impl cv::bookmark::BookmarkDataSource for MockDatabase {
    type Error = CVInteractionError;

    async fn add_bookmark(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        let mut bookmarks = self.cv_bookmarks.lock().unwrap();
        for bookmark in bookmarks.iter() {
            if *bookmark.user_id() == user_id && *bookmark.cv_id() == cv_id {
                return Err(CVInteractionError::AlreadyExists);
            }
        }
        bookmarks.push(Bookmark::new(user_id, cv_id));
        Ok(())
    }

    async fn delete_bookmark(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        let mut bookmarks = self.cv_bookmarks.lock().unwrap();
        for bookmark in bookmarks.iter() {
            if *bookmark.user_id() == user_id && *bookmark.cv_id() == cv_id {
                bookmarks.retain(|bookmark| {
                    *bookmark.user_id() != user_id || *bookmark.cv_id() != cv_id
                });
                return Ok(());
            }
        }
        Err(CVInteractionError::NotFound)
    }

    async fn get_bookmarks_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Bookmark>, Self::Error> {
        let bookmarks = self.cv_bookmarks.lock().unwrap().clone();
        let stream = futures_util::stream::iter(bookmarks.into_iter());
        let stream = stream.filter(move |bookmark| {
            let bookmark = bookmark.clone();
            async move { *bookmark.user_id() == user_id }
        });
        Ok(stream.map(|bookmark| bookmark).boxed())
    }

    async fn get_bookmark(
        &self,
        user_id: ObjectId,
        cv_id: ObjectId,
    ) -> Result<Bookmark, Self::Error> {
        let bookmarks = self.cv_bookmarks.lock().unwrap().clone();
        for bookmark in bookmarks.iter() {
            if *bookmark.user_id() == user_id && *bookmark.cv_id() == cv_id {
                return Ok(bookmark.clone());
            }
        }
        Err(CVInteractionError::NotFound)
    }

    async fn get_bookmarks_of_cv(
        &self,
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Bookmark, Self::Error>>, Self::Error> {
        let bookmarks = self.cv_bookmarks.lock().unwrap().clone();
        let stream = futures_util::stream::iter(bookmarks.into_iter());
        let stream = stream.filter(move |bookmark| {
            let bookmark = bookmark.clone();
            async move { *bookmark.cv_id() == cv_id }
        });
        Ok(stream.map(|bookmark| Ok(bookmark)).boxed())
    }

    async fn get_bookmarked_cvs_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, Self::Error>>, Self::Error> {
        let cvs = self.cvs.lock().unwrap().clone();
        let bookmarks = self.cv_bookmarks.lock().unwrap().clone();
        let bookmarks = bookmarks
            .into_iter()
            .filter(move |bookmark| *bookmark.user_id() == user_id)
            .map(|bookmark| *bookmark.cv_id())
            .collect::<Vec<_>>();
        let cvs = cvs
            .into_iter()
            .filter_map(move |cv| {
                if bookmarks.contains(&&cv.id.into()) {
                    Some(Ok(cv))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let stream = futures_util::stream::iter(cvs);
        Ok(stream.boxed())
    }
}

#[tokio::test]
async fn basic() {
    let db = MockDatabase::new();
    let dummy_user_id = ObjectId::new();
    let dummy_cv_id = ObjectId::new();
    BookmarkService::bookmark_cv(&db, dummy_user_id, dummy_cv_id)
        .await
        .expect_err("Should fail as no cv or user available");
    let user_id = UserService::create_user(
        &db,
        CreateUserInputBuilder::default()
            .with_primary_email("email1@email.com")
            .with_username("testuser1")
            .with_password("testuser1")
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
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .id;
    let cv_id = CVService::create_cv(
        &db,
        CreateCVInputBuilder::default()
            .with_title("Test CV")
            .with_author_id(user_id)
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .id;
    let cv_id2 = CVService::create_cv(
        &db,
        CreateCVInputBuilder::default()
            .with_title("Test CV 2")
            .with_author_id(user_id)
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .id;
    BookmarkService::bookmark_cv(&db, user_id2.into(), cv_id.into())
        .await
        .unwrap();
    BookmarkService::bookmark_cv(&db, user_id2.into(), cv_id2.into())
        .await
        .unwrap();
    BookmarkService::bookmark_cv(&db, user_id3.into(), cv_id.into())
        .await
        .unwrap();

    let likes_count = BookmarkService::get_bookmark_count_of_cv(&db, cv_id.into())
        .await
        .unwrap();
    assert_eq!(likes_count, 2);
    let bookmarks = BookmarkService::get_bookmarked_cvs_of_user(&db, user_id.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    assert_eq!(0, bookmarks.len());
    let bookmarks = BookmarkService::get_bookmarked_cvs_of_user(&db, user_id2.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    let cv_ids = bookmarks
        .iter()
        .map(|cv| cv.as_ref().unwrap().id)
        .collect::<Vec<_>>();
    assert_eq!(2, cv_ids.len());
    assert!(cv_ids.contains(&&cv_id.into()));
    assert!(cv_ids.contains(&&cv_id2.into()));

    let bookmarks = BookmarkService::get_bookmarked_cvs_of_user(&db, user_id3.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    let cv_ids = bookmarks
        .iter()
        .map(|cv| cv.as_ref().unwrap().id)
        .collect::<Vec<_>>();
    assert_eq!(1, bookmarks.len());
    assert!(cv_ids.contains(&&cv_id.into()));
    assert!(!cv_ids.contains(&&cv_id2.into()));

    BookmarkService::unbookmark_cv(&db, user_id.into(), cv_id.into())
        .await
        .expect_err("Should fail as user has not bookmarked this cv");
    BookmarkService::unbookmark_cv(&db, user_id2.into(), cv_id.into())
        .await
        .unwrap();

    let likes_count = BookmarkService::get_bookmark_count_of_cv(&db, cv_id.into())
        .await
        .unwrap();
    assert_eq!(1, likes_count);
    let bookmarks = BookmarkService::get_bookmarked_cvs_of_user(&db, user_id.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    assert_eq!(0, bookmarks.len(), "User 1 should have no bookmarks");
    let bookmarks = BookmarkService::get_bookmarked_cvs_of_user(&db, user_id2.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    let cv_ids = bookmarks
        .iter()
        .map(|cv| cv.as_ref().unwrap().id)
        .collect::<Vec<_>>();
    assert_eq!(1, cv_ids.len());
    assert!(!cv_ids.contains(&&cv_id.into()), "User 2 unbokmarked cv 1");
    assert!(cv_ids.contains(&&cv_id2.into()));

    let bookmarks = BookmarkService::get_bookmarked_cvs_of_user(&db, user_id3.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    let cv_ids = bookmarks
        .iter()
        .map(|cv| cv.as_ref().unwrap().id)
        .collect::<Vec<_>>();
    assert_eq!(1, bookmarks.len());
    assert!(cv_ids.contains(&&cv_id.into()));
    assert!(
        !cv_ids.contains(&&cv_id2.into()),
        "User 3 has not bookmarked cv 2"
    );
}
