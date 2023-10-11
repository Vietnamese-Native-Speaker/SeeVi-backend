use crate::data_source::user_data_source::UserDataSource;
use crate::data_source::user_data_source_error::UserDataSourceError;
use crate::models::education::Education;
use crate::{
    data_source::mongo::MongoDB,
    models::users::{
        create_user_input::{CreateUserInput, CreateUserInputBuilder},
        update_user_input::UpdateUserInputBuilder,
    },
};
use mongodb::bson::Uuid;
use serial_test::serial;

fn create_demo_user_input(test_uuid: Uuid) -> CreateUserInput {
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
            end_date: None
        })
        .with_education(Education {
            school: "school 2".to_string(),
            major: "major 2".to_string(),
            minor: Some("minor 2".to_string()),
            degree: "degree 2".to_string(),
            start_date: None,
            end_date: None
        })
        .with_about("about".to_string())
        .with_avatar(test_uuid)
        .with_cover_photo(test_uuid)
        .build()
        .unwrap()
}

#[tokio::test]
#[serial]
async fn test_create_user_and_get_user_by_username() {
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let input = create_demo_user_input(uuid);
    let username_clone = input.username.clone();
    let input_clone = input.clone();
    let check_input = mongodb.create_user(input).await.unwrap();
    assert_eq!(check_input.username, "username".to_string());
    assert_eq!(check_input.first_name, "first_name".to_string());
    assert_eq!(check_input.last_name, "last_name".to_string());
    assert_eq!(check_input.country, Some("country".to_string()));
    assert_eq!(check_input.skills, vec!["skill".to_string()]);
    assert_eq!(check_input.cv, vec![]);
    assert_eq!(check_input.primary_email, "primary_email".to_string());
    assert_eq!(
        check_input.other_mails,
        vec!["other_mails".to_string(), "other_mails2".to_string()]
    );
    assert_eq!(check_input.about, Some("about".to_string()));
    assert_eq!(check_input.avatar, Some(uuid));
    assert_eq!(check_input.cover_photo, Some(uuid));
    assert_eq!(check_input.friends_list, vec![]);
    assert_eq!(check_input.education, vec![
        Education {
            school: "school 1".to_string(),
            major: "major 1".to_string(),
            minor: Some("minor 1".to_string()),
            degree: "degree 1".to_string(),
            start_date: None,
            end_date: None
        },
        Education {
            school: "school 2".to_string(),
            major: "major 2".to_string(),
            minor: Some("minor 2".to_string()),
            degree: "degree 2".to_string(),
            start_date: None,
            end_date: None
        }
    ]);
    assert_eq!(check_input.rating, None);
    assert_eq!(check_input.level, None);
    assert_eq!(check_input.shared_cvs, vec![]);
    assert_eq!(check_input.saved_cvs, vec![]);
    assert_eq!(check_input.liked_cvs, vec![]);
    let check_input2 = mongodb.create_user(input_clone).await;
    assert_eq!(
        check_input2,
        Err(UserDataSourceError::UsernameTaken(username_clone))
    );
}

#[tokio::test]
#[serial]
async fn test_get_user_by_id() {
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let input = create_demo_user_input(uuid);
    mongodb.create_user(input).await.unwrap();
    let uuid2 = mongodb
        .get_user_by_username("username")
        .await
        .unwrap()
        .user_id;
    let check_input = mongodb.get_user_by_id(uuid2).await.unwrap();
    assert_eq!(check_input.username, "username".to_string());
    assert_eq!(check_input.first_name, "first_name".to_string());
    assert_eq!(check_input.last_name, "last_name".to_string());
    assert_eq!(check_input.country, Some("country".to_string()));
    assert_eq!(check_input.skills, vec!["skill".to_string()]);
    assert_eq!(check_input.cv, vec![]);
    assert_eq!(check_input.primary_email, "primary_email".to_string());
    assert_eq!(
        check_input.other_mails,
        vec!["other_mails".to_string(), "other_mails2".to_string()]
    );
    assert_eq!(check_input.about, Some("about".to_string()));
    assert_eq!(check_input.avatar, Some(uuid));
    assert_eq!(check_input.cover_photo, Some(uuid));
    assert_eq!(check_input.friends_list, vec![]);
    assert_eq!(check_input.education, vec![
        Education {
            school: "school 1".to_string(),
            major: "major 1".to_string(),
            minor: Some("minor 1".to_string()),
            degree: "degree 1".to_string(),
            start_date: None,
            end_date: None
        },
        Education {
            school: "school 2".to_string(),
            major: "major 2".to_string(),
            minor: Some("minor 2".to_string()),
            degree: "degree 2".to_string(),
            start_date: None,
            end_date: None
        }
    ]);
    assert_eq!(check_input.rating, None);
    assert_eq!(check_input.level, None);
    assert_eq!(check_input.shared_cvs, vec![]);
    assert_eq!(check_input.saved_cvs, vec![]);
    assert_eq!(check_input.liked_cvs, vec![]);
    let uuid3 = Uuid::new();
    let check_input2 = mongodb.get_user_by_id(uuid3).await;
    assert_eq!(check_input2, Err(UserDataSourceError::UuidNotFound(uuid3)));
}

#[tokio::test]
#[serial]
async fn test_delete_user() {
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let input = create_demo_user_input(uuid);
    mongodb.create_user(input).await.unwrap();
    let uuid2 = mongodb
        .get_user_by_username("username")
        .await
        .unwrap()
        .user_id;
    let uuid3 = Uuid::new();
    let error = mongodb.delete_user(uuid3).await;
    assert_eq!(error, Err(UserDataSourceError::UuidNotFound(uuid3)));
    mongodb.delete_user(uuid2).await.unwrap();
    let error2 = mongodb.get_user_by_id(uuid2).await;
    assert_eq!(error2, Err(UserDataSourceError::UuidNotFound(uuid2)))
}

#[tokio::test]
#[serial]
async fn test_update_user_info() {
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let input = create_demo_user_input(uuid);
    let check_input = mongodb.create_user(input).await.unwrap();
    let updateinput = UpdateUserInputBuilder::default()
        .with_user_id(check_input.user_id)
        .with_username("username2".to_string())
        .with_first_name("first_name2".to_string())
        .with_last_name("last_name2".to_string())
        .with_country("country2".to_string())
        .with_education(vec![Education {
            school: "school 3".to_string(),
            major: "major 3".to_string(),
            minor: Some("minor 3".to_string()),
            degree: "degree 3".to_string(),
            start_date: None,
            end_date: None
        }])
        .with_skills(vec!["skill".to_string(), "skill2".to_string()])
        .with_primary_email("primary_email2".to_string())
        .with_about("about2".to_string())
        .build()
        .unwrap();
    let check_input2 = mongodb.update_user_info(updateinput).await.unwrap();
    assert_eq!(check_input2.username, "username2".to_string());
    assert_eq!(check_input2.first_name, "first_name2".to_string());
    assert_eq!(check_input2.last_name, "last_name2".to_string());
    assert_eq!(check_input2.country, Some("country2".to_string()));
    assert_eq!(
        check_input2.skills,
        vec!["skill".to_string(), "skill2".to_string()]
    );
    assert_eq!(check_input2.cv, vec![]);
    assert_eq!(check_input2.primary_email, "primary_email2".to_string());
    assert_eq!(
        check_input2.other_mails,
        vec!["other_mails".to_string(), "other_mails2".to_string()]
    );
    assert_eq!(check_input2.about, Some("about2".to_string()));
    assert_eq!(check_input2.avatar, Some(uuid));
    assert_eq!(check_input2.cover_photo, Some(uuid));
    assert_eq!(check_input2.friends_list, vec![]);
    assert_eq!(check_input2.education, vec![Education {
        school: "school 3".to_string(),
        major: "major 3".to_string(),
        minor: Some("minor 3".to_string()),
        degree: "degree 3".to_string(),
        start_date: None,
        end_date: None
    }]);
    assert_eq!(check_input2.rating, None);
    assert_eq!(check_input2.level, None);
    assert_eq!(check_input2.shared_cvs, vec![]);
    assert_eq!(check_input2.saved_cvs, vec![]);
    assert_eq!(check_input2.liked_cvs, vec![]);
}
