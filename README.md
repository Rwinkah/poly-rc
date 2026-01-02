# poly-rc

This project aims to provide a **production-grade execution SDK** for Polymarket, designed for:
- Trading bots
- Long-running services
- Infrastructure and backend systems



# Sample Usage

```rust

use poly_rc;


trait OrderBook {
    fn get_order_books() {

    }
}


struct PolyRC {
    key: String,
    ...
}


impl OrderBook for PolyRC {

}




fn main() {
    let client = PolyRC::new(...);
    let order_books = client.get_order_books();
    let pricing = client.get_pricing();
}


```