use crate::api::errors::ServiceError;

pub type ServiceResult<T> = Result<T, ServiceError>;
