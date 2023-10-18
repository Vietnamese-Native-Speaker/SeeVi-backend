use crate::models::{
    comment::{create_comment_input::CreateCommentInputBuilder, CreateCommentInput},
    cv::create_cv_input::CreateCVInputBuilder,
};

fn generate_comment_inputs() -> Vec<CreateCommentInputBuilder> {
    vec![
        CreateCommentInputBuilder::default()
            .content("Comment 1".to_string())
            .to_owned(),
        CreateCommentInputBuilder::default()
            .content("Comment 2".to_string())
            .to_owned(),
        CreateCommentInputBuilder::default()
            .content("Comment 3".to_string())
            .to_owned(),
        CreateCommentInputBuilder::default()
            .content("Comment 4".to_string())
            .to_owned(),
        CreateCommentInputBuilder::default()
            .content(r#"This is a very long comment. "#.to_string())
            .to_owned(),
        CreateCommentInputBuilder::default()
            .content("Comment 6".to_string())
            .to_owned(),
    ]
}

fn generate_reply_inputs() -> Vec<CreateCommentInputBuilder> {
    vec![
        CreateCommentInputBuilder::default()
            .content("Reply 1".to_string())
            .to_owned(),
        CreateCommentInputBuilder::default()
            .content(r#"This is a very long reply."#.to_string())
            .to_owned(),
        CreateCommentInputBuilder::default()
            .content("Reply 3".to_string())
            .to_owned(),
    ]
}

lazy_static::lazy_static! {
    pub(super) static ref COMMENT_INPUTS: Vec<CreateCommentInputBuilder> = generate_comment_inputs();
    pub(super) static ref REPLY_INPUTS: Vec<CreateCommentInputBuilder> = generate_reply_inputs();
}
