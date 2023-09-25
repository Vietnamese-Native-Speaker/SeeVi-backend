mod error;
pub mod mongo;
#[cfg(test)]
mod tests;
mod traits;

pub use traits::cv::CVDataSource;
pub use traits::friends_list::FriendsListDataSource;
pub use traits::user::UserDataSource;
pub use error::Error as DataSourceError;
