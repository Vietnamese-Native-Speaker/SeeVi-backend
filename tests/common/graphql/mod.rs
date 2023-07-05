mod prepared;
use std::collections::HashMap;

pub fn make_graphql(query: &str, operation_name: &str, variables: serde_json::Value) -> String {
    let variables = serde_json::to_string(&variables).unwrap();
    TEMPLATE
        .replace("{{query}}", &query.replace("\n", " "))
        .replace("{{operation_name}}", operation_name)
        .replace("{{variables}}", &variables)
}

pub static TEMPLATE: &str = r#"{
"query": "{{query}}",
"operationName": "{{operation_name}}",
"variables": {{variables}}
}"#;

pub use prepared::*;
