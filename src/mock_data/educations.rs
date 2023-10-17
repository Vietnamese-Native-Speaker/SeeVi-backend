use lazy_static::lazy_static;
use mongodb::bson::DateTime;

use crate::models::education::EducationBuilder;

fn educations() -> Vec<EducationBuilder> {
    vec![
        EducationBuilder::default()
            .major("computer science".to_owned())
            .degree("bachelor".to_string())
            .school("hcmus".to_string())
            .start_date(Some(
                DateTime::parse_rfc3339_str("2010-01-01T00:00:00Z").unwrap(),
            ))
            .end_date(Some(
                DateTime::parse_rfc3339_str("2023-01-01T00:00:00Z").unwrap(),
            ))
            .to_owned(),
        EducationBuilder::default()
            .major("computer science".to_owned())
            .degree("bachelor".to_string())
            .school("hcmut".to_string())
            .start_date(Some(
                DateTime::parse_rfc3339_str("2017-01-01T00:00:00Z").unwrap(),
            ))
            .end_date(Some(
                DateTime::parse_rfc3339_str("2020-01-01T00:00:00Z").unwrap(),
            ))
            .to_owned(),
        EducationBuilder::default()
            .major("computer science".to_owned())
            .degree("bachelor".to_string())
            .school("hcmut".to_string())
            .start_date(Some(
                DateTime::parse_rfc3339_str("2018-01-01T00:00:00Z").unwrap(),
            ))
            .end_date(Some(
                DateTime::parse_rfc3339_str("2023-01-01T00:00:00Z").unwrap(),
            ))
            .to_owned(),
    ]
}

lazy_static! {
    pub(super) static ref EDUCATIONS: Vec<EducationBuilder> = educations();
}
