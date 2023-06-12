use mongodb::bson::Uuid;

use crate::models::{
    education::Education,
    users::{create_user_input::CreateUserInputBuilder, update_user_input::UpdateUserInputBuilder},
};

use super::User;

#[test]
fn test_create_user_input_to_user() {
    let uuid = Uuid::new();
    let test_user_input = CreateUserInputBuilder::default()
        .with_username("username")
        .with_first_name("first_name")
        .with_last_name("last_name")
        .with_country("country")
        .with_skill("skill")
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_other_mail("other_mails2")
        .with_education(Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_education(Education {
            institution: "University of Example 2".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_about("about".to_string())
        .with_avatar(uuid)
        .with_cover_photo(uuid)
        .build()
        .unwrap();

    let _user = User {
        user_id: Uuid::new(),
        username: test_user_input.username,
        first_name: test_user_input.first_name,
        last_name: test_user_input.last_name,
        country: test_user_input.country,
        skills: test_user_input.skills,
        primary_email: test_user_input.primary_email,
        other_mails: test_user_input.other_mails,
        about: test_user_input.about,
        education: test_user_input.education,
        avatar: test_user_input.avatar,
        cover_photo: test_user_input.cover_photo,
        rating: test_user_input.rating,
        level: test_user_input.level,
        shared_cvs: Vec::default(),
        saved_cvs: Vec::default(),
        liked_cvs: Vec::default(),
        friends_list: Vec::default(),
        cv: Vec::default(),
    };
}
