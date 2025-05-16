use crate::orders_id::OperationStatus;
use crate::*;
use crate::{orders_id::OperationType, serde_help::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    #[into]
    /// Причина отмены
    pub reason: String,
    #[default(None)]
    /// Идентификатор операции
    pub external_operation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationResponseData {
    pub operation: Operation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_cancel-post#operation>
pub struct Operation {
    /// Сумма операции в фиатной валюте
    pub amount: f64,
    /// Уникальный идентификатор операции
    pub operation_id: String,
    /// Тип операции
    pub operation_type: OperationType,
    /// Идентификатор заказа
    pub order_id: String,
    #[serde(with = "option_iso8601")]
    /// Дата и время создания операции
    pub created: Option<Time>,
    /// Идентификатор операции на стороне продавца
    pub external_operation_id: Option<String>,
    /// Дополнительные параметры операции
    pub params: Option<serde_json::Value>,
    /// Причина ошибки
    pub reason: Option<String>,
    /// Статус операции
    pub status: OperationStatus,
    #[serde(with = "option_iso8601")]
    /// Дата и время обновления операции
    pub updated: Option<Time>,
}
