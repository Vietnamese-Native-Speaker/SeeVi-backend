extern crate proc_macro;
use async_graphql_parser::types::OperationType;
use proc_macro::TokenStream;
mod request;
mod variable;

use request::{make_function_string, Operation};

static TEMPLATE: &str = r#""\"query\": \"{{query}}\", \"operationName\": \"{{operation_name}}\", \"variables\": {{variables}}""#;

fn make_graphql_template(query: &str, operation_name: &str) -> String {
    TEMPLATE
        .replace("{{query}}", &query.replace("\n", " "))
        .replace("{{operation_name}}", operation_name)
}

fn make_function_body(operation: &Operation, query: &str) -> String {
    let variables = operation
        .variable_definitions
        .iter()
        .map(|var| {
            if var.nullable {
                format!("\"{}\": {}.map(|v| v.into())", var.name, var.name)
            } else {
                format!("\"{}\": {}.into()", var.name, var.name)
            }
        })
        .collect::<Vec<_>>();
    let variables = variables.join(", ");
    let variables_decl = format!("let variables = serde_json::json!({{{}}})", variables);
    let graphql = make_graphql_template(query, &operation.name);
    format!(
        r#"{};
let graphql = {}.replace("{{{{variables}}}}", &serde_json::to_string(&variables).unwrap());
graphql.to_string()"#,
        variables_decl, graphql
    )
}

#[proc_macro]
/// Make a function from a graphql query.
/// Function's name will be `query_{operation_name}` or `mutation_{operation_name}`.
/// The function will take variables as arguments, therefore shorthand syntax of GraphQL is not supported.
/// # Example
/// ```rust
/// #[macro_use] extern crate graphql_query_maker;
/// make_graphql!(
/// mutation userRegister($user: CreateUserInput!) {
///     userRegister(newUser: $user) {
///         id,
///         username
///     }
/// }
/// );
///
/// make_graphql!(
/// query friendslist($after: string, $before: string, $first: int, $last: int) {
///     userdetail {
///         friends(first: $first, last: $last, after: $after, before: $before) {
///             edges {
///                 node {
///                     id,
///                     username
///                 }
///                 cursor
///             }
///             pageinfo {
///                 hasnextpage,
///                 haspreviouspage,
///                 startcursor,
///                 endcursor
///             }
///         }
///     }
/// }
/// );
///
/// fn main() {
///    mutation_user_register(serde_json::json!({
///        "user": {
///            "username": "test",
///            "password": "test"
///        }
///    }));
///    query_friendslist(
///        Option::<()>::None,
///        Option::<()>::None,
///        Option::<()>::None,
///        Some(10),
///    );
/// }
/// ```
pub fn make_graphql(token_stream: TokenStream) -> TokenStream {
    let graphql_str = token_stream.to_string();
    let rs = make_function_string(&graphql_str);
    match rs {
        Ok(rs) => rs.parse().unwrap(),
        Err(e) => panic!("{}", e),
    }
}

#[test]
fn test_make_graphql() {
    let graphql = r#"
query loginSomething($info: LoginInfo!, $name: String!) {
    login(loginInfo: $info) {
        accessToken,
        refreshToken,
    }
}
"#;
    let function = make_function_string(graphql);
    println!("{}", function.unwrap());
    panic!()
}
