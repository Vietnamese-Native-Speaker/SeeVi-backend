#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub nullable: bool,
}

pub(crate) fn make_function_variables(variables: &[Variable]) -> String {
    if variables.is_empty() {
        return String::new();
    }
    let first = if variables[0].nullable {
        format!(
            "{}: Option<serde_json::Value>",
            variables[0].name
        )
    } else {
        format!("{}: serde_json::Value", variables[0].name)
    };
    variables.into_iter().skip(1).fold(first, |acc, var| {
        if var.nullable {
            format!(
                "{}, {}: Option<serde_json::Value>",
                acc, var.name
            )
        } else {
            format!("{}, {}: Into<serde_json::Value>", acc, var.name)
        }
    })
}
