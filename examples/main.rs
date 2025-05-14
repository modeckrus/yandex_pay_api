use yandex_pay_api::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt::init();
    // Инициализация клиента
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
                .title("Телефон")
                .total(29999.99)
                .build(),
        ])
        .total(CartTotal::new().amount(29999.99).build())
        .external_id("cart_001")
        .build();

    // Создание запроса
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
    println!("Payment URL: {}", response.payment_url);
    Ok(())
}
