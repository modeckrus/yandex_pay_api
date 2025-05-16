use builder_pattern::Builder;
use serde::{Deserialize, Serialize};

use crate::orders::*;
use crate::serde_help::*;

/// Запрос на рекуррентное списание по подписке
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/subscriptions/merchant_v1_subscriptions_recur-post#body>
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecurrentChargeRequest {
    #[into]
    /// Сумма списания
    #[serde(with = "string_as_float")]
    pub amount: f64,

    /// Корзина
    pub cart: RenderedCart,
    #[default(CurrencyCode::default())]
    /// Трехбуквенный код валюты заказа (ISO 4217)
    pub currency_code: CurrencyCode,

    #[into]
    /// Идентификатор заказа на стороне продавца (должен быть уникальным)
    pub order_id: String,

    #[into]
    /// Идентификатор стартового заказа
    pub parent_order_id: String,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Произвольные данные по заказу для внутреннего использования
    pub metadata: Option<String>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Назначение платежа
    pub purpose: Option<String>,
}

/// Ответ на списание подписки
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/subscriptions/merchant_v1_subscriptions_recur-post#recursubscriptionresponsedata>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurSubscriptionResponseData {
    pub operation_id: String,
}
