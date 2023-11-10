use mongodb::bson::{oid::ObjectId, Uuid};

use crate::{models::{
    cv::{create_cv_input::CreateCVInputBuilder, CreateCVInput},
    education::Education,
    experience::ExperienceBuilder,
    sex::Sex,
    users::{create_user_input::CreateUserInputBuilder, CreateUserInput}, comment::Comment,
}, object_id::ScalarObjectId, common::DateTime};

mod bookmark;
mod comment_data_source;
mod cv_bookmark_data_source;
mod cv_data_source;
mod cv_like_data_source;
mod cv_share_data_source;
mod like;
mod user_data_source;

fn create_demo_user_input() -> CreateUserInput {
    let id = Uuid::new();
    CreateUserInputBuilder::default()
        .with_password("password")
        .with_username("username")
        .with_first_name("first_name")
        .with_last_name("last_name")
        .with_country("country")
        .with_skill("skill")
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_other_mail("other_mails2")
        .with_education(Education {
            school: "school 1".to_string(),
            major: "major 1".to_string(),
            minor: Some("minor 1".to_string()),
            degree: "degree 1".to_string(),
            start_date: None,
            end_date: None,
        })
        .with_education(Education {
            school: "school 2".to_string(),
            major: "major 2".to_string(),
            minor: Some("minor 2".to_string()),
            degree: "degree 2".to_string(),
            start_date: None,
            end_date: None,
        })
        .with_about("about".to_string())
        .with_avatar(id)
        .with_cover_photo(id)
        .with_city("city")
        .with_personalities("personality")
        .with_rating(4.0)
        .with_sex(Sex::Male)
        .with_experience(
            ExperienceBuilder::default()
                .with_title("title")
                .with_company("company")
                .with_description("description")
                .with_employment_type("employment_type")
                .with_location("location")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

fn create_demo_cv_input(author_id: ObjectId) -> CreateCVInput {
    CreateCVInputBuilder::default()
        .with_author_id(author_id)
        .with_title("title")
        .with_description("description")
        .with_tag("tag")
        .with_tag("tag2")
        .build()
        .unwrap()
}

fn create_test_comment(
    comment_id: ScalarObjectId,
    author_id: ScalarObjectId,
    content: String,
) -> Comment {
    Comment {
        id: comment_id,
        author: author_id,
        content,
        created: DateTime::now(),
        replies: vec![],
    }
}
