use crate::models::experience::ExperienceBuilder;
use lazy_static::lazy_static;

fn experiences() -> Vec<ExperienceBuilder> {
    vec![
        ExperienceBuilder::default()
            .with_title("Software Engineer".to_string())
            .with_company("Company 1".to_string())
            .with_location("HCM City".to_string())
            .with_description("Some short description".to_string())
            .with_employment_type("Full-time".to_string())
            .to_owned(),
        ExperienceBuilder::default()
            .with_title("Architect Designer".to_string())
            .with_company("Company 2".to_string())
            .with_location("Hanoi".to_string())
            .with_employment_type("Full-time".to_string())
            .with_description("Some short description about experience 2".to_string())
            .to_owned(),
        ExperienceBuilder::default()
            .with_title("Architect Designer".to_string())
            .with_employment_type("Part-time".to_string())
            .with_company("Company 3".to_string())
            .with_location("Danang City".to_string())
            .with_description("Some short description about experience 3".to_string())
            .to_owned(),
    ]
}

lazy_static! {
    pub(super) static ref EXPERIENCES: Vec<ExperienceBuilder> = experiences();
}
