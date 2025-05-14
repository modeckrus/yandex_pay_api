mod orders;
mod orders_cancel;
mod orders_capture;
mod orders_id;
mod orders_refund;
mod orders_submit;
mod orders_subscriptions;
mod serde_help;
use std::sync::Arc;

use builder_pattern::Builder;
use bytes::Bytes;
pub use orders::*;
pub use orders_cancel::*;
pub use orders_capture::*;
pub use orders_id::*;
pub use orders_refund::*;
pub use orders_submit::*;
pub use orders_subscriptions::*;

pub trait HttpClient: Clone {
    fn send<T: serde::de::DeserializeOwned>(
        &self,
        request: YandexPayApiRequest,
    ) -> impl Future<Output = R<T>>;
}

pub(crate) type R<T = (), E = YandexPayApiError> = std::result::Result<T, E>;
pub(crate) type Time = chrono::DateTime<chrono::Utc>;

#[derive(Debug, thiserror::Error)]
#[error("Yandex Pay API error: {0}")]
pub enum YandexPayApiError {
    #[error("Yandex Pay reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Yandex Pay serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Yandex Pay API error: {0}")]
    Api(YandexPayApiResponseError),
    #[error("Yandex Pay error: {0}")]
    Other(#[from] anyhow::Error),
}

pub(crate) type S = Arc<str>;
#[derive(Debug, Clone)]
pub struct YandexPayApi<C: HttpClient> {
    pub client: C,
    pub base_url: S,
    pub api_key: S,
}

impl<C: HttpClient> YandexPayApi<C> {
    pub fn new(base_url: S, api_key: S, client: C) -> Self {
        YandexPayApi {
            client,
            base_url,
            api_key,
        }
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }
}

#[derive(Debug, Clone, Builder)]
pub struct YandexPayApiRequest {
    #[into]
    //serialized body
    pub body: Bytes,
    #[into]
    //url
    pub url: S,
    #[into]
    //Authorization Token
    pub api_key: S,
    #[into]
    #[default(default_request_id())]
    //Request Id
    pub request_id: S,
    #[default(9999)]
    //In milliseconds
    pub request_timeout: u32,
    #[default(0)]
    //Current attempt number
    pub request_attempt: u32,
}

fn default_request_id() -> S {
    uuid::Uuid::now_v7().to_string().into()
}

impl HttpClient for reqwest::Client {
    fn send<T: serde::de::DeserializeOwned>(
        &self,
        request: YandexPayApiRequest,
    ) -> impl Future<Output = R<T>> {
        let client = self.clone();

        async move {
            let body = request.body.clone();
            let response = client
                .post(&*request.url)
                .header("Authorization", format!("Api-Key {}", request.api_key))
                .header("X-Request-Id", &*request.request_id)
                .header("X-Request-Timeout", request.request_timeout.to_string())
                .header("X-Request-Attempt", request.request_attempt.to_string())
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await?;

            if response.status().is_success() {
                let result = response.text().await?;
                let result = serde_json::from_str::<YandexPayApiResponse<T>>(&result)?;
                Ok(result.data)
            } else {
                let error_message = response.text().await?;
                tracing::error!("{}", error_message);
                let error = serde_json::from_str::<YandexPayApiResponseError>(&error_message)?;
                Err(YandexPayApiError::Api(error))
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct YandexPayApiResponse<T> {
    pub data: T,
    pub code: Option<u32>,
    pub status: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct YandexPayApiResponseError {
    pub code: Option<u32>,
    pub status: Option<String>,
    #[serde(default = "Default::default")]
    pub message: S,
}

impl std::fmt::Display for YandexPayApiResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Yandex Pay API error: StatusCode: {:?}, Status: {:?}, Message: {}",
            self.code, self.status, self.message
        )
    }
}
