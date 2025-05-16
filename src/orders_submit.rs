use crate::{orders_capture::CartWithRequiredTotalWithoutFinalPrice, serde_help::*};
use builder_pattern::Builder;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_submit-post#body>
pub struct SubmitRequest {
    #[into]
    /// Идентификатор операции в системе продавца
    /// Max length: 2048
    pub external_operation_id: String,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Итоговая корзина
    pub cart: Option<CartWithRequiredTotalWithoutFinalPrice>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Итоговая сумма заказа. Равна cart.total.amount.
    /// Является обязательным полем, если передается cart.
    /// Example: 123.45
    pub order_amount: Option<f64>,
}
