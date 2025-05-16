mod orders;
mod orders_cancel;
mod orders_capture;
mod orders_id;
mod orders_refund;
mod orders_submit;
mod orders_subscriptions;
mod orders_subscriptions_id;
mod orders_subscriptions_recur;
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
pub use orders_subscriptions_id::*;
pub use orders_subscriptions_recur::*;

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
}

pub(crate) type S = Arc<str>;
#[cfg(not(feature = "reqwest"))]
#[derive(Debug, Clone)]
pub struct YandexPayApi<C: HttpClient> {
    pub client: C,
    pub base_url: S,
    pub api_key: S,
}

#[cfg(feature = "reqwest")]
#[derive(Debug, Clone)]
pub struct YandexPayApi<C: HttpClient = reqwest::Client> {
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

/// Yandex Pay API
impl<C: HttpClient> YandexPayApi<C> {
    /// Запрос на создание ссылки на оплату заказа.
    ///
    /// Запрос используется для создания и получения ссылки на оплату заказа.
    pub async fn create_order(&self, request: CreateOrderRequest) -> R<CreateOrderResponse> {
        let url = format!("{}/api/merchant/v1/orders", self.base_url);
        let bytes = serde_json::to_vec(&request)?;
        let r = YandexPayApiRequest::new()
            .url(url)
            .method(Method::Post)
            .body(Some(bytes.into()))
            .api_key(self.api_key.clone())
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }
    /// Запрос на получение деталей заказа.
    ///
    /// Запрос возвращает детали заказа и список транзакций по возврату.
    pub async fn get_order(&self, order_id: impl Into<String>) -> R<OrderResponseData> {
        let url = format!(
            "{}/api/merchant/v1/orders/{}",
            self.base_url,
            order_id.into()
        );
        let r = YandexPayApiRequest::new()
            .url(url)
            .api_key(self.api_key.clone())
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }
    /// Запрос на отмену платежа.
    ///
    /// Доступно только для платежей в статусе AUTHORIZED. В случае успеха статус платежа изменится на VOIDED.
    pub async fn cancel_order(
        &self,
        order_id: impl Into<String>,
        request: CancelOrderRequest,
    ) -> R<OperationResponseData> {
        let url = format!(
            "{}/api/merchant/v1/orders/{}/cancel",
            self.base_url,
            order_id.into()
        );
        let bytes = serde_json::to_vec(&request)?;
        let r = YandexPayApiRequest::new()
            .url(url)
            .api_key(self.api_key.clone())
            .method(Method::Post)
            .body(Some(bytes.into()))
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }
    /// Запрос на возврат средств за заказ.
    ///
    /// Доступно только для платежей в статусе CAPTURED и PARTIALLY_REFUNDED. В случае успешного выполнения запроса изменится статус платежа:
    ///
    ///     на REFUNDED, если был произведен полный возврат;
    ///
    ///     на PARTIALLY_REFUNDED, если после совершения возврата в заказе остались ещё товары.
    ///
    /// Метод является асинхронным.
    ///
    /// Узнать результат возврата можно через механизм событий или вызов метода состояния операции. Событие OPERATION_STATUS_UPDATED будет отправлено как в случае успеха, так и при возникновении ошибки в процессе совершения возврата.
    ///
    /// Для выполнения полного возврата достаточно передать refundAmount, равный сумме заказа.
    ///
    /// Для выполнения частичного возврата дополнительно нужно передать итоговую корзину предоставляемых товаров и услуг. Сформировать итоговую корзину можно одним из способов:
    ///     передать целевое состояние корзины после выполнения возврата с помощью поля targetCart. Если это поле не указано, то считается, что корзина возвращается полностью.
    ///
    ///     Поле targetShipping применимо только к Yandex Pay Checkout. В остальных случаях следует оставить это поле пустым. Если это поле не указано, то считается, что стоимость доставки возвращается полностью.
    ///     
    ///     передать данные о товарах, подлежащих возврату, с помошью поля refundCart: в поле укажите, сколько единиц товара нужно вернуть или на какую сумму следует уменьшить стоимость товара. Если поле не указано, возврат осуществляется на всю корзину.
    ///
    /// Примечание
    ///
    ///     Для данной стратегии рекомендуется указывать идентификатор операции externalOperationId, который служит токеном идемпотентности. Это позволит избежать риска повторных возвратов.
    pub async fn refund_order(
        &self,
        order_id: impl Into<String>,
        request: RefundRequest,
    ) -> R<OperationResponseData> {
        let url = format!(
            "{}/api/merchant/v2/orders/{}/refund",
            self.base_url,
            order_id.into()
        );
        let bytes = serde_json::to_vec(&request)?;
        let r = YandexPayApiRequest::new()
            .url(url)
            .api_key(self.api_key.clone())
            .method(Method::Post)
            .body(Some(bytes.into()))
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }
    /// Запрос на списание средств за заказ.
    ///
    /// Списание заблокированных средств. Доступно только для платежей в статусе AUTHORIZED. При успешном результате запроса статус платежа изменится на CAPTURED.
    ///
    /// В случае передачи суммы подтверждения меньшей, чем оригинальная, оставшаяся часть платежа будет возвращена. В данном случае нужно передать итоговую корзину предоставляемых товаров и услуг. Итоговая корзина должна формироваться из текущей корзины исключением некоторых позиций, по которым производился возврат.
    pub async fn capture_order(
        &self,
        order_id: impl Into<String>,
        request: CaptureOrderRequest,
    ) -> R<OperationResponseData> {
        let url = format!(
            "{}/api/merchant/v1/orders/{}/capture",
            self.base_url,
            order_id.into()
        );
        let bytes = serde_json::to_vec(&request)?;
        let r = YandexPayApiRequest::new()
            .url(url)
            .api_key(self.api_key.clone())
            .method(Method::Post)
            .body(Some(bytes.into()))
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }
    /// Запрос на отмену платежа.
    ///
    /// Доступно для платежей в любом статусе. Запрещает дальнейшую оплату заказа, а также, если оплата уже произошла, производит полный возврат средств клиенту. В случае успеха статус платежа изменится на FAILED.
    pub async fn rollback_order(&self, order_id: impl Into<String>) -> R<serde_json::Value> {
        let url = format!(
            "{}/api/merchant/v1/orders/{}/rollback",
            self.base_url,
            order_id.into()
        );
        let r = YandexPayApiRequest::new()
            .url(url)
            .api_key(self.api_key.clone())
            .method(Method::Post)
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }

    /// Запрос на подтверждение оплаты для Сплита с оплатой при получении.
    ///
    /// Доступно для платежей в статусе CONFIRMED. При успешном результате запроса статус изменится на CAPTURED.
    ///
    /// Если состав корзины на этапе подтверждения заказа отличается от состава корзины на этапе оформления, передайте новые значения для полей cart и orderAmount.
    pub async fn submit_order(
        &self,
        order_id: impl Into<String>,
        request: SubmitRequest,
    ) -> R<OperationResponseData> {
        let url = format!(
            "{}/api/merchant/v1/orders/{}/submit",
            self.base_url,
            order_id.into()
        );
        let bytes = serde_json::to_vec(&request)?;
        let r = YandexPayApiRequest::new()
            .url(url)
            .api_key(self.api_key.clone())
            .method(Method::Post)
            .body(Some(bytes.into()))
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }

    pub async fn get_operation(
        &self,
        external_operation_id: impl Into<String>,
    ) -> R<OperationResponseData> {
        let url = format!(
            "{}/api/merchant/v1/operations/{}",
            self.base_url,
            external_operation_id.into()
        );
        let r = YandexPayApiRequest::new()
            .url(url)
            .api_key(self.api_key.clone())
            .method(Method::Get)
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }
    /// Запрос на создание подписки.
    ///
    /// Используется для создания подписки и получения ссылки для ее оформления.
    pub async fn create_subscription(
        &self,
        subscription: CreateSubscriptionRequest,
    ) -> Result<CreateSubscriptionResponseData, YandexPayApiError> {
        let url = format!("{}/api/merchant/v1/subscriptions", self.base_url);
        let bytes = serde_json::to_vec(&subscription)?;
        let r = YandexPayApiRequest::new()
            .url(url)
            .api_key(self.api_key.clone())
            .method(Method::Post)
            .body(Some(bytes.into()))
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }

    /// Запрос для очередного списания по подписке.
    ///
    /// Запрос используется для безакцептного списания денежных средств с привязанного к подписке счета или карты. Для списания нужно передать номер заказа, корзину, сумму списания, и номер стартового заказа, который был использован при создании подписки.
    pub async fn recur_subscription(
        &self,
        subscription: CreateRecurrentChargeRequest,
    ) -> Result<RecurSubscriptionResponseData, YandexPayApiError> {
        let url = format!("{}/api/merchant/v1/subscriptions/recur", self.base_url);
        let bytes = serde_json::to_vec(&subscription)?;
        let r = YandexPayApiRequest::new()
            .url(url)
            .api_key(self.api_key.clone())
            .method(Method::Post)
            .body(Some(bytes.into()))
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }

    ///Запрос на получение информации по подписке.
    ///
    /// Возвращает идентификатор подписки, состояние подписки и привязанного способа оплаты.
    pub async fn get_subscription(
        &self,
        // ID подписки
        customer_subscription_id: impl Into<String>,
        request: GetSubscriptionRequest,
    ) -> Result<CustomerSubscriptionResponseData, YandexPayApiError> {
        let url = format!(
            "{}/api/merchant/v1/subscriptions/{}",
            self.base_url,
            customer_subscription_id.into()
        );
        let bytes = serde_json::to_vec(&request)?;
        let r = YandexPayApiRequest::new()
            .url(url)
            .api_key(self.api_key.clone())
            .method(Method::Get)
            .body(Some(bytes.into()))
            .build();
        let response = self.client.send(r).await?;
        Ok(response)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Post,
}

#[derive(Debug, Clone, Builder)]
pub struct YandexPayApiRequest {
    #[default(None)]
    //serialized body
    pub body: Option<Bytes>,
    #[default(Method::Get)]
    //Method
    pub method: Method,
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
#[cfg(feature = "reqwest")]
impl HttpClient for reqwest::Client {
    fn send<T: serde::de::DeserializeOwned>(
        &self,
        request: YandexPayApiRequest,
    ) -> impl Future<Output = R<T>> {
        let client = self.clone();

        async move {
            let body = request.body.clone();
            let method = match request.method {
                Method::Get => reqwest::Method::GET,
                Method::Post => reqwest::Method::POST,
            };
            let mut request_builder = client
                .request(method, &*request.url)
                .header("Authorization", format!("Api-Key {}", request.api_key))
                .header("X-Request-Id", &*request.request_id)
                .header("X-Request-Timeout", request.request_timeout.to_string())
                .header("X-Request-Attempt", request.request_attempt.to_string())
                .header("Content-Type", "application/json");
            if let Some(body) = body {
                request_builder = request_builder.body(body);
            }
            let response = request_builder.send().await?;

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
