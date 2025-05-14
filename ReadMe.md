yandex_pay_api
===========================

![Crates.io Version](https://img.shields.io/crates/v/yandex_pay_api)
![docs.rs](https://img.shields.io/docsrs/yandex_pay_api)

`yandex_pay_api` — это библиотека на языке Rust для взаимодействия с API Yandex Pay. Она предоставляет удобный и типобезопасный способ интеграции Yandex Pay в ваши приложения на Rust.

## Возможности

- **Управление заказами**: Создание, отправка и получение заказов.
- **Работа с корзиной**: Определение и управление элементами корзины с детализированными атрибутами.
- **Методы оплаты**: Указание доступных методов оплаты и предпочтений.
- **Поддержка сериализации**: Встроенная поддержка JSON-сериализации/десериализации с использованием `serde`.
- **Паттерн Builder**: Упрощение создания объектов с помощью паттерна Builder.

## Установка
Добавьте следующее в ваш `Cargo.toml`:

```toml
[dependencies]
yandex_pay_api = "0.1.0"
```

## Использование

### Пример

```rust
use yandex_pay_api::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt::init();

    // Инициализация клиента API
    let client = reqwest::Client::new();
    let api = YandexPayApi::new(
        "https://sandbox.pay.yandex.ru".into(),
        std::env::var("YANDEX_MERCHANT_ID")?.into(),
        client,
    );

    // Создание корзины
    let cart = RenderedCart::new()
        .items(vec![
            RenderedCartItem::new()
                .product_id("prod_123")
                .quantity(ItemQuantity::new().count(1.0).available(5.0).build())
                .title("Смартфон")
                .total(29999.99)
                .build(),
        ])
        .total(CartTotal::new().amount(29999.99).build())
        .external_id("cart_001")
        .build();

    // Создание запроса на заказ
    let request = CreateOrderRequest::new()
        .cart(cart)
        .order_id("order_123")
        .currency_code(CurrencyCode::Rub)
        .available_payment_methods(vec![AvailablePaymentMethod::Card])
        .redirect_urls(Some(
            MerchantRedirectUrls::new()
                .on_error("https://example.com/error")
                .on_success("https://example.com/success")
                .on_abort(Some("https://example.com/abort".into()))
                .build(),
        ))
        .preferred_payment_method(Some(PreferredPaymentMethod::FullPayment))
        .ttl(Some(1800))
        .build();

    // Отправка запроса
    let response = api.create_order(request).await?;
    println!("Ссылка на оплату: {}", response.payment_url);

    Ok(())
}
```

## Документация

Для подробной документации посетите [API Reference](https://docs.rs/yandex_pay_api).

## Вклад

Мы приветствуем ваши вклады! Пожалуйста, создавайте issue или отправляйте pull request на [GitHub](https://github.com/modeckrus/yandex_pay_api).

## Лицензия

Этот проект лицензирован на условиях лицензии MIT. Подробнее см. в файле [LICENSE-APACHE](LICENSE-APACHE).

## Благодарности

Эта библиотека вдохновлена официальной [документацией API Yandex Pay](https://pay.yandex.ru/docs/).  