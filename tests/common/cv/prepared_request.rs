use seevi_backend::{models::cv::CreateCVInput, object_id::ScalarObjectId};

use crate::common::graphql::make_graphql;

pub static CREATE_CV: &str = r#"
mutation createCV($cv: CreateCVInput!) {
    createCV(cv: $cv) {
        id,
        title,
        author_id,
        comments,
    }
}"#;

pub static GET_CV: &str = r#"
query getCV($id: ScalarObjectId!) {
    getCVById(id: $id) {
        id,
        title,
        author_id,
        comments,
    }
}"#;

pub static ADD_COMMENT: &str = r#"
mutation addComment($cv_id: ScalarObjectId, $comment: CreateCommentInput!) {
    addComment($cv_id, comment: $comment) {
        id,
        cv_id,
        author_id,
        content,
    }
}"#;

pub static REPLY_COMMENT: &str = r#"
mutation replyComment($comment_id: ScalarObjectId!, $comment: CreateCommentInput!) {
    replyComment(comment_id: $comment_id, comment: $comment) {
        content,
    }
}"#;

pub static GET_COMMENT: &str = r#"
query getComment($id: ScalarObjectId!) {
    getCommentById(id: $id) {
        id,
        cv_id,
        author_id,
        content,
        replies,
    }
}"#;

pub static CV_COMMENTS: &str = r#"
query getCommentsOfCv($id: ObjectId!, $after: String, $before: String, $first: Int, $last: Int) {
    getCommentsOfCv(cvId: $id, after: $after, before: $before, first: $first, last: $last) {
        edges {
            node {
                id,
                content,
            }
        }
        pageInfo {
            hasNextPage,
            hasPreviousPage,
            startCursor,
            endCursor
        }
    }
}"#;

pub static COMMENT_REPLIES: &str = r#"
query getCommentsOfCv($id: ObjectId!, $after: String, $before: String, $first: Int, $last: Int) {
    getCommentsOfCv(commentId: $id, after: $after, before: $before, first: $first, last: $last) {
        edges {
            node {
                id,
                content,
            }
        }
        pageInfo {
            hasNextPage,
            hasPreviousPage,
            startCursor,
            endCursor
        }
    }
}"#;

pub fn graphql_comment_replies(
    id: ScalarObjectId,
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
) -> String {
    make_graphql(
        COMMENT_REPLIES,
        "getRepliesOfComment",
        serde_json::json!(
            {
                "id": id,
                "after": after,
                "before": before,
                "first": first,
                "last": last,
            }
        ),
    )
}

pub fn graphql_cv_comments(
    id: ScalarObjectId,
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
) -> String {
    make_graphql(
        CV_COMMENTS,
        "getCommentsOfCv",
        serde_json::json!(
            {
                "id": id,
                "after": after,
                "before": before,
                "first": first,
                "last": last,
            }
        ),
    )
}

pub fn graphql_get_cv(id: ScalarObjectId) -> String {
    make_graphql(
        GET_CV,
        "getCVById",
        serde_json::json!(
            {
                "id": id,
            }
        ),
    )
}

pub fn graphql_create_cv(input: CreateCVInput) -> String {
    make_graphql(
        CREATE_CV,
        "createCV",
        serde_json::json!(
            {
                "cv": input,
            }
        ),
    )
}

pub fn graphql_add_comment_to_cv(
    token: &str,
    cv_id: ScalarObjectId,
    author_id: ScalarObjectId,
    content: &str,
) -> String {
    make_graphql(
        ADD_COMMENT,
        "addComment",
        serde_json::json!(
            {
                "comment": {
                    "author": cv_id,
                    "content": content,
                },
                "cv_id": cv_id,
            }
        ),
    )
}

pub fn graphql_reply_comment(token: &str, comment_id: ScalarObjectId, content: &str) -> String {
    make_graphql(
        REPLY_COMMENT,
        "replyComment",
        serde_json::json!(
            {
                "comment_id": comment_id,
                "comment": {
                    "content": content,
                },
            }
        ),
    )
}
