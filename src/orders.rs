use crate::serde_help::*;
use crate::*;
use builder_pattern::Builder;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#body>
pub struct CreateOrderRequest {
    /// Корзина
    pub cart: RenderedCart,
    /// Трехбуквенный код валюты заказа (ISO 4217)
    #[default(CurrencyCode::Rub)]
    pub currency_code: CurrencyCode,
    #[into]
    /// Идентификатор заказа на стороне продавца (должен быть уникальным). Дальнейшее взаимодействие по заявке на оплату будет осуществляться с использованием этого идентификатора. Также данный идентификатор будет использоваться в сверках
    pub order_id: String,
    #[default(vec![AvailablePaymentMethod::Card])]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Доступные методы оплаты на платежной форме Яндекс Пэй.
    /// Если вы интегрируете оплату только одним методом, например, Карта — указывается один метод ["CARD"]. Для платежей по банковским картам и через Сплит необходимо передать: ["CARD", "SPLIT"].
    pub available_payment_methods: Vec<AvailablePaymentMethod>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Номер телефона клиента.
    /// Используется для упрощения авторизации, а также может увеличить вероятность одобрения по Сплиту.
    /// Для номера телефона предлагаем использовать формат +71234567890 или 71234567890. В строке допустимо наличие других символов, однако все символы, кроме цифр, игнорируются. Если номер телефона начинается с 8, то он обрабатывается как аналогичный номер, где 8 заменена на +7.
    /// Например, такие переданные значения 71234567890, +71234567890, 81234567890, +7 (123) 456-78-90 будут обрабатываться как 71234567890.
    pub billing_phone: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Дополнительные параметры для оформления офлайн заказа
    pub extensions: Option<OrderExtensions>,
    #[default(false)]
    /// Проводить ли заказ по флоу "отложенно оплаты".
    /// Если параметр установлен в true, то заказ будет оформлятся как заказ с отложенной оплатой, смотри раздел "Способы оплаты и платёжные механики".
    pub is_prepayment: bool,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Произвольные данные по заказу для внутреннего использования
    /// Max length: 2048
    pub metadata: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Поверхность на которой инициализировали создание заказа
    /// Необходимо для последующей аналитики
    /// WEBSITE: Кнопка размещена на сайте. Ссылка на оплату сформировалась после действий (нажатия кнопки) пользователя на сайте
    /// APP: Кнопка размещена в мобильном приложении. Ссылка на оплату сформировалась после действий (нажатия кнопки) пользователя в приложении
    /// CRM: Ссылка на оплату сформирована менеджером в CRM или другой админке
    /// CASH_REGISTER: Ссылка на оплату сформирована для отображения на оффлайн-кассе
    /// CMS_PLUGIN: Ссылка на оплату сформирована в плагине для CMS
    pub order_source: Option<OrderSource>,
    #[default(Some(PreferredPaymentMethod::FullPayment))]
    /// Предпочтительный метод оплаты.
    /// Переданный метод будет автоматически выбран на форме оплаты, если это не противоречит доступным методам оплаты. По умолчанию - Карта.
    pub preferred_payment_method: Option<PreferredPaymentMethod>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Конструктор предвыбранного плана для оплаты в Сплит.
    /// Доступно только по согласованию
    pub public_constructor: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Назначение платежа
    pub purpose: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ссылки для переадресации пользователя с формы оплаты. Обязательно для онлайн продавца
    pub redirect_urls: Option<MerchantRedirectUrls>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Дополнительная информация, наличие которой может увеличить вероятность одобрения по Сплиту. Доступно в полной мере только для онлайн-магазинов.
    pub risk: Option<MerchantRiskInfo>,
    #[default(Some(1800))]
    /// Параметр определяет время, в течение которого пользователь может воспользоваться ссылкой на форму оплаты заказа.
    /// По истечении времени пользователь будет видеть сообщение "Вышло время оплаты".
    /// Если пользователь воспользовался ссылкой до истечения времени, то на завершение оплаты отводится дополнительно 15 минут. Если по истечении дополнительного времени оплата не произошла, заказ считается не оплаченным окончательно.
    /// Это отражается в изменении статуса заказа Order.paymentStatus на значение FAILED. Также выполняется отправка уведомления об изменении статуса заказа. Если продавец бронировал товар, то после изменения статуса заказ можно отменить.
    /// Время жизни (сек): 180 <= ttl <= 604800
    /// Время жизни (при создании ссылки в личном кабинете, сек): 86400
    pub ttl: Option<u32>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Дополнительные данные uniqr
    pub uniqr_metadata: Option<UniqrMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#renderedcart>
pub struct RenderedCart {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Корзина товаров, которую оплачивает покупатель.
    pub items: Vec<RenderedCartItem>,
    /// Итоговая информация о стоимости заказа.
    pub total: CartTotal,
    #[into]
    /// Переданный продавцом идентификатор корзины
    pub external_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#renderedcartitem>
pub struct RenderedCartItem {
    #[into]
    /// Id товара в системе продавца. В параметрах запроса каждый идентификатор товара productId должен быть уникальным
    pub product_id: String,
    /// Количество товара в заказе
    pub quantity: ItemQuantity,
    #[into]
    // Наименование товара. Max length: 2048
    pub title: String,
    #[into]
    #[serde(with = "string_as_float")]
    /// Суммарная цена за позицию с учётом скидок на позицию. Example: 123.45
    pub total: f64,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Описание товара. Max length: 2048
    pub description: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Цена за единицу товара с учётом скидок на позицию. Example: 123.45
    pub discounted_unit_price: Option<f64>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Промо параметры товара
    pub features: Option<CartItemFeatures>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Количество баллов Плюса. Поле только для чтения. Example: 123.45
    pub points_amount: Option<f64>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Данные для формирования чека
    pub receipt: Option<ItemReceipt>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Суммарная цена за позицию без учета скидок. Example: 123.45
    pub subtotal: Option<f64>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Полная цена за единицу товара без учетка скидки. Example: 123.45
    pub unit_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#carttotal>
pub struct CartTotal {
    #[into]
    #[serde(with = "string_as_float")]
    pub amount: f64,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Количество баллов Плюса
    /// Поле только для чтения. Переданные значения будут проигнорированы.
    /// Example: 123.45
    pub points_amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#billingreport>
pub struct BillingReport {
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Обязательное поле только для офлайн-магазинов. Идентификатор точки продаж
    /// Max length: 2048
    pub branch_id: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Обязательное поле только для офлайн-магазинов. Идентификатор менеджера
    /// Max length: 2048
    pub manager_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#qrdata>
pub struct QRData {
    #[into]
    /// Max length: 2048
    pub token: String, //QR token
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#smsoffer>
pub struct SMSOffer {
    #[into]
    /// Номер телефона клиента (пример +71234567890) для СМС-информирования. Только для merchantId офлайн-магазина.
    /// Max length: 2048
    /// Pattern: ^\+\d+$
    pub phone: String, // Customer's phone number (e.g., +71234567890) for SMS notifications. Only for merchantId of offline stores.
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#customeraggregates>
pub struct CustomerAggregates {
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Сумма первого успешного заказа
    pub amount_first_successful_order: Option<f64>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Сумма последнего успешного заказа
    pub amount_latest_successful_order: Option<f64>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Куки. Max length: 2048
    pub cookie: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]

    /// Количество дней с момента последнего сброса пароля     
    pub days_since_last_password_reset: Option<i32>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]

    /// Количество неуспешных попыток входа за последний день
    pub failed_login_attempts_one_day: Option<i32>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]

    /// Количество неуспешных попыток входа за последние 7 дней
    pub failed_login_attempts_seven_days: Option<i32>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Дата первого успешного заказа в истории
    pub first_successful_order_date: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Вход с сохраненной куки (ранее уже оплачивал заказ)
    pub historical_cookie_login: Option<bool>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Вход с сохраненного устройства (ранее уже оплачивал заказ)
    pub historical_device_login: Option<bool>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Дата последнего сброса пароля
    pub last_password_reset_date: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Дата последнего успешного заказа за последний год
    pub latest_successful_order_last_year_date: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Был ли у пользователя успешный заказ по указанному адресу ранее
    pub previous_successful_orders_at_same_address: Option<bool>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_string_as_float")]
    /// Процент выкупа за последние полгода
    pub redemption_rate_last_half_year: Option<f64>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Дата регистрации на сайте мерчанта
    pub registration_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#periodcheckaggregates>
pub struct PeriodCheckAggregates {
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Количество успешных заказов за последние 9 месяцев
    pub successful_orders_count_nine_months: Option<i32>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Количество успешных заказов за последний месяц
    pub successful_orders_count_one_month: Option<i32>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Количество успешных заказов за последние 6 месяцев
    pub successful_orders_count_six_months: Option<i32>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Количество успешных заказов за последние 3 месяца
    pub successful_orders_count_three_months: Option<i32>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Количество успешных заказов за последние 12 месяцев
    pub successful_orders_count_twelve_months: Option<i32>,
    #[serde(with = "option_string_as_float")]
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Сумма успешных заказов за последние 9 месяцев. Example: 123.45
    pub total_amount_successful_orders_nine_months: Option<f64>,
    #[serde(with = "option_string_as_float")]
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Сумма успешных заказов за последний месяц. Example: 123.45
    pub total_amount_successful_orders_one_month: Option<f64>,
    #[serde(with = "option_string_as_float")]
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Сумма успешных заказов за последние 6 месяцев. Example: 123.45
    pub total_amount_successful_orders_six_months: Option<f64>,
    #[serde(with = "option_string_as_float")]
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Сумма успешных заказов за последние 3 месяца. Example: 123.45
    pub total_amount_successful_orders_three_months: Option<f64>,
    #[serde(with = "option_string_as_float")]
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Сумма успешных заказов за последние 12 месяцев. Example: 123.45
    pub total_amount_successful_orders_twelve_months: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#itemquantity>
pub struct ItemQuantity {
    #[serde(with = "string_as_float")]
    #[into]
    /// Количество товара в заказе
    pub count: f64,
    #[serde(with = "string_as_float")]
    #[into]
    /// Максимально доступное количество товара
    pub available: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#cartitemfeatures>
pub struct CartItemFeatures {
    #[default(Some(false))]
    /// Не распределять баллы Плюса. Default: false
    pub points_disabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#itemreceipt>
pub struct ItemReceipt {
    pub tax: Tax,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,
    #[serde(with = "option_string_as_float")]
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Не должно содержать больше двух знаков после запятой. Например: 1.12, 5.1, 10, 11.00 .
    pub excise: Option<f64>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_quantity: Option<MarkQuantity>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure: Option<Measure>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<PaymentMethodType>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_subject_type: Option<PaymentSubjectType>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>, // Base64 encoded string (1 to 32 bytes)
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier: Option<Supplier>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>, // Max length: 2048
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#agent>
pub struct Agent {
    /// Признак агента по предмету расчёта
    pub agent_type: AgentType,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Max length: 2048
    pub operation: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments_operator: Option<PaymentsOperator>,
    /// Max length: 2048
    #[default(vec![])]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub phones: Vec<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_operator: Option<TransferOperator>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#markquantity>
pub struct MarkQuantity {
    #[into]
    pub denominator: i32,
    #[into]
    pub numerator: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#supplier>
pub struct Supplier {
    #[into]
    /// Max length: 2048
    pub inn: String,
    #[into]
    /// Max length: 2048
    pub name: String,
    #[default(vec![])]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Max length: 2048
    pub phones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#paymentsoperator>
pub struct PaymentsOperator {
    #[default(vec![])]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Max length: 2048
    pub phones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#transferoperator>
pub struct TransferOperator {
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Max length: 2048
    pub address: Option<String>,
    #[into]
    ///  Max length: 2048
    pub inn: String,
    #[into]
    /// Max length: 2048
    pub name: String,
    #[default(vec![])]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Max length: 2048
    pub phones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#renderedcart>
pub struct OrderExtensions {
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Информация о месте и авторе оформления заказа.
    pub billing_report: Option<BillingReport>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Дополнительные параметры для отправки ссылки на оплату с использованием QR
    pub qr_data: Option<QRData>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Дополнительные параметры для отправки ссылки на оплату с использованием SMS.
    /// Обязательное поле только для офлайн-магазинов.
    pub sms_offer: Option<SMSOffer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#merchantredirecturls>
pub struct MerchantRedirectUrls {
    #[into]
    /// Обязательное поле только для онлайн-магазинов.
    /// Ссылка для переадресации пользователя в случае возникновения ошибки во время оплаты,
    /// или если срок ссылки на оплату истек.
    pub on_error: String,
    #[into]
    /// Обязательное поле только для онлайн-магазинов. Ссылка для переадресации пользователя в случае успешной оплаты.
    pub on_success: String,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ссылка для переадресации пользователя в случае отмены процесса оплаты. Отмену оплаты осуществляет пользователь на форме для оплаты.
    pub on_abort: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#merchantriskinfo>
pub struct MerchantRiskInfo {
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Агрегированные данные о клиенте
    pub customer_aggregates: Option<CustomerAggregates>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Идентификатор устройства клиента (device_id/gaid/idfa/ifv). Max length: 2048
    pub device_id: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Признак экспресс-доставки
    pub is_express_shipping: Option<bool>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Агрегаты по чекам за различные периоды
    pub period_check_aggregates: Option<PeriodCheckAggregates>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Адрес доставки. Если выбран способ получения «самовывоз» (PICKUP), то нужно указать адрес пункта выдачи.
    pub shipping_address: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Номер телефона получателя (пример +71234567890)
    pub shipping_phone: Option<String>,
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Способ получения заказа
    pub shipping_type: Option<ShippingType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#uniqrmetadata>
pub struct UniqrMetadata {
    #[into]
    /// Идентификатор QR-кода. Max length: 2048
    pub uni_qr_id: String,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
/// Налог <https://pay.yandex.ru/docs/ru/custom/fns#tax>
pub enum Tax {
    /// НДС по ставке 20%
    Vat20 = 1,
    /// НДС по ставке 10%
    Vat10 = 2,
    /// НДС по расчетной ставке 20/120
    Vat20_120 = 3,
    /// НДС по расчетной ставке 10/110
    Vat10_110 = 4,
    /// НДС по ставке 0%
    Vat0 = 5,
    /// Без НДС
    NoVat = 6,
    /// НДС по ставке 7%
    Vat7 = 7,
    /// НДС по ставке 5%
    Vat5 = 8,
    /// НДС по расчетной ставке 7/107   
    Vat7_107 = 9,
    /// НДС по расчетной ставке 5/105
    Vat5_105 = 10,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
/// Значения paymentMethodType - «признак способа расчета» (тег 1214)
/// <https://pay.yandex.ru/docs/ru/custom/fns#payment-method-type>
pub enum PaymentMethodType {
    /// Полная предварительная оплата до момента передачи предмета расчета
    FullPrepayment = 1,
    /// Частичная предварительная оплата до момента передачи предмета расчета
    PartialPrepayment = 2,
    /// Аванс
    Advance = 3,
    /// Полная оплата в момент передачи предмета расчета
    FullPayment = 4,
    /// Частичная оплата предмета расчета в момент его передачи с последующей оплатой в кредит
    PartialPayment = 5,
    /// Передача предмета расчета без его оплаты в момент его передачи с последующей оплатой в кредит
    CreditTransfer = 6,
    /// Оплата предмета расчета после его передачи с оплатой в кредит
    CreditPayment = 7,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
/// Значения paymentSubjectType - «признак предмета расчета» (тег 1212)
/// <https://pay.yandex.ru/docs/ru/custom/fns#payment-subject-type>
pub enum PaymentSubjectType {
    /// Товар
    Goods = 1,
    /// Подакцизный товар
    ExciseGoods = 2,
    /// Работа
    Work = 3,
    /// Услуга
    Service = 4,
    /// Ставка азартной игры
    GamblingBet = 5,
    /// Выигрыш азартной игры
    GamblingWin = 6,
    /// Лотерейный билет
    LotteryTicket = 7,
    /// Выигрыш лотереи
    LotteryWin = 8,
    /// Предоставление РИД
    IntellectualProperty = 9,
    /// Платеж
    Payment = 10,
    /// Агентское вознаграждение
    AgencyFee = 11,
    /// Составной предмет расчета
    Composite = 12,
    /// Иной предмет расчета
    Other = 13,
    /// Имущественное право
    PropertyRight = 14,
    /// Внереализационныи доход
    NonOperatingIncome = 15,
    /// Страховые взносы: о суммах расходов, уменьшающих сумму налога (авансовых платежей) в соответствии с п. 3.1 статьи 346.21 НК РФ
    InsuranceContributionsTaxReduction = 16,
    /// Торговый сбор
    TradeFee = 17,
    /// Курортный сбор
    ResortFee = 18,
    /// Залог
    Deposit = 19,
    /// Расход: о суммах произведенных расходов в соответствии со статьей 346.16 НК РФ, уменьшающих доход
    ExpenseTaxReduction = 20,
    /// Взносы на обязательное пенсионное страхование ИП
    PensionInsuranceIP = 21,
    /// Взносы на обязательное пенсионное страхование
    PensionInsurance = 22,
    /// Взносы на обязательное медицинское страхование ИП
    MedicalInsuranceIP = 23,
    /// Взносы на обязательное медицинское страхование
    MedicalInsurance = 24,
    /// Взносы на обязательное социальное страхование
    SocialInsurance = 25,
    /// Платеж казино
    CasinoPayment = 26,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
/// Значения quantity.measure - «мера количества предмета расчета» (тег 2108)
/// <https://pay.yandex.ru/docs/ru/custom/fns#measure-code>
pub enum Measure {
    /// Штуки или единицы
    Units = 0,
    /// Грамм
    Gram = 10,
    /// Килограмм
    Kilogram = 11,
    /// Тонна
    Tonne = 12,
    /// Сантиметр
    Centimeter = 20,
    /// Дециметр
    Decimeter = 21,
    /// Метр
    Meter = 22,
    /// Квадратный сантиметр
    SquareCentimeter = 30,
    /// Квадратный дециметр
    SquareDecimeter = 31,
    /// Квадратный метр
    SquareMeter = 32,
    /// Миллилитр
    Milliliter = 40,
    /// Литр
    Liter = 41,
    /// Кубический метр
    CubicMeter = 42,
    /// Киловатт час
    KilowattHour = 50,
    /// Гигакалория
    Gigacalorie = 51,
    /// Сутки (день)
    Day = 70,
    /// Час
    Hour = 71,
    /// Минута
    Minute = 72,
    /// Секунда
    Second = 73,
    /// Килобайт
    Kilobyte = 80,
    /// Мегабайт
    Megabyte = 81,
    /// Гигабайт
    Gigabyte = 82,
    /// Терабайт
    Terabyte = 83,
    /// Применяется при использовании иных мер измерения
    Other = 255,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
/// Признак агента по предмету расчета (тег 1222)
/// <https://pay.yandex.ru/docs/ru/custom/fns#agent-type>
pub enum AgentType {
    /// Банковский платежный агент
    BankPaymentAgent = 1,
    /// Банковский платежный субагент
    BankPaymentSubagent = 2,
    /// Платежный агент
    PaymentAgent = 3,
    /// Платежный субагент
    PaymentSubagent = 4,
    /// Поверенный
    Attorney = 5,
    /// Комиссионер
    Commissioner = 6,
    /// Иной агент
    OtherAgent = 7,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ShippingType {
    #[serde(rename = "COURIER")]
    Courier,
    #[serde(rename = "PICKUP")]
    Pickup,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PreferredPaymentMethod {
    #[serde(rename = "FULLPAYMENT")]
    FullPayment,
    #[serde(rename = "SPLIT")]
    Split,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrderSource {
    #[serde(rename = "WEBSITE")]
    /// WEBSITE: Кнопка размещена на сайте. Ссылка на оплату сформировалась после действий (нажатия кнопки) пользователя на сайте
    Website,
    #[serde(rename = "APP")]
    /// APP: Кнопка размещена в мобильном приложении. Ссылка на оплату сформировалась после действий (нажатия кнопки) пользователя в приложении
    App,
    #[serde(rename = "CRM")]
    /// CRM: Ссылка на оплату сформирована менеджером в CRM или другой админке
    Crm,
    #[serde(rename = "CASH_REGISTER")]
    /// CASH_REGISTER: Ссылка на оплату сформирована для отображения на оффлайн-кассе
    CashRegister,
    #[serde(rename = "CMS_PLUGIN")]
    /// CMS_PLUGIN: Ссылка на оплату сформирована в плагине для CMS
    CmsPlugin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AvailablePaymentMethod {
    #[serde(rename = "CARD")]
    Card,
    #[serde(rename = "SPLIT")]
    Split,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub enum CurrencyCode {
    #[default]
    #[serde(rename = "RUB")]
    /// Russian Ruble
    /// ISO 4217 code: RUB
    Rub,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_orders-post#createorderresponsedata>
pub struct CreateOrderResponse {
    pub payment_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_create_order_request() {
        let json = r#"
        {
            "availablePaymentMethods": [],
            "billingPhone": "string",
            "cart": {
                "externalId": "string",
                "items": [
                    {
                        "description": "string",
                        "discountedUnitPrice": "123.45",
                        "features": {
                            "pointsDisabled": false
                        },
                        "pointsAmount": "123.46",
                        "productId": "string",
                        "quantity": {
                            "available": "123.45",
                            "count": "123.45"
                        },
                        "receipt": {
                            "agent": {
                                "agentType": 1,
                                "operation": "string",
                                "paymentsOperator": {
                                    "phones": [
                                        "string"
                                    ]
                                },
                                "phones": [
                                    "string"
                                ],
                                "transferOperator": {
                                    "address": "string",
                                    "inn": "string",
                                    "name": "string",
                                    "phones": [
                                        "string"
                                    ]
                                }
                            },
                            "excise": "123.45",
                            "markQuantity": {
                                "denominator": 0,
                                "numerator": 0
                            },
                            "measure": 0,
                            "paymentMethodType": 1,
                            "paymentSubjectType": 1,
                            "productCode": "string",
                            "supplier": {
                                "inn": "string",
                                "name": "string",
                                "phones": [
                                    "string"
                                ]
                            },
                            "tax": 1,
                            "title": "string"
                        },
                        "subtotal": "123.45",
                        "title": "string",
                        "total": "123.45",
                        "unitPrice": "123.45"
                    }
                ],
                "total": {
                    "amount": "123.45",
                    "pointsAmount": "123.45"
                }
            },
            "currencyCode": "RUB",
            "extensions": {
                "billingReport": {
                    "branchId": null,
                    "managerId": null
                },
                "qrData": {
                    "token": "string"
                },
                "smsOffer": {
                    "phone": "string"
                }
            },
            "isPrepayment": false,
            "metadata": "string",
            "orderId": "string",
            "orderSource": "WEBSITE",
            "preferredPaymentMethod": "FULLPAYMENT",
            "publicConstructor": "string",
            "purpose": "string",
            "redirectUrls": {
                "onAbort": "string",
                "onError": "string",
                "onSuccess": "string"
            },
            "risk": {
                "billingPhone": "string",
                "customerAggregates": {
                    "amountFirstSuccessfulOrder": "123.45",
                    "amountLatestSuccessfulOrder": "123.45",
                    "cookie": "string",
                    "daysSinceLastPasswordReset": 0,
                    "failedLoginAttemptsOneDay": 0,
                    "failedLoginAttemptsSevenDays": 0,
                    "firstSuccessfulOrderDate": "string",
                    "historicalCookieLogin": false,
                    "historicalDeviceLogin": false,
                    "lastPasswordResetDate": "string",
                    "latestSuccessfulOrderLastYearDate": "string",
                    "previousSuccessfulOrdersAtSameAddress": false,
                    "redemptionRateLastHalfYear": "123.45",
                    "registrationDate": "string"
                },
                "deviceId": "string",
                "isExpressShipping": false,
                "periodCheckAggregates": {
                    "successfulOrdersCountNineMonths": 0,
                    "successfulOrdersCountOneMonth": 0,
                    "successfulOrdersCountSixMonths": 0,
                    "successfulOrdersCountThreeMonths": 0,
                    "successfulOrdersCountTwelveMonths": 0,
                    "totalAmountSuccessfulOrdersNineMonths": "123.45",
                    "totalAmountSuccessfulOrdersOneMonth": "123.45",
                    "totalAmountSuccessfulOrdersSixMonths": "123.45",
                    "totalAmountSuccessfulOrdersThreeMonths": "123.45",
                    "totalAmountSuccessfulOrdersTwelveMonths": "123.45"
                },
                "shippingAddress": "string",
                "shippingPhone": "string",
                "shippingType": "COURIER"
            },
            "ttl": 1800,
            "uniqrMetadata": {
                "uniQrId": "string"
            }
        }
        "#;

        let parsed: CreateOrderRequest = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.billing_phone, Some("string".to_string()));
        assert_eq!(parsed.cart.external_id, "string".to_string());
        assert_eq!(parsed.currency_code, CurrencyCode::Rub);
        assert_eq!(parsed.order_id, "string".to_string());
    }

    #[test]
    fn test_create_order() {
        let request = CreateOrderRequest::new()
            .cart(
                RenderedCart::new()
                    .items(vec![])
                    .external_id("123")
                    .total(CartTotal::new().amount(100.0).build())
                    .build(),
            )
            .order_id("123")
            .build();
        let json = serde_json::to_string(&request).unwrap();
        println!("{}", json);
    }
}
