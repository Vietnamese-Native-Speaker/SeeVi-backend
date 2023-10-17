use crate::models::{sex::Sex, users::create_user_input::CreateUserInputBuilder};
use lazy_static::lazy_static;

use super::{educations::EDUCATIONS, experiences::EXPERIENCES};

fn generate_user_inputs() -> Vec<CreateUserInputBuilder> {
    [
        CreateUserInputBuilder::default()
            .with_username("user1")
            .with_password("password1")
            .with_primary_email("user1@gmail.com")
            .with_first_name("firstname1")
            .with_last_name("lastname1")
            .with_sex(Sex::Male)
            .with_education(EDUCATIONS[0].build().unwrap())
            .with_other_mail("othermail1@gmail.com")
            .with_other_mail("otherothermail1@gmail.com").to_owned(),
        CreateUserInputBuilder::default()
            .with_username("user2")
            .with_password("password2")
            .with_primary_email("user2@gmail.com")
            .with_first_name("firstname2")
            .with_last_name("lastname2")
            .with_education(EDUCATIONS[1].build().unwrap())
            .with_education(EDUCATIONS[2].build().unwrap())
            .with_experience(EXPERIENCES[0].build().unwrap())
            .with_sex(Sex::Male)
            .with_other_mail("othermail2@gmail.com")
            .with_other_mail("otherothermail2@gmail.com").to_owned(),
        CreateUserInputBuilder::default()
            .with_username("user3")
            .with_password("password3")
            .with_primary_email("user3@gmail.com")
            .with_first_name("firstname3")
            .with_experience(EXPERIENCES[1].build().unwrap())
            .with_education(EDUCATIONS[2].build().unwrap())
            .with_sex(Sex::Female)
            .with_last_name("lastname3")
            .with_other_mail("othermail3@gmail.com")
            .with_other_mail("otherothermail3@gmail.com"),
    ]
    .into()
}

lazy_static! {
    pub(super) static ref USER_INPUTS: Vec<CreateUserInputBuilder> = generate_user_inputs();
}
