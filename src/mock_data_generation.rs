use async_graphql::futures_util::future;

use crate::{
    data_source::{mongo::MongoDB, CVDataSource, UserDataSource},
    models::{
        comment::CreateCommentInput,
        cv::{create_cv_input::CreateCVInputBuilder, CreateCVInput},
        education::Education,
        users::{create_user_input::CreateUserInputBuilder, CreateUserInput},
    },
    object_id::ScalarObjectId,
};

fn generate_users() -> Vec<CreateUserInput> {
    [
        CreateUserInputBuilder::default()
            .with_username("user1")
            .with_password("password1")
            .with_primary_email("user1@gmail.com")
            .with_first_name("firstname1")
            .with_last_name("lastname1")
            .with_education(Education {
                institution: "hcmus".to_string(),
                course: Some("cs101".to_string()),
                degree: Some("bachelor".to_string()),
            })
            .with_education(Education {
                institution: "hcmus".to_string(),
                course: Some("cs103".to_string()),
                degree: Some("bachelor".to_string()),
            })
            .with_other_mail("othermail1@gmail.com")
            .with_other_mail("otherothermail1@gmail.com")
            .build()
            .unwrap(),
        CreateUserInputBuilder::default()
            .with_username("user2")
            .with_password("password2")
            .with_primary_email("user2@gmail.com")
            .with_first_name("firstname2")
            .with_last_name("lastname2")
            .with_education(Education {
                institution: "hcmus".to_string(),
                course: Some("mth252".to_string()),
                degree: Some("bachelor".to_string()),
            })
            .with_education(Education {
                institution: "hcmus".to_string(),
                course: Some("cs303".to_string()),
                degree: Some("bachelor".to_string()),
            })
            .with_other_mail("othermail2@gmail.com")
            .with_other_mail("otherothermail2@gmail.com")
            .build()
            .unwrap(),
        CreateUserInputBuilder::default()
            .with_username("user3")
            .with_password("password3")
            .with_primary_email("user3@gmail.com")
            .with_first_name("firstname3")
            .with_last_name("lastname3")
            .with_education(Education {
                institution: "hcmus".to_string(),
                course: Some("cs202".to_string()),
                degree: Some("bachelor".to_string()),
            })
            .with_education(Education {
                institution: "hcmus".to_string(),
                course: Some("cs401".to_string()),
                degree: Some("bachelor".to_string()),
            })
            .with_other_mail("othermail3@gmail.com")
            .with_other_mail("otherothermail3@gmail.com")
            .build()
            .unwrap(),
    ]
    .into()
}

fn generate_cvs(author_ids: [ScalarObjectId; 3]) -> Vec<CreateCVInput> {
    [
        CreateCVInputBuilder::default()
            .with_author_id(author_ids[0])
            .with_title("cv1")
            .with_tag("dummy_tag1")
            .with_tag("dummy_tag2")
            .with_tag("dummy_tag3")
            .build()
            .unwrap(),
        CreateCVInputBuilder::default()
            .with_author_id(author_ids[1])
            .with_title("cv1")
            .with_tag("dummy_tag1")
            .with_tag("dummy_tag2")
            .with_tag("dummy_tag3")
            .build()
            .unwrap(),
        CreateCVInputBuilder::default()
            .with_author_id(author_ids[2])
            .with_title("cv1")
            .with_tag("dummy_tag1")
            .with_tag("dummy_tag10")
            .with_tag("dummy_tag7")
            .build()
            .unwrap(),
    ]
    .into()
}

fn generate_comments(author_ids: [ScalarObjectId; 3]) -> Vec<CreateCommentInput> {
    [
        CreateCommentInput {
            author: author_ids[0],
            content: "comment1".to_string(),
        },
        CreateCommentInput {
            author: author_ids[1],
            content: "comment2".to_string(),
        },
        CreateCommentInput {
            author: author_ids[2],
            content: "comment3".to_string(),
        },
    ]
    .into()
}

fn generate_replies(author_ids: [ScalarObjectId; 2]) -> Vec<CreateCommentInput> {
    [
        CreateCommentInput {
            author: author_ids[0],
            content: "reply1".to_string(),
        },
        CreateCommentInput {
            author: author_ids[1],
            content: "reply2".to_string(),
        },
    ]
    .into()
}

pub async fn generate_mock_data(mongodb: &MongoDB) {
    let users = generate_users();
    let author_ids = users
        .iter()
        .map(|user| async { mongodb.create_user(user.clone()).await.unwrap().id });
    let author_ids = future::join_all(author_ids).await;
    let cvs = generate_cvs(author_ids.clone().try_into().unwrap());
    let cv_ids = cvs
        .iter()
        .map(|cv| async { mongodb.create_cv(cv.clone()).await.unwrap().id });
    let cv_ids = future::join_all(cv_ids).await;
    let comments = generate_comments([author_ids[0], author_ids[1], author_ids[2]]);
    // let comment_ids = comments
    //     .iter()
    //     .map(|comment| async { mongodb.create(*comment).await.unwrap().id });
    // let comment_ids = future::join_all(comment_ids).await;
    // let replies = generate_replies([author_ids[0], author_ids[1]]);
    // let reply_ids = replies
    //     .iter()
    //     .map(|reply| async { mongodb.create_comment(*reply).await.unwrap().id });
    // let reply_ids = future::join_all(reply_ids).await;
}
