use async_graphql::{Context, ErrorExtensions, Object};

use crate::{
    data_source::mongo::{MongoDB, MongoForTesting},
    models::users::{CreateUserInput, User},
    object_id::ScalarObjectId,
    services::{
        auth_service::AuthService,
        cv_service::{
            bookmark_service::BookmarkService, comment_service::CommentService,
            cv_service::CVService, like_service::LikeService as CVLikeService,
            share_service::ShareService,
        },
        user_service::UserService,
    },
};

use super::{authorization, GqlResult};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn user_register(&self, ctx: &Context<'_>, new_user: CreateUserInput) -> GqlResult<User> {
        let rs = AuthService::register(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            new_user,
        )
        .await;
        match rs {
            Ok(user) => Ok(user),
            Err(e) => Err(e.extend()),
        }
    }

    async fn send_friend_request(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        friend_id: ScalarObjectId,
        message: Option<String>,
    ) -> GqlResult<bool> {
        let rs = UserService::send_friend_request(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            user_id.into(),
            friend_id.into(),
            message,
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn accept_friend_request(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        friend_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let rs = UserService::accept_friend_request(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            user_id.into(),
            friend_id.into(),
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn decline_friend_request(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        friend_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let rs = UserService::reject_friend_request(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            user_id.into(),
            friend_id.into(),
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn create_cv(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        title: String,
        description: String,
    ) -> GqlResult<bool> {
        let rs = CVService::create_cv(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            user_id.into(),
            title,
            description,
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn delete_cv(
        &self,
        ctx: &Context<'_>,
        cv_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let rs = CVService::delete_cv(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            cv_id.into(),
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn change_cv_title(
        &self,
        ctx: &Context<'_>,
        cv_id: ScalarObjectId,
        title: String,
    ) -> GqlResult<bool> {
        let rs = CVService::change_title(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            cv_id.into(),
            title,
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn change_cv_description(
        &self,
        ctx: &Context<'_>,
        cv_id: ScalarObjectId,
        description: String,
    ) -> GqlResult<bool> {
        let rs = CVService::change_description(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            cv_id.into(),
            description,
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn add_one_tag(
        &self,
        ctx: &Context<'_>,
        cv_id: ScalarObjectId,
        tag: String,
    ) -> GqlResult<bool> {
        let rs = CVService::add_tag(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            cv_id.into(),
            tag,
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn remove_one_tag(
        &self,
        ctx: &Context<'_>,
        cv_id: ScalarObjectId,
        tag: String,
    ) -> GqlResult<bool> {
        let rs = CVService::remove_tag(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            cv_id.into(),
            tag,
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn add_comment(
        &self,
        ctx: &Context<'_>,
        cv_id: ScalarObjectId,
        author_id: ScalarObjectId,
        content: String,
    ) -> GqlResult<bool> {
        let rs = CVService::add_comment(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            cv_id.into(),
            author_id.into(),
            content,
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn remove_comment(
        &self,
        ctx: &Context<'_>,
        cv_id: ScalarObjectId,
        comment_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let rs = CVService::remove_comment(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            cv_id.into(),
            comment_id.into(),
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn update_content_comment(
        &self,
        ctx: &Context<'_>,
        comment_id: ScalarObjectId,
        content: String,
    ) -> GqlResult<bool> {
        let rs = CommentService::update_content_comment(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            comment_id.into(),
            content,
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn add_reply_to_comment(
        &self,
        ctx: &Context<'_>,
        comment_id: ScalarObjectId,
        author_id: ScalarObjectId,
        content: String,
    ) -> GqlResult<bool> {
        let rs = CommentService::add_reply_comment(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            comment_id.into(),
            author_id.into(),
            content,
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn remove_reply_from_comment(
        &self,
        ctx: &Context<'_>,
        comment_id: ScalarObjectId,
        reply_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let rs = CommentService::remove_reply_comment(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            comment_id.into(),
            reply_id.into(),
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn like_comment(
        &self,
        ctx: &Context<'_>,
        comment_id: ScalarObjectId,
        user_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let rs = CommentService::add_like_comment(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            user_id.into(),
            comment_id.into(),
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn unlike_comment(
        &self,
        ctx: &Context<'_>,
        comment_id: ScalarObjectId,
        user_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let rs = CommentService::remove_like_comment(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            user_id.into(),
            comment_id.into(),
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn share_cv(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        cv_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        authorization(ctx)?;
        let rs = ShareService::share_cv(db, user_id.into(), cv_id.into()).await;
        rs.map_err(|e| e.into()).map(|_| true)
    }

    async fn unshare_cv(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        cv_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        authorization(ctx)?;
        let rs = ShareService::unshare_cv(db, user_id.into(), cv_id.into()).await;
        rs.map_err(|e| e.into()).map(|_| true)
    }

    async fn like_cv(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        cv_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        authorization(ctx)?;
        let rs = CVLikeService::like_cv(db, user_id.into(), cv_id.into()).await;
        rs.map_err(|e| e.into()).map(|_| true)
    }

    async fn unlike_cv(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        cv_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        authorization(ctx)?;
        let rs = CVLikeService::unlike_cv(db, user_id.into(), cv_id.into()).await;
        rs.map_err(|e| e.into()).map(|_| true)
    }

    async fn bookmark_cv(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        cv_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        authorization(ctx)?;
        let rs = BookmarkService::bookmark_cv(db, user_id.into(), cv_id.into()).await;
        rs.map_err(|e| e.into()).map(|_| true)
    }

    async fn unbookmark_cv(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        cv_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        authorization(ctx)?;
        let rs = BookmarkService::unbookmark_cv(db, user_id.into(), cv_id.into()).await;
        rs.map_err(|e| e.into()).map(|_| true)
    }
}
