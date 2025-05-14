use builder_pattern::Builder;
use serde::{Deserialize, Serialize};

use crate::orders::*;
use crate::serde_help::*;

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/subscriptions/merchant_v1_subscriptions-post#body>
pub struct OrderSubscriptionRequest {
    /// Трехбуквенный код валюты заказа (ISO 4217)
    #[default(CurrencyCode::Rub)]
    pub currency_code: CurrencyCode,
    #[into]
    /// Идентификатор заказа на стороне продавца (должен быть уникальным)
    pub order_id: String,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Корзина
    pub cart: Option<RenderedCart>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Сумма, которую будет списана в будущем
    pub future_write_off_amount: Option<f64>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Количество периодов подписки
    pub interval_count: Option<i32>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Единица времени в которой будет периодичность подписки
    pub interval_unit: Option<IntervalUnit>,
    #[default(false)]
    /// Позволяет привязать карту пользователя без сформированной корзины товаров
    pub is_binding: bool,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Произвольные данные по заказу для внутреннего использования
    pub metadata: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поверхность на которой инициализировали создание заказа
    pub order_source: Option<OrderSource>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Назначение платежа
    pub purpose: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ссылки для переадресации пользователя с формы оплаты
    pub redirect_urls: Option<MerchantRedirectUrls>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Название подписки
    pub title: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Количество периодов триального периода
    pub trial_count: Option<i32>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Дата окончания пробного периода
    pub trial_end_at: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Единица времени в которой будет периодичность триального периода
    pub trial_unit: Option<IntervalUnit>,
    #[default(Some(1800))]
    /// Время жизни заказа (в секундах)
    pub ttl: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntervalUnit {
    Second,
    Day,
    Week,
    Month,
    Year,
}
