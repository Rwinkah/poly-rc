pub mod public;

#[cfg(test)]
mod tests {
    use crate::public::orderbook::TokenId;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let _result = public::orderbook::get_orderbook_summary(TokenId {
            token_id: String::from("1"),
        })
        .await;
    }
}
