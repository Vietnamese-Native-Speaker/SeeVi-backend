use async_graphql::async_trait::async_trait;
use futures_core::stream::BoxStream;

use crate::models::cv::CV;
use crate::models::cv_details::CVDetails;
use crate::services::cv_service::error::CVServiceError;

/// Primary abstraction for CV Details Data Source. Ones should implement this trait for
/// different type of database in order to provide that data source to services
#[async_trait]
pub trait CVDetailsDataSource {
    /// Return the CV with the provided filter.
    type Error: std::error::Error + Sync + Send + Into<CVServiceError>;
    async fn get_cvs_by_filter(
        &self,
        cv_details: CVDetails,
    ) -> Result<BoxStream<CV>, Self::Error>;
}
