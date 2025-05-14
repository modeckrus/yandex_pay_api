use crate::serde_help::*;
use builder_pattern::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// Тело запроса на возврат
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v2_refund-post#body>
pub struct RefundRequest {
    #[serde(with = "string_as_float")]
    /// Сумма к возврату
    /// Example: 123.45
    pub refund_amount: f64,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Идентификатор точки продаж
    /// Max length: 2048
    pub branch_id: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    ///  Идентификатор операции возврата в системе продавца. Должен быть уникальным.
    /// Передайте этот параметр, чтобы получить возможность отслеживать состояние операции возврата через метод operations/{external_operation_id}.
    /// Если операция не завершена (обрабатывается или остановлена), то повторный вызов метода возврата с такими же аргументами и таким же значением externalOperationId будет идемпотентным: в ответе вернется та же операция. Иначе вернется ошибка.
    /// Если процесс возврата был успешно запущен, то повторный вызов метода возврата с тем же externalOperationId вернет ошибку с "reasonCode": "DUPLICATE_EXTERNAL_OPERATION_ID".
    pub external_operation_id: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Идентификатор менеджера
    /// Max length: 2048
    pub manager_id: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Причина возврата
    /// Max length: 2048
    pub motive: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Описывает позиции корзины, которые нужно вернуть.
    pub refund_cart: Option<TargetCart>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Описывает итоговое состояние корзины после выполнения возврата.
    pub target_cart: Option<TargetCart>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Описывает итоговое состояние доставки после выполнения возврата.
    pub target_shipping: Option<TargetShipping>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// Описывает итоговое состояние корзины
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v2_refund-post#targetcart>
pub struct TargetCart {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Позиции корзины
    pub items: Vec<TargetCartItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// Позиция корзины
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v2_refund-post#targetcartitem>
pub struct TargetCartItem {
    #[into]
    /// Идентификатор позиции в корзине на момент создания заказа.
    /// Max length: 2048
    pub product_id: String,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Цена одной единицы товара/услуги после выполнения операции.
    /// Необходимо указать, если цена одной единицы уменьшается в результате операции.
    /// Это может быть полезным, если необходимо вернуть часть денег за товар или подтверждении заказа.
    /// Если не указывать это поле в запросе, то считается, что цена осталась прежней.
    /// Example: 123.45
    pub price: Option<f64>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Количество единиц товара/услуги, которое останется у пользователя после выполнения операции.
    /// Если не указывать это поле в запросе, то считается, что количество не изменилось.
    /// Example: 123.45
    pub quantity_count: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// Описывает итоговое состояние доставки
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v2_refund-post#targetshipping>
pub struct TargetShipping {
    #[serde(with = "string_as_float")]
    /// Стоимость доставки после выполнения операции
    /// Example: 123.45
    pub amount: f64,
}
