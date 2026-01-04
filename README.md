# poly-rc

A **production-grade Rust SDK** for interacting with the Polymarket API, designed for:
- Trading bots
- Long-running services
- Infrastructure and backend systems

## Features

- **Orderbook API** - Get orderbook summaries for tokens
- **Pricing API** - Get market prices, midpoint prices, and price history
- **Sports API** - Query sports teams and metadata
- **Spreads API** - Get bid-ask spreads
- Type-safe models with automatic serialization/deserialization
- Async/await support with tokio
- Comprehensive error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
poly-rc = "0.1.0"
```

Note: If you're using `#[tokio::main]` in your code, you'll also need to add tokio with the `macros` feature:

```toml
tokio = { version = "1.48.0", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use poly_rc::public::{
    PubClient, TokenId,
    orderbook::OrderBook,
    pricing::{Pricing, models::MarketPriceDTO},
};
use poly_rc::shared::Side;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PubClient::new();
    let orderbook = client
        .get_orderbook_summary(TokenId {
            token_id: "your_token_id".to_string(),
        })
        .await?;
    Ok(())
}
```

### Using a runtime manually

```rust
use poly_rc::public::{PubClient, TokenId, orderbook::OrderBook};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let client = PubClient::new();
        let orderbook = client
            .get_orderbook_summary(TokenId {
                token_id: "your_token_id".to_string(),
            })
            .await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    })
}
```

### Already in an async context

If you're already in an async function (e.g., in a web framework or another async runtime), you can use the library directly:

```rust
async fn my_handler() -> Result<(), Box<dyn std::error::Error>> {
    let client = PubClient::new();
    let orderbook = client
        .get_orderbook_summary(TokenId {
            token_id: "your_token_id".to_string(),
        })
        .await?;
    Ok(())
}
```

## API Modules

The SDK provides traits for different API modules:
- `OrderBook` - Orderbook operations
- `Pricing` - Market pricing operations
- `Sports` - Sports data operations
- `Spreads` - Bid-ask spread operations

See the documentation for detailed API reference.

## Error Handling

All API methods return `Result<T, ApiError>`. Handle errors appropriately:

```rust
match client.get_orderbook_summary(token_id).await {
    Ok(summary) => println!("Success: {:?}", summary),
    Err(ApiError::Http(error)) => eprintln!("HTTP error: {}", error.status),
    Err(ApiError::Decode(msg)) => eprintln!("Decode error: {}", msg),
    Err(ApiError::Unexpected(msg)) => eprintln!("Unexpected error: {}", msg),
}
```

