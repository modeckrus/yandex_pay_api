use crate::Time;
use crate::serde_help::*;
use builder_pattern::Builder;
use serde::{Deserialize, Serialize};

/// Запрос на рекуррентное списание по подписке
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/subscriptions/merchant_v1_customer_subscription-get#body>
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetSubscriptionRequest {
    #[into]
    pub check_card_active: bool,
}

/// Ответ на списание подписки
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/subscriptions/merchant_v1_subscriptions_recur-post#recursubscriptionresponsedata>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSubscriptionResponseData {
    pub status: SubscriptionStatus,
    pub subscription_plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_iso8601")]
    pub cancelled_at: Option<Time>,
    pub customer_subscription_id: String,
    pub is_card_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_iso8601")]
    pub next_write_off: Option<Time>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionStatus {
    New,
    Active,
    Cancelled,
    Expired,
}
