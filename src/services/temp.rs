use mongodb::Database;

pub type GqlResult<T> = Result<T, async_graphql::Error>;

pub async fn temp_function(db: Database) -> GqlResult<String> {
    Ok("Temp function!".to_string())
}