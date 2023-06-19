use crate::models::users::{
    create_user_input::{CreateUserInput, CreateUserInputBuilder},
    update_user_input::{UpdateUserInput, UpdateUserInputBuilder},
};
use crate::models::education::Education;
use mongodb::bson::Uuid;
use serial_test::serial;
use crate::data_source::user_data_source::UserDataSource;
use crate::data_source::user_data_source_error::UserDataSourceError;
use super::mongo::MongoDB;

#[tokio::test]
#[serial]
async fn test_create_user_and_get_user_by_username(){
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let input = CreateUserInputBuilder::default()
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
    assert_eq!(check_input.other_mails, vec!["other_mails".to_string(), "other_mails2".to_string()]);
    assert_eq!(check_input.about, Some("about".to_string()));
    assert_eq!(check_input.avatar, Some(uuid));
    assert_eq!(check_input.cover_photo, Some(uuid));
    assert_eq!(check_input.friends_list, vec![]);
    assert_eq!(check_input.education.len(), 2);
    assert_eq!(check_input.education[0].institution, "University of Example 1".to_string());
    assert_eq!(check_input.education[0].course, Some("Computer Science".to_string()));
    assert_eq!(check_input.education[0].degree, Some("Bachelor's Degree".to_string()));
    assert_eq!(check_input.education[1].institution, "University of Example 2".to_string());
    assert_eq!(check_input.education[1].course, Some("Computer Science".to_string()));
    assert_eq!(check_input.education[1].degree, Some("Bachelor's Degree".to_string()));
    assert_eq!(check_input.rating, None);
    assert_eq!(check_input.level, None);
    assert_eq!(check_input.shared_cvs, vec![]);
    assert_eq!(check_input.saved_cvs, vec![]);
    assert_eq!(check_input.liked_cvs, vec![]);
    let check_input2 = mongodb.create_user(input_clone).await;
    assert_eq!(check_input2, Err(UserDataSourceError::UsernameTaken(username_clone)));

}

#[tokio::test]
#[serial]
async fn test_get_user_by_id(){
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let input = CreateUserInputBuilder::default()
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
    mongodb.create_user(input).await.unwrap();
    let uuid2 = mongodb.get_user_by_username("username").await.unwrap().user_id;
    let check_input = mongodb.get_user_by_id(uuid2).await.unwrap();
    assert_eq!(check_input.username, "username".to_string());
    assert_eq!(check_input.first_name, "first_name".to_string());
    assert_eq!(check_input.last_name, "last_name".to_string());
    assert_eq!(check_input.country, Some("country".to_string()));
    assert_eq!(check_input.skills, vec!["skill".to_string()]);
    assert_eq!(check_input.cv, vec![]);
    assert_eq!(check_input.primary_email, "primary_email".to_string());
    assert_eq!(check_input.other_mails, vec!["other_mails".to_string(), "other_mails2".to_string()]);
    assert_eq!(check_input.about, Some("about".to_string()));
    assert_eq!(check_input.avatar, Some(uuid));
    assert_eq!(check_input.cover_photo, Some(uuid));
    assert_eq!(check_input.friends_list, vec![]);
    assert_eq!(check_input.education.len(), 2);
    assert_eq!(check_input.education[0].institution, "University of Example 1".to_string());
    assert_eq!(check_input.education[0].course, Some("Computer Science".to_string()));
    assert_eq!(check_input.education[0].degree, Some("Bachelor's Degree".to_string()));
    assert_eq!(check_input.education[1].institution, "University of Example 2".to_string());
    assert_eq!(check_input.education[1].course, Some("Computer Science".to_string()));
    assert_eq!(check_input.education[1].degree, Some("Bachelor's Degree".to_string()));
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
async fn test_delete_user(){
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let input = CreateUserInputBuilder::default()
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
    mongodb.create_user(input).await.unwrap();
    let uuid2 = mongodb.get_user_by_username("username").await.unwrap().user_id;
    let uuid3 = Uuid::new();
    let error = mongodb.delete_user(uuid3).await;
    assert_eq!(error, Err(UserDataSourceError::UuidNotFound(uuid3)));
    mongodb.delete_user(uuid2).await.unwrap();
    let error2 = mongodb.get_user_by_id(uuid2).await;
    assert_eq!(error2, Err(UserDataSourceError::UuidNotFound(uuid2)))
}

#[tokio::test]
#[serial]
async fn test_update_user_info(){
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let input = CreateUserInputBuilder::default()
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
    let check_input = mongodb.create_user(input).await.unwrap();
    let updateinput = UpdateUserInputBuilder::default()
        .with_user_id(check_input.user_id)
        .with_username("username2".to_string())
        .with_first_name("first_name2".to_string())
        .with_last_name("last_name2".to_string())
        .with_country("country2".to_string())
        .with_education(vec![Education{
            institution: "University of Example 3".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
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
    assert_eq!(check_input2.skills, vec!["skill".to_string(), "skill2".to_string()]);
    assert_eq!(check_input2.cv, vec![]);
    assert_eq!(check_input2.primary_email, "primary_email2".to_string());
    assert_eq!(check_input2.other_mails, vec!["other_mails".to_string(), "other_mails2".to_string()]);
    assert_eq!(check_input2.about, Some("about2".to_string()));
    assert_eq!(check_input2.avatar, Some(uuid));
    assert_eq!(check_input2.cover_photo, Some(uuid));
    assert_eq!(check_input2.friends_list, vec![]);
    assert_eq!(check_input2.education.len(), 1);
    assert_eq!(check_input2.education[0].institution, "University of Example 3".to_string());
    assert_eq!(check_input2.education[0].course, Some("Computer Science".to_string()));
    assert_eq!(check_input2.education[0].degree, Some("Bachelor's Degree".to_string()));
    assert_eq!(check_input2.rating, None);
    assert_eq!(check_input2.level, None);
    assert_eq!(check_input2.shared_cvs, vec![]);
    assert_eq!(check_input2.saved_cvs, vec![]);
    assert_eq!(check_input2.liked_cvs, vec![]);
}

