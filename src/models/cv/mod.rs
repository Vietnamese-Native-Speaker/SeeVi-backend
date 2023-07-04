pub mod create_cv_input;
pub mod cv;
pub mod update_cv_input;
pub use update_cv_input::UpdateCVInput;
pub use create_cv_input::CreateCVInput;
pub use cv::CV;
#[cfg(test)]
mod tests;
