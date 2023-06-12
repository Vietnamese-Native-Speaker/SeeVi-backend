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

    let user = User {
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
        rating: panic!(),
        level: panic!(),
        shared_cvs: Vec::default(),
        saved_cvs: Vec::default(),
        liked_cvs: Vec::default(),
        friends_list: Vec::default(),
        cv: Vec::default(),
    };
}

#[test]
fn test_create_user_input_builder() {
    use mongodb::bson::Uuid;

    let uuid = Uuid::new();
    let uuid2 = Uuid::new();
    let test_user_input = CreateUserInputBuilder::default()
        .with_username("username")
        .with_first_name("first_name")
        .with_last_name("last_name")
        .with_country("country")
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

    assert_eq!(test_user_input.username, "username".to_string());
    assert_eq!(test_user_input.first_name, "first_name".to_string());
    assert_eq!(test_user_input.last_name, "last_name".to_string());
    assert_eq!(test_user_input.country, Some("country".to_string()));
    assert_eq!(test_user_input.primary_email, "primary_email".to_string());
    assert_eq!(
        test_user_input.other_mails,
        vec!["other_mails".to_string(), "other_mails2".to_string()]
    );
    assert_eq!(test_user_input.about, Some("about".to_string()));
    assert_eq!(test_user_input.avatar, Some(uuid));
    assert_eq!(test_user_input.cover_photo, Some(uuid));
    assert_eq!(
        test_user_input.education,
        vec![
            Education {
                institution: "University of Example 1".to_string(),
                course: Some("Computer Science".to_string()),
                degree: Some("Bachelor's Degree".to_string()),
            },
            Education {
                institution: "University of Example 2".to_string(),
                course: Some("Computer Science".to_string()),
                degree: Some("Bachelor's Degree".to_string()),
            }
        ]
    );
}

#[test]
fn test_update_user_input_builder() {
    let test_update_user = UpdateUserInputBuilder::default()
        .with_username("username".to_string())
        .with_first_name("first_name".to_string())
        .with_last_name("last_name".to_string())
        .with_country("country".to_string())
        .with_skills("skills".to_string())
        .with_primary_email("primary_email".to_string())
        .with_about("about".to_string())
        .with_education(vec![Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        }])
        .build()
        .unwrap();

    assert_eq!(test_update_user.username, Some("username".to_string()));
    assert_eq!(test_update_user.first_name, Some("first_name".to_string()));
    assert_eq!(test_update_user.last_name, Some("last_name".to_string()));
    assert_eq!(test_update_user.country, Some("country".to_string()));
    assert_eq!(test_update_user.skills, Some("skills".to_string()));
    assert_eq!(
        test_update_user.primary_email,
        Some("primary_email".to_string())
    );
    assert_eq!(test_update_user.about, Some("about".to_string()));
    assert_eq!(
        test_update_user.education,
        Some(vec![Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        }])
    );
}
