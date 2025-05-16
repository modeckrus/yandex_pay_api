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

## Features
- **reqwest** - use reqwest as http client `default`
- **rustls** - use rustls for reqwest client `default`
- **native-tls** - use native-tls for reqwest client

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

## Пример для своего http клиента
`Cargo.toml`
```toml
[dependencies]
yandex_pay_api = {version = "0.2.0", no-default-features = true}
reqwest = {version = "0.12"}
```
`main.rs`
```rust
use yandex_pay_api::*;

struct MyHttpClient(reqwest::Client);

impl HttpClient for MyHttpClient {
    fn send<T: serde::de::DeserializeOwned>(
        &self,
        request: YandexPayApiRequest,
    ) -> impl Future<Output = R<T>> {
        let client = self.0.clone();

        async move {
            let body = request.body.clone();
            let response = client
                .post(&*request.url)
                .header("Authorization", format!("Api-Key {}", request.api_key))
                .header("X-Request-Id", &*request.request_id)
                .header("X-Request-Timeout", request.request_timeout.to_string())
                .header("X-Request-Attempt", request.request_attempt.to_string())
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await?;

            if response.status().is_success() {
                let result = response.text().await?;
                let result = serde_json::from_str::<YandexPayApiResponse<T>>(&result)?;
                Ok(result.data)
            } else {
                let error_message = response.text().await?;
                tracing::error!("{}", error_message);
                let error = serde_json::from_str::<YandexPayApiResponseError>(&error_message)?;
                Err(YandexPayApiError::Api(error))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt::init();
    // Инициализация клиента
    let client = MyHttpClient(reqwest::Client::new());
    let api = YandexPayApi::new(
        "https://sandbox.pay.yandex.ru".into(),
        // Ваш API-ключ для sandbox он совпадает с MERCHANT_ID
        std::env::var("YANDEX_MERCHANT_ID")?.into(),
        client,
    );
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