use crate::models::cv::create_cv_input::CreateCVInputBuilder;
use lazy_static::lazy_static;

fn generate_cv_inputs() -> Vec<CreateCVInputBuilder> {
    vec![
        CreateCVInputBuilder::default()
            .with_title("My CV 1")
            .with_tag("dummy_tag1")
            .with_tag("dummy_tag2")
            .with_tag("dummy_tag3"),
        CreateCVInputBuilder::default()
            .with_title("My CV 2")
            .with_tag("dummy_tag1")
            .with_tag("dummy_tag2")
            .with_tag("dummy_tag3"),
        CreateCVInputBuilder::default()
            .with_title("My CV 3")
            .with_tag("dummy_tag1")
            .with_tag("dummy_tag10")
            .with_tag("dummy_tag7"),
    ]
}

lazy_static! {
    pub(super) static ref CV_INPUTS: Vec<CreateCVInputBuilder> = generate_cv_inputs();
}
