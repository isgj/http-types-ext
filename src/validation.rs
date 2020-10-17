use async_trait::async_trait;
use http_types::{Request, Result};
use serde::de::DeserializeOwned;
use tide::{convert::json, Body, Request as TRequest, Response, StatusCode};
use validator::Validate;

#[async_trait]
pub trait ValidDeserialize {
    async fn valid_json<T: DeserializeOwned + Validate>(&mut self) -> Result<T>;
    async fn valid_form<T: DeserializeOwned + Validate>(&mut self) -> Result<T>;
}

#[async_trait]
impl ValidDeserialize for Request {
    async fn valid_json<T: DeserializeOwned + Validate>(&mut self) -> Result<T> {
        let data: T = self.body_json().await?;
        data.validate()?;
        Ok(data)
    }

    async fn valid_form<T: DeserializeOwned + Validate>(&mut self) -> Result<T> {
        let data: T = self.body_form().await?;
        data.validate()?;
        Ok(data)
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> ValidDeserialize for TRequest<State> {
    async fn valid_json<T: DeserializeOwned + Validate>(&mut self) -> Result<T> {
        let data: T = self.body_json().await?;
        data.validate()?;
        Ok(data)
    }

    async fn valid_form<T: DeserializeOwned + Validate>(&mut self) -> Result<T> {
        let data: T = self.body_form().await?;
        data.validate()?;
        Ok(data)
    }
}

/// Helper after middleware for tide.
/// It will map serde_json::Error --> 400 with a body {"error": "stringed error"}
/// and validator::ValidationError --> 422 and the body will be the serialized error
pub async fn map_validation_errors(mut res: Response) -> Result<Response> {
    if let Some(e) = res.downcast_error::<serde_json::Error>() {
        let err = e.to_string();
        res.set_status(StatusCode::BadRequest);
        res.set_body(json!({ "error": err }));
    }
    if let Some(e) = res.downcast_error::<validator::ValidationErrors>() {
        let body = Body::from_json(e)?;
        res.set_status(StatusCode::UnprocessableEntity);
        res.set_body(body);
    }

    Ok(res)
}
