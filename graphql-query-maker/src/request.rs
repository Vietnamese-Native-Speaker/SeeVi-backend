use async_graphql_parser::types::OperationType;
use regex::Regex;

use crate::{
    make_function_body,
    variable::{make_function_variables, Variable},
};

pub struct Operation {
    pub name: String,
    pub operation_type: OperationType,
    pub variable_definitions: Vec<Variable>,
}

pub struct Request {
    pub query: String,
    pub operation: Operation,
}

pub(crate) fn make_function_declaration(operation: Operation) -> String {
    let function_name = make_function_name(&operation.name, operation.operation_type);
    let function_variables = make_function_variables(&operation.variable_definitions);
    format!("pub fn {}({}) -> String", function_name, function_variables)
}

fn camel_to_snake(string: &str) -> String {
    let re = Regex::new("(.)([A-Z][a-z]+)").unwrap();
    let re2 = Regex::new("([a-z0-9])([A-Z])").unwrap();
    let new = re.replace_all(string, |caps: &regex::Captures| {
        format!("{}_{}", &caps[1], &caps[2])
    });
    return re2
        .replace_all(&new, |caps: &regex::Captures| {
            format!("{}_{}", &caps[1], &caps[2])
        })
        .to_lowercase()
        .to_lowercase()
        .to_string();
}

fn make_function_name(operation_name: &str, operation_type: OperationType) -> String {
    let operation_name = camel_to_snake(operation_name);
    match operation_type {
        OperationType::Query => format!("query_{}", operation_name),
        OperationType::Mutation => format!("mutation_{}", operation_name),
        OperationType::Subscription => format!("subscription_{}", operation_name),
    }
}

fn parse_query(graphql_str: &str) -> Result<Request, String> {
    let document = async_graphql_parser::parse_query(graphql_str).unwrap();
    let defs: Vec<Result<_, String>> = document
        .operations
        .iter()
        .map(|op| {
            let name = op.0.ok_or("Operation name is not found".to_string())?;
            Ok((
                name,
                op.1.node.ty,
                op.1.clone().map(|node| node.variable_definitions.clone()),
            ))
        })
        .collect::<Vec<_>>();
    if defs.len() > 1 {
        return Err("Multiple operations are not supported".to_string());
    }
    let operation = defs
        .get(0)
        .ok_or("Operation is not found".to_string())?
        .to_owned()?;
    let name = operation.0;
    let variables = operation
        .2
        .node
        .clone()
        .into_iter()
        .map(|var| {
            let name = var.node.name.node.to_string();
            let nullable = var.node.var_type.node.nullable;
            Variable { name, nullable }
        })
        .collect::<Vec<_>>();
    let name = name.to_string();
    let operation = Operation {
        name,
        operation_type: operation.1,
        variable_definitions: variables,
    };
    Ok(Request {
        query: graphql_str.to_string(),
        operation,
    })
}

pub(crate) fn make_function_string(graphql_str: &str) -> Result<String, String> {
    let request = parse_query(graphql_str)?;
    let body = make_function_body(&request.operation, request.query.as_str());
    let operation = request.operation;
    let function = make_function_declaration(operation);
    Ok(format!("{} {{{}}}", function, body))
}
