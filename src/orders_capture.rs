use crate::{
    orders_id::{CartItemType, Coupon, Discount, Measurements, ShippingMethodType},
    serde_help::*,
    *,
};
use builder_pattern::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_capture-post#body>
pub struct CaptureOrderRequest {
    /// Итоговая корзина
    pub cart: CartWithRequiredTotalWithoutFinalPrice,
    /// Идентификатор операции
    pub external_operation_id: String,
    /// Сумма к списанию. Если не указана, будет списана итоговая стоимость переданной корзины
    #[serde(with = "option_string_as_float")]
    pub order_amount: Option<f64>,
    /// Итоговый способ доставки
    pub shipping: ShippingPrice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_capture-post#shippingprice>
pub struct ShippingPrice {
    #[serde(with = "string_as_float")]
    pub amount: f64,
    pub method_type: ShippingMethodType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_capture-post#cartwithrequiredtotalwithoutfinalprice>
pub struct CartWithRequiredTotalWithoutFinalPrice {
    /// Позиции корзины
    pub items: Vec<CartItemWithoutFinalPriceCamelCase>,
    /// Итоговая информация о стоимости заказа
    pub total: CartTotal,
    /// Внутренний идентификатор корзины Яндекс Пэй
    pub cart_id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[default(vec![])]
    /// Купоны, применённые к корзине
    pub coupons: Vec<Coupon>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[default(vec![])]
    /// Скидки, применённые к корзине
    pub discounts: Vec<Discount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[default(None)]
    /// Переданный продавцом идентификатор корзины
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Измерения корзины
    pub measurements: Option<Measurements>,
    #[into]
    #[default(CartVersion::default())]
    /// Версия корзины
    pub version: CartVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CartVersion {
    #[default]
    #[serde(rename = "VALID")]
    /// Версия корзины по умолчанию
    Valid,
    #[serde(rename = "SEMIVALID")]
    SemiValid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_capture-post#cartitemwithoutfinalpricecamelcase>
pub struct CartItemWithoutFinalPriceCamelCase {
    #[into]
    /// Id товара в системе продавца
    pub product_id: String,
    /// Количество товара в заказе
    pub quantity: ItemQuantity,
    /// Описание товара
    #[serde(skip_serializing_if = "Option::is_none")]
    #[default(None)]
    pub description: Option<String>,
    /// Цена за единицу товара с учётом скидок на позицию
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    #[default(None)]
    pub discounted_unit_price: Option<f64>,
    /// Промо параметры товара
    #[serde(skip_serializing_if = "Option::is_none")]
    #[default(None)]
    pub features: Option<CartItemFeatures>,
    /// Размеры и вес товара
    #[serde(skip_serializing_if = "Option::is_none")]
    #[default(None)]
    pub measurements: Option<Measurements>,
    /// Количество баллов Плюса
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    #[default(None)]
    pub points_amount: Option<f64>,
    /// Данные для формирования чека
    #[serde(skip_serializing_if = "Option::is_none")]
    #[default(None)]
    pub receipt: Option<ItemReceipt>,
    /// Суммарная цена за позицию без учета скидок
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    #[default(None)]
    pub subtotal: Option<f64>,
    /// Наименование товара
    #[serde(skip_serializing_if = "Option::is_none")]
    #[default(None)]
    pub title: Option<String>,
    /// Тип товара
    #[serde(skip_serializing_if = "Option::is_none")]
    #[default(None)]
    pub item_type: Option<CartItemType>,
    /// Полная цена за единицу товара без учёта скидки
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    #[default(None)]
    pub unit_price: Option<f64>,
}
