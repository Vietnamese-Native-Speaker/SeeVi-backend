pub mod query;
pub mod mutation;

pub type GqlResult<T> = Result<T, async_graphql::Error>;
