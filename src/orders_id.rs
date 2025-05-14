use crate::orders::ItemReceipt;
use crate::serde_help::*;
use crate::*;
use builder_pattern::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет данные ответа для получения деталей заказа.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#orderresponsedata>
pub struct OrderResponseData {
    pub delivery: Option<Delivery>,
    pub operations: Vec<OrderResponseOperation>,
    pub order: Option<BaseMerchantApiOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет данные о доставке.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#delivery>
pub struct Delivery {
    #[serde(with = "string_as_float")]
    pub price: f64,
    #[serde(with = "option_string_as_float")]
    pub actual_price: Option<f64>,
    #[serde(with = "option_iso8601")]
    pub created: Option<Time>,
    pub status: DeliveryStatus,
    #[serde(with = "option_iso8601")]
    pub updated: Option<Time>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные статусы доставки.
pub enum DeliveryStatus {
    New,
    Estimating,
    Expired,
    ReadyForApproval,
    Collecting,
    Preparing,
    Delivering,
    Delivered,
    Returning,
    Returned,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет данные операции в ответе на запрос деталей заказа.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#orderresponseoperation>
pub struct OrderResponseOperation {
    #[serde(with = "string_as_float")]
    pub amount: f64,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[serde(rename = "operationType")]
    pub operation_type: OperationType,
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub approval_code: Option<String>,
    #[serde(with = "option_iso8601")]
    pub created: Option<Time>,
    pub external_operation_id: Option<String>,
    pub params: Option<serde_json::Value>,
    pub reason: Option<String>,
    pub status: OperationStatus,
    #[serde(with = "option_iso8601")]
    pub updated: Option<Time>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные типы операций.
pub enum OperationType {
    Authorize,
    BindCard,
    Refund,
    Capture,
    Void,
    Recurring,
    Prepayment,
    Submit,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные статусы операции.
pub enum OperationStatus {
    Pending,
    Success,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет данные заказа.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#basemerchantapiorder>
pub struct BaseMerchantApiOrder {
    /// Корзина
    pub cart: Cart,
    #[serde(default)]
    /// Трехбуквенный код валюты заказа (ISO 4217)
    /// Максимальная длина: 2048
    /// Пример: `RUB`
    pub currency_code: CurrencyCode,

    /// Дата и время создания заказа (ISO 8601)
    #[serde(with = "option_iso8601")]
    pub created: Option<Time>,
    #[serde(default)]
    /// Флаг, идет ли заказ по флоу "отложенной оплаты"
    /// Значение по умолчанию: `false`
    pub is_prepayment: bool,

    /// Идентификатор продавца
    /// Максимальная длина: 2048
    pub merchant_id: Option<String>,

    /// Произвольные данные, переданные при инициализации кнопки
    /// Максимальная длина: 2048
    pub metadata: Option<String>,

    /// Полная стоимость заказа к оплате с учётом возвратов, доставки, скидок и промокодов
    /// Пример: `123.45`
    #[serde(with = "string_as_float")]
    pub order_amount: f64,

    /// Id существующего заказа на стороне продавца, переданный при инициализации кнопки
    /// Максимальная длина: 2048
    pub order_id: String,

    /// Выбранный способ оплаты
    pub payment_method: Option<PaymentMethod>,

    /// Статус оплаты
    /// Возможные значения: `PENDING`, `AUTHORIZED`, `CAPTURED`, `VOIDED`, `REFUNDED`, `CONFIRMED`, `PARTIALLY_REFUNDED`, `FAILED`, `null`
    pub payment_status: Option<PaymentStatus>,

    /// Ссылка на оплату заказа
    /// Максимальная длина: 2048
    pub payment_url: Option<String>,

    /// Причина (применимо для статусов VOIDED/FAILED)
    /// Максимальная длина: 2048
    pub reason: Option<String>,

    /// Выбранный способ доставки
    pub shipping_method: Option<ShippingMethod>,

    /// Дата и время обновления заказа (ISO 8601)
    #[serde(with = "option_iso8601")]
    pub updated: Option<Time>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные статусы оплаты.
pub enum PaymentStatus {
    Pending,
    Authorized,
    Captured,
    Voided,
    Refunded,
    Confirmed,
    PartiallyRefunded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет данные корзины.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#cart>
pub struct Cart {
    /// Позиции корзины
    pub items: Vec<CartItem>,

    /// Внутренний идентификатор корзины Яндекс Пэй.
    /// Максимальная длина: 2048
    pub cart_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Купоны, применённые к корзине
    pub coupons: Vec<Coupon>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Скидки, применённые к корзине
    pub discounts: Vec<Discount>,

    /// Переданный продавцом идентификатор корзины
    /// Максимальная длина: 2048
    pub external_id: Option<String>,

    /// Измерения корзины
    pub measurements: Option<Measurements>,

    /// Итоговая стоимость корзины, которая пойдет в оплату
    pub total: CartTotal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Представляет данные о выбранном способе оплаты.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#paymentmethod>
pub struct PaymentMethod {
    /// Тип способа оплаты
    pub method_type: MethodType,

    /// Последние 4 цифры карты
    /// Максимальная длина: 2048
    pub card_last4: Option<String>,

    /// Платежная система
    pub card_network: Option<CardNetwork>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные типы способов оплаты.
pub enum MethodType {
    Card,
    Split,
    Sbp,
    SplitSbp,
    CashOnDelivery,
    CardOnDelivery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные платежные системы.
pub enum CardNetwork {
    Amex,
    Discover,
    Jcb,
    Mastercard,
    Maestro,
    Visaelectron,
    Visa,
    Mir,
    Unionpay,
    Uzcard,
    Humocard,
    Unknown,
    Undefined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Представляет данные о выбранном способе доставки.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#shippingmethod>
pub struct ShippingMethod {
    /// Тип способа доставки
    pub method_type: ShippingMethodType,

    /// Опции курьерской доставки
    pub courier_option: Option<CourierOption>,

    /// Опции самовывоза
    pub pickup_option: Option<PickupOption>,

    /// Опции доставки Яндекс
    pub yandex_delivery_option: Option<YandexDeliveryOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные типы способов доставки.
pub enum ShippingMethodType {
    Direct,
    Pickup,
    Courier,
    YandexDelivery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет данные о позиции корзины.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#cartitem>
pub struct CartItem {
    /// Id товара в системе продавца.
    /// Максимальная длина: 2048
    pub product_id: String,

    /// Количество товара в заказе.
    pub quantity: ItemQuantity,

    /// Описание товара.
    /// Максимальная длина: 2048
    pub description: Option<String>,

    /// Цена за единицу товара с учётом скидок на позицию.
    /// Пример: `123.45`
    #[serde(with = "option_string_as_float")]
    pub discounted_unit_price: Option<f64>,

    /// Промо параметры товара.
    pub features: Option<CartItemFeatures>,

    /// Цена за единицу товара с учётом всех скидок на позицию и на корзину.
    /// Пример: `123.45`
    #[serde(with = "option_string_as_float")]
    pub final_price: Option<f64>,

    /// Размеры и вес товара. Обязательно для товара типа PHYSICAL.
    pub measurements: Option<Measurements>,

    /// Количество баллов Плюса.
    /// Поле только для чтения. Переданные значения будут проигнорированы.
    /// Пример: `123.45`
    #[serde(with = "option_string_as_float")]
    pub points_amount: Option<f64>,

    /// Данные для формирования чека.
    pub receipt: Option<ItemReceipt>,

    /// Суммарная цена за позицию без учета скидок.
    /// Пример: `123.45`
    #[serde(with = "option_string_as_float")]
    pub subtotal: Option<f64>,

    /// Наименование товара.
    /// Максимальная длина: 2048
    pub title: Option<String>,

    /// Суммарная цена за позицию с учётом скидок на позицию.
    /// Пример: `123.45`
    #[serde(with = "option_string_as_float")]
    pub total: Option<f64>,

    /// Тип товара. Важен для интеграции с доставками.
    /// Значение по умолчанию: `UNSPECIFIED`
    pub item_type: Option<CartItemType>,

    /// Полная цена за единицу товара без учёта скидки.
    /// Пример: `123.45`
    #[serde(with = "option_string_as_float")]
    pub unit_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные типы товара.
pub enum CartItemType {
    Physical,
    Digital,
    Unspecified,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// Представляет данные о купоне.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#coupon>
pub struct Coupon {
    #[into]
    /// Код купона.
    /// Максимальная длина: 2048
    pub value: String,
    #[default(None)]
    /// Описание купона. Например, "Скидка 3%".
    /// Максимальная длина: 2048
    pub description: Option<String>,
    #[default(None)]
    /// Статус купона.
    pub status: Option<CouponStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные статусы купона.
pub enum CouponStatus {
    Valid,
    Invalid,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// Представляет данные о скидке.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#discount>
pub struct Discount {
    #[into]
    /// Сумма скидки.
    /// Пример: `123.45`
    #[serde(with = "string_as_float")]
    pub amount: f64,
    #[into]
    /// Текстовое описание.
    /// Максимальная длина: 2048
    pub description: String,
    #[into]
    /// Идентификатор скидки в системе мерчанта.
    /// Максимальная длина: 2048
    pub discount_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
/// Представляет измерения.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#measurements>
pub struct Measurements {
    #[into]
    /// Высота, в метрах.
    pub height: f64,
    #[into]
    /// Длина, в метрах.
    pub length: f64,
    #[into]
    /// Вес, в килограммах.
    pub weight: f64,
    #[into]
    /// Ширина, в метрах.
    pub width: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет опции курьерской доставки.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#courieroption>
pub struct CourierOption {
    /// Стоимость доставки
    /// Пример: `123.45`
    #[serde(with = "string_as_float")]
    pub amount: f64,

    /// Категория доставки
    pub category: CourierCategory,

    /// ID выбранного варианта доставки в системе продавца
    /// Максимальная длина: `2048`
    pub courier_option_id: String,

    /// Название способа доставки. Показывается пользователю в списке вариантов
    /// Максимальная длина: `2048`
    pub title: String,

    /// Индивидуальные методы оплаты для метода доставки. Этот параметр нужно использовать, если нужно ограничить методы оплаты, указанные в `availablePaymentMethods`. Если параметр не указан, то используются все методы оплаты, перечисленные в `availablePaymentMethods`.
    /// Возможные значения: `CARD`, `SPLIT`, `CASH_ON_DELIVERY`, `CARD_ON_DELIVERY`
    #[serde(default = "Vec::new")]
    pub allowed_payment_methods: Vec<AllowedPaymentMethodType>,

    /// Выбранные пользователем дата и интервал. Только для `type: FLEXIBLE`
    pub customer_choice: Option<FlexibleCustomerChoice>,

    /// Ближайшая дата доставки для `type: PLAIN`. Начало интервала выбора даты доставки для `type: FLEXIBLE`
    #[serde(with = "option_iso8601")]
    pub from_date: Option<Time>,

    /// Начало интервала времени доставки. Только для `type: PLAIN`
    pub from_time: Option<String>,

    /// Тип службы доставки
    pub provider: Option<DeliveryProvider>,

    /// Чек на доставку
    pub receipt: Option<ItemReceipt>,

    /// Кодирует интервалы времени доставки, доступные для выбора. Только для `type: FLEXIBLE`
    pub time_intervals: Option<FlexibleTimeIntervals>,

    /// Самая поздняя дата доставки для `type: PLAIN`. Конец интервала выбора даты доставки для `type: FLEXIBLE`
    #[serde(with = "option_iso8601")]
    pub to_date: Option<Time>,

    /// Конец интервала времени доставки. Только для `type: PLAIN`
    pub to_time: Option<String>,
    #[serde(default)]
    /// Тип опции. Для `FLEXIBLE` вариантов доставки пользователю дается возможность выбрать желаемые дату и интервал:
    ///
    /// - Дата доставки выбирается покупателем в отрезке `[fromDate, toDate]`
    /// - Чтобы предоставить пользователю выбор интервала в течение дня, заполните `timeIntervals`
    ///
    /// Для `PLAIN` вариантов такой выбор отсутствует.
    /// Значение по умолчанию: `PLAIN`
    pub option_type: CourierOptionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные категории курьерской доставки.
pub enum CourierCategory {
    Express,
    Today,
    Standard,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные типы опций курьерской доставки.
pub enum CourierOptionType {
    #[default]
    Plain,
    Flexible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные типы служб доставки.
pub enum DeliveryProvider {
    Boxberry,
    Cdek,
    RussianPost,
    Ems,
    Courier,
    Dhl,
    ExpressDelivery,
    Fivepost,
    OzonRocket,
    Dpd,
    SberLogistics,
    Pek,
    Pickpoint,
    Kce,
    PonyExpress,
    YandexDelivery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет опции самовывоза.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#pickupoption>
pub struct PickupOption {
    /// Адрес в виде строки.
    /// Максимальная длина: 2048
    pub address: String,

    /// Координаты местоположения.
    pub location: Location,

    /// Уникальный id точки самовывоза в системе продавца.
    /// Максимальная длина: 2048
    pub pickup_point_id: String,

    /// Название точки самовывоза.
    /// Максимальная длина: 2048
    pub title: String,

    /// Индивидуальные методы оплаты для выбранного способа самовывоза.
    #[serde(default = "Vec::new")]
    pub allowed_payment_methods: Vec<AllowedPaymentMethodType>,

    /// Стоимость доставки в точку.
    /// Пример: `123.45`
    #[serde(with = "option_string_as_float")]
    pub amount: Option<f64>,

    /// Дополнительное описание.
    /// Максимальная длина: 2048
    pub description: Option<String>,
    #[serde(with = "option_iso8601")]
    /// Ближайшая возможная дата доставки.

    /// Формат: `YYYY-MM-DD`
    pub from_date: Option<Time>,
    #[serde(default = "Vec::new")]
    /// Телефоны для связи.
    /// Максимальная длина: 2048
    pub phones: Vec<String>,

    /// Тип точки вывоза.
    pub provider: Option<PickupProvider>,

    /// Чек на доставку.
    pub receipt: Option<ItemReceipt>,
    #[serde(default = "Vec::new")]
    /// График работы точки.
    pub schedule: Vec<PickupSchedule>,

    /// Срок хранения товара в точке самовывоза в днях.
    pub storage_period: Option<i32>,

    #[serde(with = "option_iso8601")]
    /// Самая поздняя дата доставки.
    /// Формат: `YYYY-MM-DD`
    pub to_date: Option<Time>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные типы методов оплаты.
pub enum AllowedPaymentMethodType {
    Card,
    Split,
    CashOnDelivery,
    CardOnDelivery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Возможные типы точек самовывоза.
pub enum PickupProvider {
    YandexMarket,
    Boxberry,
    Cdek,
    InStore,
    RussianPost,
    Pickpoint,
    Dpd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет график работы точки самовывоза.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#pickupschedule>
pub struct PickupSchedule {
    /// День недели или диапазон дней.
    /// Пример: "пн-пт"
    /// Максимальная длина: 2048
    pub label: String,

    /// Время открытия.
    /// Формат: `HH:mm`
    ///
    /// Пример: "08:00"
    pub from_time: String,

    /// Время закрытия.
    /// Формат: `HH:mm`
    ///
    /// Пример: "20:00"
    pub to_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет опции доставки Яндекс.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#yandexdeliveryoption>
pub struct YandexDeliveryOption {
    /// Стоимость доставки.

    /// Пример: `123.45`
    #[serde(with = "string_as_float")]
    pub amount: f64,

    /// Категория доставки.
    /// Возможные значения: `EXPRESS`, `TODAY`, `STANDARD`
    pub category: CourierCategory,

    /// Название способа доставки. Показывается пользователю в списке вариантов.
    /// Максимальная длина: `2048`
    pub title: String,

    /// Id предложения Яндекс Доставки.
    /// Максимальная длина: `2048`
    pub yandex_delivery_option_id: String,

    /// Индивидуальные методы оплаты для метода доставки.
    /// Этот параметр нужно использовать,
    /// если нужно ограничить методы оплаты, указанные в availablePaymentMethods.
    /// Если параметр не указан, то используются все методы оплаты, перечисленные в availablePaymentMethods.
    #[serde(default = "Vec::new")]
    pub allowed_payment_methods: Vec<AllowedPaymentMethodType>,

    /// Дата и время начала доставки.
    #[serde(with = "option_iso8601")]
    pub from_datetime: Option<Time>,

    /// Чек на доставку.
    pub receipt: Option<ItemReceipt>,

    /// Дата и время окончания доставки.
    #[serde(with = "option_iso8601")]
    pub to_datetime: Option<Time>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет выбор даты и времени доставки.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#flexiblecustomerchoice>
pub struct FlexibleCustomerChoice {
    #[serde(with = "iso8601")]
    /// Дата доставки.
    pub date: Time,

    /// Интервал времени доставки.
    pub time: Option<TimeInterval>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет интервалы времени доставки.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#flexibletimeintervals>
pub struct FlexibleTimeIntervals {
    #[serde(rename = "type")]
    /// Тип интервалов.
    /// Если указан тип GRID, то необходимо задать поле grid. Если указан тип VALUES, то необходимо задать поле values
    pub time_intervals_type: TimeIntervalsType,

    /// Сетка интервалов.
    /// Кодирует интервалы в виде сетки. Используйте этот формат, если необходимо задать больше 20 интервалов доставки. Пример: {"start": "09:00", "end": "21:00", "duration": "00:20", "step": "01:00"} трактуется как набор интервалов: [{"start": "09:00", "end": "09:20"}, {"start": "10:00", "end": "10:20"}, ..., {"start": "20:00", "end": "20:20"}]
    pub grid: Option<FlexibleTimeIntervalsGridDescriptor>,
    #[serde(default)]
    /// Список интервалов.
    /// Задаёт список интервалов напрямую. Подходит для небольшого количества интервалов доставки. Рекомендуемое максимальная количество интервалов - 20
    pub values: Vec<TimeInterval>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет интервал времени.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#timeinterval>
pub struct TimeInterval {
    /// Время начала интервала.
    pub start: String,

    /// Время конца интервала.
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет сетку интервалов времени доставки.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#flexibletimeintervalsgriddescriptor>
pub struct FlexibleTimeIntervalsGridDescriptor {
    /// Продолжительность каждого интервала.
    pub duration: String,

    /// Максимальное время начала самого последнего интервала.
    pub end: String,

    /// Время начала самого первого интервала.
    pub start: String,

    /// Разница во времени между началами двух соседних интервалов.
    pub step: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Тип интервалов.
pub enum TimeIntervalsType {
    Grid,
    Values,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Представляет координаты местоположения.
/// <https://pay.yandex.ru/docs/ru/custom/backend/yandex-pay-api/order/merchant_v1_order-get#location>
pub struct Location {
    /// Широта.
    pub latitude: f64,

    /// Долгота.
    pub longitude: f64,
}
